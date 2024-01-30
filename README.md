# Rust webserver
![Build](https://img.shields.io/github/actions/workflow/status/Efimish/rust-webserver/build.yaml?logo=GitHub)
[![Size](https://img.shields.io/github/languages/code-size/Efimish/rust-webserver)](https://github.com/Efimsih/rust-webserver)
![License](https://img.shields.io/github/license/Efimish/rust-webserver)

Simple web server written in `Rust` using `axum` framwork, `PostgreSQL` and `sqlx` for queries.\
Frontend to test this: [`website`](../../../website)

### Installation
1. Clone this repo
2. Setup `.env` file (check `.env.example`)
3. Run docker compose (`docker compose up -d`)
4. Install sqlx-cli (`cargo install sqlx-cli`)
5. Run migrations from `/migrations` folder (`sqlx migrate run`)
6. Build and start the server (`cargo run`)

### To-do list
- [x] Add working authorization
- [x] Make a normal error enum
- [ ] Make a normal database schema
- [x] Add Docker files to make installation easy
- [ ] Make front-end website work (at least)
