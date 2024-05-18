CREATE TABLE vote (
	comparison_id BLOB NOT NULL,
	user_id BLOB NOT NULL,
	image TEXT NOT NULL,
	status INTEGER NOT NULL DEFAULT 201,
	FOREIGN KEY(comparison_id) REFERENCES comparison(id)
		ON DELETE CASCADE,
	FOREIGN KEY(user_id) REFERENCES user(id)
		ON DELETE CASCADE,
	PRIMARY KEY (comparison_id, user_id)
) WITHOUT ROWID;
