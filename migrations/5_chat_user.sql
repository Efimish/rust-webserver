create table chat_user
(
    user_id uuid not null references "user" (user_id) on delete cascade,
    chat_id uuid not null references chat (chat_id) on delete cascade,
    primary key (user_id, chat_id)
);