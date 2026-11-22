# Rust Axum Batch Job

Keeps a local Postgres cache of users in sync with an external REST API. Users change upstream at
random, so every user is re-checked periodically. Any number of instances can run against the same
database; a row is never processed by two instances at the same time.

## Architecture

```
┌──────────────────────── backend (one instance) ───────────────────────┐
│                                                                       │
│  SyncJob (tokio interval)                                             │
│    1. claim_batch  ──── short UPDATE … FOR UPDATE SKIP LOCKED ───┐    │
│    2. GET /users/{id} (bounded concurrency, no tx open)          │    │
│    3. complete / fail ── fenced on claimed_by ───────────────────┤    │
│                                                                  ▼    │
│  Axum :8081  GET /health                                    Postgres  │
│  Mock external API :8082  GET /users/{id}                   (users)   │
└───────────────────────────────────────────────────────────────────────┘
```

| Module                       | Responsibility                                                   |
|------------------------------|------------------------------------------------------------------|
| `src/job/sync_job.rs`        | Scheduler loop, per-tick batch processing, graceful shutdown     |
| `src/repository/user.rs`     | The claim / complete / fail SQL protocol                         |
| `src/client/external_api.rs` | `reqwest` client for the external API                            |
| `src/mock/external_api.rs`   | In-process stand-in for the external system (random mutations and failures) |
| `migrations/`                | `users` table with payload + sync metadata, 50 seeded user ids   |

### Runtime and error handling

The job is a spawned tokio task joined into the same `tokio::select!` as the web servers and the
shutdown signal (`src/main.rs`).

- A failing tick (e.g. database unavailable) is logged; the loop keeps ticking.
- A failing fetch (timeout, non-2xx) releases the row with exponential backoff
  (`min(2^failure_count, max_backoff_secs)`), recording `failure_count` and `last_error`.
- A panicking per-user task is caught by the `JoinSet`; the row's lease expires and it is retried.
- On SIGINT/SIGTERM the job stops claiming, finishes the in-flight batch, then the process exits.
- `MissedTickBehavior::Skip` prevents a burst of ticks after a slow batch.

### Sync metadata

| Column             | Purpose                                                        |
|--------------------|----------------------------------------------------------------|
| `status`           | `IDLE` / `PROCESSING`                                          |
| `claimed_by`       | Id of the instance holding the claim (random per process)      |
| `lease_expires_at` | After this point the claim is void and the row can be reclaimed |
| `next_sync_at`     | When the row is due (indexed)                                  |
| `data_updated_at`  | Last time the payload *actually* changed                       |
| `last_synced_at`, `failure_count`, `last_error` | Observability and backoff         |

## Atomicity: how rows are distributed across instances

### The protocol

1. **Claim**: one short autocommit statement flips due rows to `PROCESSING`:
   ```sql
   UPDATE users SET status='PROCESSING', claimed_by=$me, lease_expires_at=now()+$lease
   WHERE id IN (SELECT id FROM users
                WHERE next_sync_at <= now() AND (status='IDLE' OR lease_expires_at < now())
                ORDER BY next_sync_at LIMIT $n
                FOR UPDATE SKIP LOCKED)
   RETURNING id
   ```
   Row locks exist only for the milliseconds this statement runs.
2. **Process**: call the external API with **no transaction open**.
3. **Write back**: `UPDATE … WHERE id=$id AND claimed_by=$me`. Zero rows affected means our lease
   expired and another instance took over, so the stale result is discarded.

### Why not hold a row lock for the whole processing time?

`SELECT … FOR UPDATE` in a transaction that spans the HTTP calls is correct and releases
automatically on crash. But it pins a pool connection and keeps a transaction open for as long as
the slowest external call. That leads to pool exhaustion, long-running transactions holding back
vacuum, and no visibility into what is being processed.

### Status flags instead of locks?

A status flag on its own is **not enough**:

- **Crash recovery**: if an instance dies after setting `PROCESSING`, nothing ever resets the row.
  A lease timestamp (`lease_expires_at`) is needed so others can reclaim it.
- **Fencing**: an instance that stalls past its lease (GC pause, slow request) must not overwrite
  the result of the instance that reclaimed the row. An owner id (`claimed_by`) checked on
  write-back is needed.

These columns are needed whichever way the flag is flipped. The remaining question is only how to
flip it atomically.

### Explicit lock (`SKIP LOCKED`) vs. optimistic compare-and-set

In Postgres every `UPDATE` takes a row lock, so "lock-free" really means *implicit lock + re-check*.
The optimistic variant drops `FOR UPDATE SKIP LOCKED` and repeats the guard in the outer `WHERE`:

```sql
UPDATE users SET status='PROCESSING', claimed_by=$me, ...
WHERE id IN (SELECT id FROM users WHERE <due> ORDER BY next_sync_at LIMIT $n)
  AND (status='IDLE' OR lease_expires_at < now())   -- required!
```

|                     | `SKIP LOCKED` (chosen)                              | Optimistic CAS                                                         |
|---------------------|-----------------------------------------------------|------------------------------------------------------------------------|
| Correctness         | ✅                                                  | ✅ only if the guard is repeated in the **outer** `WHERE`. Under READ COMMITTED, Postgres re-checks only the outer predicate after waiting on a lock, not the subquery. If you forget it, rows get double-claimed. |
| Concurrent claimers | Each skips the others' locked rows and gets the **next** N due rows | All pick the **same** top N, block on the first claimer, re-check and get **0 rows**. Instances end up taking turns instead of running in parallel. |
| Workarounds needed  | None                                                | Random ordering / per-instance offsets, which sacrifice index order and fairness |
| Portability         | Postgres 9.5+, MySQL 8+, Oracle                     | Any SQL database                                                       |

Both are equally correct and both still need the lease and owner columns. `SKIP LOCKED` costs nothing
extra (the lock lives only for the claim statement) and is clearly better when several instances
claim concurrently. Optimistic CAS is only preferable if the database lacks `SKIP LOCKED`.

### Alternative: advisory-lock leader election

`pg_try_advisory_lock(<job id>)` lets exactly one instance run the whole batch. No per-row metadata
is needed for exclusivity, but there is no parallelism across instances, and the lock is tied to a
session/connection that has to be kept alive (a crashed leader frees it on disconnect). It is a good
fit for small jobs; per-row claiming scales better.

### Idempotency

Write-back always sets the same payload for the same upstream state. `data_updated_at` is bumped
only when `(name, email) IS DISTINCT FROM` the cached values, so re-syncing an unchanged user only
touches sync metadata (`last_synced_at`, `next_sync_at`).

## Configuration

YAML files are compiled in from `backend/config/local/` (default) or `backend/config/docker/`
(feature `docker`). Most relevant: `sync_job_config.yaml` (`tick_interval_secs`, `batch_size`,
`lease_secs`, `sync_interval_secs`, `max_concurrency`, `max_backoff_secs`) and
`mock_external_api_config.yaml` (`mutation_interval_secs`, `mutation_percent`, `failure_percent`).

`lease_secs` must comfortably exceed the time to process one batch
(`batch_size / max_concurrency × request timeout`), otherwise rows are reclaimed while still being
processed. That is still safe thanks to fencing, but it wastes work.

## Running

```bash
just postgres-up
cargo run -p rust-axum-batch-job-backend
```

Inspect progress:

```sql
SELECT status, count(*), count(name) AS synced, sum(failure_count) AS failures FROM users GROUP BY 1;
```

## Tests

```bash
cargo test -p rust-axum-batch-job-backend   # requires Docker (testcontainers)
```

- `tests/claim.rs`: concurrent claims are disjoint and each gets a full batch; expired leases are
  reclaimed; a stale owner cannot write back.
- `tests/sync_job.rs`: idempotent re-sync; failure backoff and recovery; several job instances split
  the work exactly once; graceful shutdown.
