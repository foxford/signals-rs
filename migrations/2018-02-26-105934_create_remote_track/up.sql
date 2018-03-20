create table remote_track (
  id uuid default uuid_generate_v4(),
  local_track_id uuid not null,
  agent_id uuid not null,

  primary key (id),
  foreign key (local_track_id) references local_track (id) on delete cascade,
  foreign key (agent_id) references agent (id) on delete cascade
);

create index remote_track_local_track_id_idx on remote_track (local_track_id);
