create table local_track (
  id uuid default uuid_generate_v4(),
  stream_id text not null,
  track_id text not null,
  device text not null,
  kind text not null,
  label text not null,
  owner_id uuid not null,

  primary key (id),
  foreign key (owner_id) references agent (id) on delete cascade
);

create index local_track_owner_id_idx on local_track (owner_id);
