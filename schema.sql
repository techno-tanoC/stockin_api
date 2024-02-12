CREATE TABLE items(
  id         TEXT NOT NULL PRIMARY KEY,
  title      TEXT NOT NULL,
  url        TEXT NOT NULL,
  thumbnail  TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);
