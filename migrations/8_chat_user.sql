create table chat_user
(
    user_id uuid not null references "user" (id) on delete cascade,
    chat_id uuid not null references chat (id) on delete cascade,
    primary key (user_id, chat_id)
);