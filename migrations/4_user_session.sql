create table "user_session"
(
    "id" uuid primary key,
    "user_id" uuid not null references "user" ("id") on delete cascade,
    "user_ip" text not null,
    "user_agent" text not null,
    "user_country" text not null,
    "user_city" text not null,
    "last_active" timestamptz not null default now()
);