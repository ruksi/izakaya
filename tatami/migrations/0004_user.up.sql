create table "user"
(
    primary key (user_id),
    user_id       uuid                                   default uuid_generate_v4(),
    username      text collate case_insensitive not null,
    password_hash text                          not null,
    is_superuser  boolean                       not null default false,
    created_at    timestamptz                   not null default now(),
    updated_at    timestamptz                   not null default now(),
    unique (username)
);
call install_updated_at_trigger('"user"');

-- Add a separate index for pattern matching queries.
-- Usage: select * from "user" where (username collate "ucs_basic") ilike ($1 || '%')
create index on "user" (username collate "ucs_basic");
