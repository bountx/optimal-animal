-- Your SQL goes here
CREATE TABLE votes (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id),
  animal_id UUID REFERENCES animals(id),
  score1 INT2,
  score2 INT2,
  score3 INT2,
  score4 INT2,
  score5 INT2,
  score6 INT2,
  score7 INT2,
  score8 INT2,
  score9 INT2,
  score10 INT2
);