-- Your SQL goes here
CREATE TABLE animals (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  description_post TEXT,
  score1 FLOAT8,
  score2 FLOAT8,
  score3 FLOAT8,
  score4 FLOAT8,
  score5 FLOAT8,
  score6 FLOAT8,
  score7 FLOAT8,
  score8 FLOAT8,
  score9 FLOAT8,
  score10 FLOAT8,
  n_of_votes INT8 DEFAULT 0
);