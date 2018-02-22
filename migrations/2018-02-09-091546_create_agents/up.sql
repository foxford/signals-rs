CREATE TABLE agents (
  id uuid PRIMARY KEY NOT NULL,
  label VARCHAR NOT NULL DEFAULT '',
  room_id uuid NOT NULL REFERENCES rooms (id)
)
