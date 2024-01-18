create or replace function validate_email_address(
    email_address text,
    self record default null
) returns bool as
$$
declare
    is_not_unique bool;
begin
    email_address := trim(email_address);

    if self is null then
        is_not_unique :=
                exists(select
                       from user_email e
                       where e.address = email_address);
    else
        is_not_unique :=
                exists(select
                       from user_email e
                       where e.address = validate_email_address.email_address
                         and e.email_id != self.email_id);
    end if;
    if is_not_unique then
        call wrong_named_value(
                'Email is already in use',
                'Email',
                email_address,
                'Try to login with your email or reset your password'
             );
    end if;

    if (email_address collate "ucs_basic") !~ '^[^@]+@[^@]+$' then
        call wrong_named_value(
                'Email is missing an @ sign',
                'Email',
                email_address,
                'Check that your email address is correct'
             );
    end if;

    return true;
end ;
$$ language plpgsql;



create table user_email
(
    primary key (email_id, user_id),
    foreign key (user_id) references "user" (user_id) on delete cascade,
    email_id   uuid                                   default uuid_generate_v4(),
    user_id    uuid                          not null,
    address    text collate case_insensitive not null,
    created_at timestamptz                   not null default now(),
    updated_at timestamptz                   not null default now(),
    constraint valid_address check (validate_email_address(address, user_email))
);
call install_updated_at_trigger('user_email');



create or replace function clean_user_email()
    returns trigger
as
$$
begin
    new.address := trim(new.address);
    return new;
end;
$$ language plpgsql;

create or replace trigger clean_user_email_trigger
    before insert or update
    on user_email
    for each row
execute procedure clean_user_email();



alter table "user"
    add column primary_email_id uuid default null,
    add constraint fk_user_primary_email
        foreign key (user_id, primary_email_id)
            references user_email (user_id, email_id)
            on delete restrict;


create or replace function setup_user_primary_email()
    returns trigger
as
$$
begin
    if exists(select
              from "user" u
              where u.user_id = new.user_id
                and u.primary_email_id is null)
    then
        update "user" u
        set primary_email_id = new.email_id
        where u.user_id = new.user_id;
    end if;
    return new;
end;
$$ language plpgsql;

create or replace trigger setup_user_primary_email_trigger
    after insert
    on user_email
    for each row
execute procedure setup_user_primary_email();
