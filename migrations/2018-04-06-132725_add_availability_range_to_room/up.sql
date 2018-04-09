alter table room add column available_from timestamp;
alter table room alter column available_from set not null;

alter table room add column available_to timestamp;
alter table room alter column available_to set not null;
