-- Your SQL goes here
create table passive_results(
	id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))),2) || '-a' || substr(lower(hex(randomblob(2))),2) || '-%' || lower(hex(randomblob(6)))) COLLATE nocase NOT NULL,
	character_id TEXT,
	log_id TEXT,

	succeeded BOOLEAN NOT NULL,
	hunger INTEGER NOT NULL,
	total INTEGER NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

	
	FOREIGN KEY (character_id) REFERENCES characters (id),
	FOREIGN KEY (log_id) REFERENCES action_logs (id)
)