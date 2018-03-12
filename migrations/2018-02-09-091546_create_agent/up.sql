CREATE TABLE agent (
  id uuid PRIMARY KEY NOT NULL,
  label TEXT NOT NULL,
  room_id uuid NOT NULL,

  FOREIGN KEY (room_id) REFERENCES room (id) ON DELETE CASCADE
);
