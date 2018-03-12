CREATE TABLE room (
  id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  label TEXT NOT NULL
);
