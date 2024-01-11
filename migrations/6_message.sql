create table message
(
    message_id uuid primary key default uuid_generate_v4(),
    chat_id uuid not null references chat (chat_id) on delete cascade,
    sender_id uuid not null references "user" (user_id) on delete cascade,
    context text not null,
    created_at timestamptz not null default now()
);