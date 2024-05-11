Here we have some scripts\
note that most of them should be run from root directory
since they read `.env` file.\

- run `download-regexes` to download or update `regexes.yaml`
that is needed for user agent parser.\

- run `certificates-test` and then `certificates-get`
to test if you can and then get SSL certificates
using `Certbot`.\

- run `certificates-renew` when your certificates expire
(that happens every 3 months)

- run `copy-env` to copy the content of `.env.example` to `.env`