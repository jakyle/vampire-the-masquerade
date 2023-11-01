-- Your SQL goes here
create table dice_results(
	id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))),2) || '-a' || substr(lower(hex(randomblob(2))),2) || '-%' || lower(hex(randomblob(6)))) COLLATE nocase NOT NULL,
	character_id TEXT,
	log_id TEXT,

	successes INTEGER NOT NULL,
	criticals INTEGER NOT NULL,
	beastial_failure BOOLEAN NOT NULL,
	messy_critical BOOLEAN NOT NULL,
	succeeded BOOLEAN NOT NULL,

	rolls TEXT NOT NULL,
	hunger_rolls TEXT NOT NULL,

	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

	FOREIGN KEY (character_id) REFERENCES characters (id),
	FOREIGN KEY (log_id) REFERENCES action_logs (id)
)

