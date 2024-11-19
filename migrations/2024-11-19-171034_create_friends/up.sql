-- Your SQL goes here
CREATE TABLE friends (
  id UUID PRIMARY KEY,
  user1 UUID REFERENCES users(id),
  user2 UUID REFERENCES users(id)
);