# Rust webserver
Simple web server written in `Rust` using `axum` framwork, `PostgreSQL` and `sqlx` for queries.\
Frontend to test this: [`website`](../../../website)

### Installation
1. Clone this repo
2. Start your PostgreSQL database
3. Start your Redis database
4. Setup `.env` file (check `.env.example`)
5. Install sqlx-cli (`cargo install sqlx-cli`)
6. Run migrations from `/migrations` folder (`sqlx migrate run`)
7. Build and start the server (`cargo run`)

### To-do list
- [x] Add working authorization
- [x] Make a normal error enum
- [ ] Make a normal database schema
- [ ] Add Docker files to make installation easy
- [ ] Make front-end website work (at least)
