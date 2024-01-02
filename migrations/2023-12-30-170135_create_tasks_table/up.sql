create table tasks (
	id text not null primary key,
	name text not null,
	due_date date not null,
	due_time time,
	complete boolean not null default false
);