-- Your SQL goes here
create table action_logs(
	id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))),2) || '-a' || substr(lower(hex(randomblob(2))),2) || '-%' || lower(hex(randomblob(6)))) COLLATE nocase NOT NULL,

	difficulty INTEGER NOT NULL,
	modifier INTEGER NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)