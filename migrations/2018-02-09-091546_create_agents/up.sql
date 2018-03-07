CREATE TABLE agents (
  id uuid PRIMARY KEY NOT NULL,
  label TEXT NOT NULL DEFAULT '',
  room_id uuid NOT NULL REFERENCES rooms (id)
)
