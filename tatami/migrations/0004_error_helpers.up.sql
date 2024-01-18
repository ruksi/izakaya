-- Helper functions to raise errors from PL/pgSQL functions.
--
-- Error message will be human_readable_underscore_delimited_key
-- that we can use to generate error messages in our applications.
--
-- We use custom SQLSTATE error code 'WRONG' that we can catch in our applications
-- to learn that the error detail might include further JSON encoded information
-- to generate better error messages.

create or replace procedure wrong(
    message text,
    detail json default null,
    hint text default null
) as
$$
begin
    if hint is null and detail is null then
        raise sqlstate 'WRONG' using message = message;
    elsif detail is null then
        raise sqlstate 'WRONG' using message = message, hint = hint;
    elsif hint is null then
        raise sqlstate 'WRONG' using message = message, detail = detail;
    else
        raise sqlstate 'WRONG' using message = message, hint = hint, detail = detail;
    end if;
end;
$$ language plpgsql;



-- Usage:
--   call wrong_value('Username is too short', 'bob'::text, 'Must be at least 3 characters long');
--   -> [WRONG] ERROR: Username is too short
--      Detail: {"value" : "bob"}
--      Hint: Must be at least 3 characters long
create or replace procedure wrong_value(
    message text,
    value anyelement,
    hint text default null
) as
$$
begin
    call wrong(message, json_build_object('value', value), hint);
end;
$$ language plpgsql;



-- Usage:
--   call wrong_named_value('Username is invalid', 'Username', 'bob'::text, 'Must not be "bob"');
--   =>  [WRONG] ERROR: Username is invalid
--       Detail: {"name" : "Username", "value" : "bob"}
--       Hint: Must not be "bob"
create or replace procedure wrong_named_value(
    message text,
    name text,
    value anyelement,
    hint text default null
) as
$$
begin
    call wrong(message, json_build_object('name', name, 'value', value), hint);
end;
$$ language plpgsql;
