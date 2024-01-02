-- Use for unique, case-insensitive fields like usernames and emails.
-- https://www.postgresql.org/docs/current/collation.html#COLLATION-NONDETERMINISTIC
create collation case_insensitive (
    -- use external ICU library for this locale
    provider = icu,
    -- ks-level2 = case insensitive
    locale = 'und-u-ks-level2',
    -- nondeterministic = may equal even if have different bytes
    deterministic = false
    );
