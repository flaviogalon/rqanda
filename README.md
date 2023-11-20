# Q&A website developed in Rust
Implemented following along the book [Rust Web Development](https://www.manning.com/books/rust-web-development) by Bastian Gruber.

## How to run
This is 💩 at the moment, I know. By the end of the book we should have a docker-compose setup with all services on it.

### 1. Start DB
```shell
docker compose up -d
```
### 2. Start application
```shell
cargo run
```

## Running migrations
For now these steps are manual
### Apply migration
```shell
sqlx migrate run -D {db_url}
```
### Revert migration
```shell
sqlx migrate revert -D {db_url}
``` 

## TODO
Things that I'd like to see but not sure if covered by the book:
- [ ] Handle duplication in POST /questions
- [ ] Auto-generate `id` when creating answers
- [ ] Get question by ID
- [ ] Add exception handling to add answer endpoint
- [ ] Check question ID before saving an answer
- [ ] Add a get answer by question ID endpoint?
- [ ] Wait for a connection to the DB before accepting requests
- [ ] Run migrations in dockerfile (when I have one 🙃)
  - I might want to have an script to do this instead of having to install the CLI just for this in a "prod like" scenario...