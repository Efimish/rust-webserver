create table "user_read_message"
(
    "user_id" uuid not null references "user" ("id") on delete cascade,
    "message_id" uuid not null references "message" ("id") on delete cascade,
    "created_at" timestamptz not null default now(),
    primary key ("user_id", "message_id")
);