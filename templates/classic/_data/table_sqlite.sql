CREATE TABLE IF NOT EXISTS users
(
id TEXT PRIMARY KEY NOT NULL,
username VARCHAR(255) NOT NULL UNIQUE,
password VARCHAR(511) NOT NULL
);
BEGIN;
INSERT OR IGNORE INTO "users" ("id", "username", "password") VALUES ('cdd0e080-5bb1-4442-b6f7-2ba60dbd0555', 'zhangsan', '$argon2id$v=19$m=19456,t=2,p=1$rcosL5pOPdA2c7i4ZuLA4Q$s0JGh78UzMmu1qZMpVUA3b8kWYLXcZhw7uBfwhYDJ4A');
COMMIT;
