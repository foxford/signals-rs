create table room (
  id uuid default uuid_generate_v4(),
  created_at timestamp not null default now(),

  primary key (id)
);
