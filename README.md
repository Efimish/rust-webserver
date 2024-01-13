# Rust webserver
Simple web server written in `Rust` using `axum` framwork, `PostgreSQL` and `sqlx` for queries.\
Frontend to test this: [`website`](../../../website)

### Installation
1. Clone this repo
2. Start your PostgreSQL database
3. Setup `.env` file (check `.env.example`)
4. Install sqlx-cli (`cargo install sqlx-cli`)
5. Run migrations from `/migrations` folder (`sqlx migrate run`)
6. Now you can start the server (`cargo run`)

### To-do list
- [ ] Make a normal error enum
- [ ] Make a normal database schema
- [ ] Make models and static queries for them (imitating ORM)
- [x] Add working authorization
- [ ] Make front-end website work (at least)
