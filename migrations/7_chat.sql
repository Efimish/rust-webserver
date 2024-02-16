create type chat_type as enum ('saved', 'private', 'group');

create table chat
(
    id uuid primary key default uuid_generate_v4(),
    type chat_type not null,
    name text,
    description text,
    image uuid references upload (id) on delete set null,
    created_at timestamptz not null default now()
);

create or replace function check_chat_details()
returns trigger as
$$
begin
    if NEW.type != 'group' then
        NEW.name := NULL;
        NEW.image := NULL;
        NEW.description := NULL;
    end if;
    return NEW;
end;
$$ language plpgsql;

create trigger check_chat_details_trigger
before insert or update on chat
for each row
execute procedure check_chat_details();