create table room_agent (
  agent_id uuid,
  room_id uuid,
  label text not null,
  created_at timestamp not null default now(),

  primary key (agent_id, room_id),
  foreign key (agent_id) references agent (id) on delete cascade,
  foreign key (room_id) references room (id) on delete cascade
);

create index room_agent_room_id_idx on room_agent (room_id);
