create table room (
  id uuid not null default uuid_generate_v4(),

  primary key (id)
);
