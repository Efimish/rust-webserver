Here we have some scripts\
they can help you to do things easier\
some of them read `.env` file, don't forger to set it up first\

- run `copy-env` to copy the content of `.env.example` to `.env`

- run `download-regexes` to download or update `regexes.yaml`
that is needed for user agent parser.

- run `nginx-start` to create nginx configuration file
using template and run nginx with it

- run `nginx-stop` to stop nginx

- run `certificates-test` and then `certificates-get`
to test if you can and then get SSL certificates
using `Certbot`. You will need nginx running first.

- run `certificates-renew` when your certificates expire
(that happens every 3 months)

read scripts to understand them better, they have comments inside