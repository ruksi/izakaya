create or replace function validate_user_username(
    user_username text,
    self record default null
) returns bool
as
$$
declare
    is_not_unique bool;
begin
    user_username := trim(user_username);

    if self is null then
        is_not_unique :=
                exists(select
                       from "user" u
                       where u.username = user_username);
    else
        is_not_unique :=
                exists(select
                       from "user" u
                       where u.username = user_username
                         and u.user_id != self.user_id);
    end if;
    if is_not_unique then
        call wrong_named_value(
                'Username is already in use',
                'Username',
                user_username,
                'Choose more unique username'
             );
    end if;

    if char_length(user_username) < 3 then
        call wrong_named_value(
                'Username is too short',
                'Username',
                user_username,
                'Choose a username with at least 3 characters'
             );
    end if;

    if (user_username collate "ucs_basic") !~ '^[a-zA-Z0-9][a-zA-Z0-9\-]+[a-zA-Z0-9]$' then
        call wrong_named_value(
                'Username is invalid',
                'Username',
                user_username,
                'Choose a username with only letters and numbers; you may use dashes (-) to separate words'
             );
    end if;

    return true;
end ;
$$ language plpgsql;



create table "user"
(
    primary key (user_id),
    user_id    uuid                                   default uuid_generate_v4(),
    username   text collate case_insensitive not null,
    created_at timestamptz                   not null default now(),
    updated_at timestamptz                   not null default now(),
    constraint valid_username check (validate_user_username(username, "user"))
);
call install_updated_at_trigger('"user"');



create or replace function clean_user()
    returns trigger
as
$$
begin
    new.username := trim(new.username);
    return new;
end;
$$ language plpgsql;

create or replace trigger clean_user_trigger
    before insert or update
    on "user"
    for each row
execute procedure clean_user();
