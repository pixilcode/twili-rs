create table tasks (
	id text not null primary key,
	name text not null,
	due_date text not null,
	due_time text,
	complete integer not null
);