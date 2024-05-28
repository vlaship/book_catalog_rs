create table if not exists book_catalog_rs.users
(
    id bigint not null constraint users_pk primary key,
    login    text   not null,
    password text   not null
);

create unique index if not exists users_login_uindex on book_catalog_rs.users (login);
