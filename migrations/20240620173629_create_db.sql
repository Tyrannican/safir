-- Add migration script here
create table if not exists safir (
    key text not null primary key,
    value text not null,
    environment text not null
);

create index if not exists idx_key on safir(key);
