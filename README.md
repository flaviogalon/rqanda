# Q&A website developed in Rust
Implemented following along the book [Rust Web Development](https://www.manning.com/books/rust-web-development) by Bastian Gruber.

## TODO
Things that I'd like to see but not sure if covered by the book:
- [ ] Handle duplication in POST /questions
- [ ] Auto-generate `id` when creating questions and answers
- [ ] Make `id` optional in body when updating questions
- [ ] Don't override fields that are not updated
  - E.g: if the PUT request don't have `tags` it will be set to `None`
- [ ] Get question by ID
- [ ] Add exception handling to add answer endpoint
- [ ] Check question ID before saving an answer
- [ ] Add a get answer by question ID endpoint?