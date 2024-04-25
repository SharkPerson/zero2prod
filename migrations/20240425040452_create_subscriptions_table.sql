-- Add migration script here
CREATE TABLE subscriptions(
    id UUID NOT NULL,
    PRIMARY KEY (id),
    email TEXT,
    name TEXT NOT NULL UNIQUE,
    subscribed_at timestamptz NOT NULL
);
