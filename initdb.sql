create extension if not exists "uuid-ossp";

-- Ensure the database and schema names here match the databaes and schema
-- name in the `.env` file.
create database lnk_db;
create schema lnk_db;

\c lnk_db;

create table users(
    id serial primary key,
    username varchar(255) unique not null,
    email varchar(255) unique not null
);

create table password(
    id serial primary key,
    salt varchar(255) not null,
    digest varchar(255) not null,
    created timestamp with time zone default now() not null,

    user_id int references users(id) on delete cascade unique not null
);

create table test(
    id serial primary key,
    name varchar(255) not null
);
