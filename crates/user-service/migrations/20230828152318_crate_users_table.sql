-- Add migration script here
CREATE TABLE users
(
    id         UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    username   TEXT UNIQUE      NOT NULL,
    password   TEXT             NOT NULL,
    created_at TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    locked     BOOLEAN          NOT NULL DEFAULT false
);

CREATE OR REPLACE FUNCTION set_updated_at_timestamp() RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_timestamp();