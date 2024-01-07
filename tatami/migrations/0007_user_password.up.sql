-- if null, user can't login with a password
alter table "user"
    add if not exists password_hash text default null;
