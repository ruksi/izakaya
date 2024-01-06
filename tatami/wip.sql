-- create table password_authentication
-- (
--     primary key (password_id),
--     password_id uuid                 default uuid_generate_v4(),
--     hash        text        not null,
--     created_at  timestamptz not null default now(),
--     updated_at  timestamptz not null default now()
-- );
-- call install_updated_at_trigger('password_authentication');
-- create table user_authentication
-- (
--     primary key (user_id, password_id),
--     foreign key (user_id) references "user" (user_id) on delete cascade,
--     foreign key (password_id) references password_authentication (password_id) on delete cascade,
--     user_id     uuid        not null,
--     password_id uuid,
--     created_at  timestamptz not null default now(),
--     updated_at  timestamptz not null default now()
-- );
-- call install_updated_at_trigger('user_authentication');
