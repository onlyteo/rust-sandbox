-- Known users. The payload is filled in by the first sync run.
INSERT INTO users (id)
SELECT gen_random_uuid()
FROM generate_series(1, 50);
