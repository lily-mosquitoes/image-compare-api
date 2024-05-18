CREATE TABLE IF NOT EXISTS comparison (
	id BLOB PRIMARY KEY NOT NULL,
	dirname TEXT NOT NULL,
	images TEXT NOT NULL,
	created_at TEXT NOT NULL DEFAULT (datetime('now')),
	UNIQUE (dirname, images)
);
