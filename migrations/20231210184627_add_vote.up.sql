CREATE TABLE IF NOT EXISTS vote (
	comparison_id BLOB NOT NULL,
	user_id BLOB NOT NULL,
	image TEXT NOT NULL,
	FOREIGN KEY(comparison_id) REFERENCES comparison(id)
		ON DELETE CASCADE,
	FOREIGN KEY(user_id) REFERENCES user(id)
		ON DELETE CASCADE,
	PRIMARY KEY (comparison_id, user_id)
);
