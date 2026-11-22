# Plan for batch job example

## Requirements

Need to keep user data synced from an external system to local database. The user data in the external system are modified at random. Therefore it is necessary to check for all users if the data has been modified.

## Architecture

Need to implement a scheduling machanism to fetch the most recent state of cached user data. The data is cached in a Postgres database. The most recent data must be fetched from an external REST API. The data does not change very often, so the handling must be idempotent.

### Runtime

The backend application uses the Axum framework to set up a server. The scheduling mechanism must fit in with the runtime. It must be robust enough to be able to handle runtime errors in a gracefull way.

### Atomicity

The solution must handle there being several instances of the same app running in a cluster. There must be a mechanism to make sure the same rows are not processed by more than one instance at a time. Analyze if row locking is the most optimal way to solve this. Are there other solutions using status flags and other meta data so row locks are not necessary.
