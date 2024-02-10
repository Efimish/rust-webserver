# Rust webserver
[![Build](https://img.shields.io/github/actions/workflow/status/Efimish/rust-webserver/build.yaml?logo=GitHub)](https://github.com/Efimish/rust-webserver)
[![Size](https://img.shields.io/github/languages/code-size/Efimish/rust-webserver)](https://github.com/Efimish/rust-webserver)
[![License](https://img.shields.io/github/license/Efimish/rust-webserver)](https://github.com/Efimish/rust-webserver/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/Efimish/rust-webserver/status.svg)](https://deps.rs/repo/github/Efimish/rust-webserver)

Simple web server written in `Rust` using `axum` framework and `sqlx` for queries.\
I also have a simple website to test it:
[`website`](../../../website)

## Tech Stack
![Docker](https://img.shields.io/badge/-Docker-2496ED?logo=docker&logoColor=fff)
![PostgreSQL](https://img.shields.io/badge/-PostgreSQL-4169E1?logo=postgresql&logoColor=fff)
![Redis](https://img.shields.io/badge/-Redis-DC382D?logo=redis&logoColor=fff)
![NGINX](https://img.shields.io/badge/-NGINX-009639?logo=nginx&logoColor=fff)
![Certbot](https://img.shields.io/badge/-Certbot-003A70?logo=letsencrypt&logoColor=fff)\
![JWT](https://img.shields.io/badge/-Json%20Web%20Tokens-000?logo=jsonwebtokens)

### Information
Everything, except the app itself is working in Docker.\
The app may be containertized later.\
Before installing, make sure you have following tools installed:\
`git`, `cargo`, `docker`, `docker-compose`, `sqlx-cli`

### Installation
1. Clone this repo
2. Setup `.env` file (check `.env.example`)
3. Run docker compose (`docker compose up -d`)
4. Test and get your certificates (check `./data/scripts` folder)
5. Run migrations from `/migrations` folder (`sqlx migrate run`)
6. Build and start the server (`cargo run`)

### To-do list
- [ ] Containerize server
- [x] Containerize everything else
- [x] Make Authentication work properly
- [x] Make an error enum
- [ ] Make a website for testing
