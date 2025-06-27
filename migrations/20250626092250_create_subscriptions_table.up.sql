-- Add up migration script here
DROP TABLE IF EXISTS subscriptions;

CREATE TABLE subscriptions(
    id serial PRIMARY KEY,
    uuid uuid not null,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL,
    created_at timestamptz null,
    updated_at timestamptz null,
    deleted_at timestamptz null
);

CREATE
OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at
= CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER update_subscription_timestamp
    BEFORE UPDATE
    ON subscriptions
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
