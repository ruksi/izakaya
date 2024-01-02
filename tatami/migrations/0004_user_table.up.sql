create table "user"
(
    id         uuid primary key                                default uuid_generate_v4(),
    username   text collate "case_insensitive" unique not null,
    email      text collate "case_insensitive" unique not null,
    bio        text                                   not null default '',
    created_at timestamptz                            not null default now(),
    updated_at timestamptz
);

-- Create a search indexes to allow pattern-matching using the standard "ucs_basic" collation.
-- Usage: select * from "user" where (username collate "ucs_basic") ilike ($1 || '%')
create index on "user" (username collate "ucs_basic");
create index on "user" (email collate "ucs_basic");

select trigger_updated_at('"user"');
