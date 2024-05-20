# Scripts

Here we have some scripts. They can help you to do things easier.\
Some of them read `.env` file, don't forger to set it up first

- `copy-env` copies the content of `.env.example` to `.env`

- `download-regexes` downloads or updates `regexes.yaml`
that is needed for user agent parser

- `nginx-start` creates nginx configuration file
using template and runs nginx with it

- `nginx-stop` stops nginx

- `certificates-test` and `certificates-get`
tests if you can and gets SSL certificates
using `Certbot`. They require running nginx

- `certificates-renew` refreshes your certificates
(that happens every 3 months)

- `docker-run-no-api` starts docker compose with PostgreSQL and Redis
(API has to be run separately)

- `docker-run-api` starts docker compose with PostgreSQL, Redis and API

- `docker-stop` stops any of these two compose files

Read scripts to understand them better, they have comments inside
