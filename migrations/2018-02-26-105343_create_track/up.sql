create table track (
  id uuid default uuid_generate_v4(),
  owner_id uuid not null,
  metadata jsonb not null default '{}',

  primary key (id),
  foreign key (owner_id) references agent (id) on delete cascade
);

create index track_owner_id_idx on track (owner_id);
