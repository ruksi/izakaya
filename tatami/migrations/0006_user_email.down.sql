alter table "user"
    drop column if exists primary_email_id;
drop table if exists user_email;
drop function if exists setup_user_primary_email();
drop function if exists clean_user_email;
drop function if exists validate_email_address(text, record);
