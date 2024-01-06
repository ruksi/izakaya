-- Helper functions to raise errors from PL/pgSQL functions.
--
-- Error message will be human_readable_underscore_delimited_key
-- that we can use to generate error messages in our applications.
--
-- We use custom SQLSTATE error code 'WRONG' that we can catch in our applications
-- to learn that the error detail might include further JSON encoded information
-- to generate better error messages.

create or replace procedure wrong(message text, detail json default null) as
$$
begin
    if detail is null then
        raise sqlstate 'WRONG' using message = message;
    else
        raise sqlstate 'WRONG' using message = message, detail = detail;
    end if;
end;
$$ language plpgsql;



create or replace procedure wrong_value(message text, value anyelement) as
$$
begin
    call wrong(message, json_build_object('value', value));
end;
$$ language plpgsql;
