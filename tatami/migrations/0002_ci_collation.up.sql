-- Use for unique, case-insensitive fields like usernames and emails.
-- This is the more moder version if `citext` module and data type.
-- https://www.postgresql.org/docs/current/collation.html#COLLATION-NONDETERMINISTIC
create collation if not exists case_insensitive (
    -- use external ICU library for this locale
    provider = icu,
    -- ks-level2 = case insensitive
    locale = 'und-u-ks-level2',
    -- nondeterministic = may equal even if have different bytes
    deterministic = false
    );
