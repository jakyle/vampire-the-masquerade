-- Your SQL goes here
CREATE TABLE characters (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))),2) || '-a' || substr(lower(hex(randomblob(2))),2) || '-%' || lower(hex(randomblob(6)))) COLLATE nocase NOT NULL,
    name TEXT NOT NULL,

    is_active BOOLEAN NOT NULL DEFAULT 1,

    strength INTEGER NOT NULL,
    dexterity INTEGER NOT NULL,
    stamina INTEGER NOT NULL,

    charisma INTEGER NOT NULL,
    manipulation INTEGER NOT NULL,
    composure INTEGER NOT NULL,

    intelligence INTEGER NOT NULL,
    wits INTEGER NOT NULL,
    resolve INTEGER NOT NULL,

    athletics INTEGER NOT NULL,
    brawl INTEGER NOT NULL,
    crafts INTEGER NOT NULL,
    drive INTEGER NOT NULL,
    firearms INTEGER NOT NULL,
    melee INTEGER NOT NULL,
    larceny INTEGER NOT NULL,
    stealth INTEGER NOT NULL,
    survival INTEGER NOT NULL,

    animal_ken INTEGER NOT NULL,
    etiquette INTEGER NOT NULL,
    insight INTEGER NOT NULL,
    intimidation INTEGER NOT NULL,
    leadership INTEGER NOT NULL,
    performance INTEGER NOT NULL,
    persuasion INTEGER NOT NULL,
    streetwise INTEGER NOT NULL,
    subterfuge INTEGER NOT NULL,

    academics INTEGER NOT NULL,
    awareness INTEGER NOT NULL,
    finance INTEGER NOT NULL,
    investigation INTEGER NOT NULL,
    medicine INTEGER NOT NULL,
    occult INTEGER NOT NULL,
    politics INTEGER NOT NULL,
    science INTEGER NOT NULL,
    technology INTEGER NOT NULL,

    hunger INTEGER NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)