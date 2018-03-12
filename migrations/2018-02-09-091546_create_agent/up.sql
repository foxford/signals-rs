create table agent (
  id uuid not null,
  label text not null,
  room_id uuid not null,

  primary key (id),
  foreign key (room_id) references room (id) on delete cascade
);
