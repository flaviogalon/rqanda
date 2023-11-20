CREATE TABLE IF NOT EXISTS answers (
    id serial PRIMARY KEY,
    content TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    question_id INTEGER,
    CONSTRAINT fk_question_id FOREIGN KEY(question_id) REFERENCES questions(id)
);