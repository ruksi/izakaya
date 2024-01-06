-- A trigger to keep track of the last time a row was updated.
--
-- Usage: call install_updated_at_trigger('<table name>');

create or replace function set_updated_at()
    returns trigger
as
$$
begin
    new.updated_at := now();
    return new;
end;
$$ language plpgsql;

create or replace procedure install_updated_at_trigger("table_name" regclass)
as
$$
begin
    execute format('create or replace trigger set_updated_at_trigger
        before update
        on %s
        for each row
        when (old is distinct from new)
    execute function set_updated_at();', table_name);
end;
$$ language plpgsql;
