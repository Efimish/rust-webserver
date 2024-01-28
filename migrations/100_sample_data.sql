insert into upload (
    upload_id, file_name, extension, content_type, folder, size
) values (
    'fd25328f-1891-49ad-ad65-e303c76d14a2',
    'avatar.webp',
    '.webp',
    'image/webp',
    'default',
    1012
);

insert into "user" (
    username, email, password_hash, display_name, status, created_at, updated_at
) values (
    'user1', 'user1@efima.fun', '$argon2id$v=19$m=32768,t=2,p=1$QYdcuS8hJp7g/Eqzv6ChHw$Au1bzdDhRAcKsbs2V+0iov/84NvqGgO1v//U72DzoJs',
    'First user', 'Just chilling', '2023-12-10 12:00:00.000000+00', '2023-12-10 12:00:00.000000+00'
), (
    'user2', 'user2@efima.fun', '$argon2id$v=19$m=32768,t=2,p=1$Y1OTp3yEB+STQRXqdkvwlg$tlmWW8cFfF5tcGi0zaRhmi0hVKy0wH7pojQP4EVCjUI',
    'Second user', 'Working hard', '2023-12-10 14:00:00.000000+00', '2023-12-10 14:00:00.000000+00'
), (
    'user3', 'user3@efima.fun', '$argon2id$v=19$m=32768,t=2,p=1$lgD/4rtnQBM0MptE999cew$JsMG1N/LIRSkRp44EYqFlbcM1faFZBgI7ubByCJ0Dgo',
    'Third user', 'Listening to spotify', '2023-12-10 16:00:00.000000+00', '2023-12-10 16:00:00.000000+00'
), (
    'user4', 'user4@efima.fun', '$argon2id$v=19$m=32768,t=2,p=1$berv2wPDGBAUZhWo1vZhFA$UV3nXLCL3tvEjIQk3u5n8EYOrAnekpPIItmTUP3CMw8',
    'Fourth user', 'Hey there!', '2023-12-10 18:00:00.000000+00', '2023-12-10 18:00:00.000000+00'
), (
    'user5', 'user5@efima.fun', '$argon2id$v=19$m=32768,t=2,p=1$lkD+5q2VLrWb67VUEGuRkg$sKJXIGrMZWI/+cfih2El2NyAWPnJaC5ffDCZC1D7fS8',
    'Fifth user', 'Welcome to my profile!', '2023-12-10 20:00:00.000000+00', '2023-12-10 20:00:00.000000+00'
);

update "user"
set avatar = 'fd25328f-1891-49ad-ad65-e303c76d14a2'
where username = 'user1';

insert into user_session (
    user_id, session_id, user_ip, user_agent, user_country, user_city
) values (
    (select user_id from "user" where username = 'user1'),
    '1a848ee3-b26b-4744-b58a-fd95fe25ed2a',
    '1.1.1.1',
    'Windows 10',
    'Australia',
    'South Brisbane'
);

insert into chat (
    chat_type, chat_name, chat_description, chat_image
) VALUES (
    'group', 'Chat 1', 'Biggest chat on this platform', 'fd25328f-1891-49ad-ad65-e303c76d14a2'
), (
    'group', 'Chat 2', 'Second biggest chat on this platform', 'fd25328f-1891-49ad-ad65-e303c76d14a2'
);

insert into chat (
    chat_type
) VALUES (
    'private'
), (
    'saved'
);

insert into chat_user (
    user_id, chat_id
)
select user_id, chat_id
from "user", chat
where "user".username = 'user1'
and chat.chat_name = 'Chat 1'
    union
select user_id, chat_id
from "user", chat
where "user".username = 'user2'
and chat.chat_name = 'Chat 1'
    union
select user_id, chat_id
from "user", chat
where "user".username = 'user3'
and chat.chat_name = 'Chat 1'
    union
select user_id, chat_id
from "user", chat
where "user".username = 'user4'
and chat.chat_name = 'Chat 1'
    union
select user_id, chat_id
from "user", chat
where "user".username = 'user5'
and chat.chat_name = 'Chat 1'
    union
select user_id, chat_id
from "user", chat
where "user".username = 'user1'
and chat.chat_name = 'Chat 2'
    union
select user_id, chat_id
from "user", chat
where "user".username = 'user2'
and chat.chat_name = 'Chat 2'
    union
select user_id, chat_id
from "user", chat
where "user".username = 'user3'
and chat.chat_name = 'Chat 2'
    union
select user_id, chat_id
from "user", chat
where "user".username = 'user1'
and chat.chat_name = 'Chat 3';