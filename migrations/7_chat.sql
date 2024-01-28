create type chat_type as enum ('saved', 'private', 'group');

create table chat
(
    chat_id uuid primary key default uuid_generate_v4(),
    chat_type chat_type not null,
    chat_name text,
    chat_description text,
    chat_image uuid references upload (upload_id) on delete set null,
    created_at timestamptz not null default now()
);

create or replace function check_chat_details()
returns trigger as
$$
begin
    if NEW.chat_type != 'group' then
        NEW.chat_name := NULL;
        NEW.chat_image := NULL;
        NEW.chat_description := NULL;
    end if;
    return NEW;
end;
$$ language plpgsql;

create trigger check_chat_details_trigger
before insert or update on chat
for each row
execute procedure check_chat_details();