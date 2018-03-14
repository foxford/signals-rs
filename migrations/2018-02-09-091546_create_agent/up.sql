create table agent (
  id uuid not null,
  label text not null,
  room_id uuid not null,
  created_at timestamp not null default now(),

  primary key (id),
  foreign key (room_id) references room (id) on delete cascade
);
