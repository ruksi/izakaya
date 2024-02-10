alter table "user"
    drop column if exists primary_email_id;
drop table if exists user_email;
