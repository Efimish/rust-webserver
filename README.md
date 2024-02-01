# Rust webserver
[![Build](https://img.shields.io/github/actions/workflow/status/Efimish/rust-webserver/build.yaml?logo=GitHub)](https://github.com/Efimish/rust-webserver)
[![Size](https://img.shields.io/github/languages/code-size/Efimish/rust-webserver)](https://github.com/Efimish/rust-webserver)
[![License](https://img.shields.io/github/license/Efimish/rust-webserver)](https://github.com/Efimish/rust-webserver/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/Efimish/rust-webserver/status.svg)](https://deps.rs/repo/github/Efimish/rust-webserver)

Simple web server written in `Rust` using `axum` framwork, `PostgreSQL` and `sqlx` for queries.\
Frontend to test this: [`website`](../../../website)

### Installation
1. Clone this repo
2. Setup `.env` file (check `.env.example`)
3. Run docker compose (`docker compose up -d`)
4. Test and get your certificates (check `./data/scripts` folder)
5. Install sqlx-cli (`cargo install sqlx-cli`)
6. Run migrations from `/migrations` folder (`sqlx migrate run`)
7. Build and start the server (`cargo run`)

### To-do list
- [x] Add working authorization
- [x] Make a normal error enum
- [ ] Make a normal database schema
- [x] Add Docker files to make installation easy
- [ ] Make front-end website work (at least)