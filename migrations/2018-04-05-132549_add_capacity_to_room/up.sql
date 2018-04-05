alter table room add column capacity smallint check (capacity > 0);
alter table room alter column capacity set not null;
