-- Add migration script here
CREATE TABLE IF NOT EXISTS clips
(
    clip_id   TEXT PRIMARY KEY NOT NULL,
    shortcode TEXT UNIQUE NOT NULL,
    content   TEXT NOT NULL,
    title     TEXT,
    posted    TIMESTAMP NOT NULL,
    expires   TIMESTAMP,
    password  TEXT,
    hits      BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS api_keys
(
    api_key BYTEA PRIMARY KEY
);