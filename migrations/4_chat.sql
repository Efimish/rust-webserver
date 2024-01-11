create type chat_type as enum ('saved', 'private', 'group');

create table chat
(
    chat_id uuid primary key default uuid_generate_v4(),
    chat_type chat_type
);