create table user_email
(
    primary key (email_id, user_id),
    foreign key (user_id) references "user" (user_id) on delete cascade,
    email_id   uuid                                   default uuid_generate_v4(),
    user_id    uuid                          not null,
    email      text collate case_insensitive not null,
    created_at timestamptz                   not null default now(),
    updated_at timestamptz                   not null default now(),
    unique (email)
);
call install_updated_at_trigger('user_email');

-- Add a separate index for pattern matching queries.
-- Usage: select * from user_email where (email collate "ucs_basic") ilike ($1 || '%')
create index on user_email (email collate "ucs_basic");

alter table "user"
    add column primary_email_id uuid default null,
    add constraint fk_user_primary_email
        foreign key (user_id, primary_email_id)
            references user_email (user_id, email_id)
            on delete restrict;
