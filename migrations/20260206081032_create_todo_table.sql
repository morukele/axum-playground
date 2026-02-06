-- Add migration script here

-- create enum for status
CREATE TYPE status as ENUM('Completed', 'InProgress', 'NotStarted', 'Deleted');

-- setting timezone for project
SET timezone = 'Europe/Paris';

-- create table
CREATE TABLE todos
(
    id          uuid PRIMARY KEY,
    name        VARCHAR NOT NULL,
    description VARCHAR, -- could be null
    status      STATUS NOT NULL, -- custom status type
    created_at TIMESTAMPTZ NOT NULL ,
    updated_at TIMESTAMPTZ NOT NULL
);
