create table "user"
(
    primary key (user_id),
    user_id       uuid                                   default uuid_generate_v4(),
    username      text collate case_insensitive not null,
    password_hash text                          not null,
    created_at    timestamptz                   not null default now(),
    updated_at    timestamptz                   not null default now(),
    unique (username)
);
call install_updated_at_trigger('"user"');
