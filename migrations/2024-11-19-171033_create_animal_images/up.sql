-- Your SQL goes here
CREATE TABLE animal_images (
  id UUID PRIMARY KEY,
  animal_id UUID REFERENCES animals(id),
  image_url TEXT NOT NULL
);