create table chat
(
    chat_id uuid primary key default uuid_generate_v4(),
    chat_name text not null
);