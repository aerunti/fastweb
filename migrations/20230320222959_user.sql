-- Add migration script here
create table users (
    id serial8 primary key,
    name varchar(255) not null default 'user',
    email varchar(20) UNIQUE not null,
    passw varchar(255) not null, -- 'passwd hash'
    create_dt timestamp not null default current_timestamp, -- 'create datetime'
    update_dt timestamp not null default current_timestamp, -- 'udpate datetime'
    status varchar(10) not null default 'normal', -- comment 'status: normal, blocked, deleted',
    permissions text not null default '|user|' -- 'permissons like: |user|, |user|admin|, |user|admin|'
);