# Rust webserver

[![Build](https://img.shields.io/github/actions/workflow/status/Efimish/rust-webserver/build.yaml?logo=GitHub)](https://github.com/Efimish/rust-webserver)
[![Size](https://img.shields.io/github/languages/code-size/Efimish/rust-webserver)](https://github.com/Efimish/rust-webserver)
[![License](https://img.shields.io/github/license/Efimish/rust-webserver)](https://github.com/Efimish/rust-webserver/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/Efimish/rust-webserver/status.svg)](https://deps.rs/repo/github/Efimish/rust-webserver)

Simple REST api written in `Rust` using `axum` framework\
~~I also have a simple website to test it:~~\
It was not updated in ages, not currently maintained
and I'm really bad at front-end stuff
[`website`](../../../website)

## Tech Stack

![Rust](https://img.shields.io/badge/-Rust-000?logo=rust&logoColor=fff)\
![Docker](https://img.shields.io/badge/-Docker-2496ED?logo=docker&logoColor=fff)
![PostgreSQL](https://img.shields.io/badge/-PostgreSQL-4169E1?logo=postgresql&logoColor=fff)
![Redis](https://img.shields.io/badge/-Redis-DC382D?logo=redis&logoColor=fff)
![NGINX](https://img.shields.io/badge/-NGINX-009639?logo=nginx&logoColor=fff)
![Certbot](https://img.shields.io/badge/-Certbot-003A70?logo=letsencrypt&logoColor=fff)\
![JWT](https://img.shields.io/badge/-Json%20Web%20Tokens-000?logo=jsonwebtokens)

## Information

This is basically a REST api\
I wrote it for fun and learning purposes\
It consists of five parts:

- Api itself
- Postgres database to store data
- Redis database (not currently used for anything)
- Nginx as reverse proxy to serve on ports 80 and 443 for http and https respectively, serve certificates and static files
- Certbot to get and renew ssl certificates

Postgres, Redis and Certbot are run using Docker, check one of `docker-compose` files\
Nginx is run natively (because it needs to work on host network)\
Api could be run both ways (docker/native)

## Features

- SSL certificates generation / renewal using `Certbot`
- Communication with database using `sqlx`
- `RSA` keys generation
- `JsonWebTokens` signing / validation
- Hashing passwords using `Argon2`
- Request body validation
- User agent string parser
- Getting user's country and city based on ip address
- Sending emails through SMTP
- Serving static files

## File structure

I like to change it somethimes and move stuff around, but still:

- `.sqlx` - sqlx queries metadata saved to build in offline mode on github and docker
- `data` - data, not related to api. Secured there using volumes in docker-compose and not only
- `keys` - RSA keys, server loads them from there. In case they are not found, they will be generated there
- `migrations` - raw SQL migrations that form the database structure from scratch. Used by `sqlx`
- `scripts` - shell scripts that help to do some stuff easier.
- `static` - contains static files
- `src` - source code, it is documented using `rustdoc`, check it out

## Pre-Installation

Necessary components and tools:

- `curl`
- `envsubst`
- `git`
- `nginx`
- `docker` and `docker-compose`
- `rust` and `cargo`
- `sqlx-cli`

## Installation

1. Clone this repo
2. Create `.env` file in the root folder and fill it (check `.env.example`) (or use `scripts/copy-env`)
3. Download `regexes.yaml` into root folder using `scripts/download-regexes` (or manually)
4. Run docker compose (`scripts/docker-run-[api|no-api]`)
5. Run migrations from `/migrations` folder (`sqlx migrate run`)
6. Start nginx (`scripts/nginx-start`)
7. Test and get your certificates (`scripts/certificates-[test|get]`)
8. If you decided to run api outside of docker, start it (`cargo run`)
