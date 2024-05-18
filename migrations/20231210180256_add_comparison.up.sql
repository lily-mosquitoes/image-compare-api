CREATE TABLE comparison (
	id BLOB PRIMARY KEY NOT NULL,
	dirname TEXT NOT NULL,
	images TEXT NOT NULL,
	created_at TEXT NOT NULL DEFAULT (datetime('now')),
	created_by INTEGER NOT NULL,
	FOREIGN KEY(created_by) REFERENCES admin(id)
		ON DELETE CASCADE,
	UNIQUE (dirname, images)
) WITHOUT ROWID;
