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

insert into chat (
    chat_name
) values ('Chat 1'), ('Chat 2'), ('Chat 3');

-- insert into chat_user (
--     user_id, chat_id
-- ) values (
--     select user_id from "user" where username='user1',
--     select chat_id from chat where chat_name='Chat 1'
-- ), (
--     select user_id from "user" where username='user2',
--     select chat_id from chat where chat_name='Chat 1'
-- ), (
--     select user_id from "user" where username='user3',
--     select chat_id from chat where chat_name='Chat 1'
-- );