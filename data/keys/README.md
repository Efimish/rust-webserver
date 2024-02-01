In this directory we keep our private and public rsa keys\
We need them to encode and decode JWTs\
You can move keys to different directory, check
`src/http/extractors/auth_user.rs` function `read_or_generate`