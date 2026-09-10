CREATE TABLE IF NOT EXISTS reactions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  message_id INTEGER NOT NULL,
  author TEXT NOT NULL,
  emoji TEXT NOT NULL,
  UNIQUE(message_id, author, emoji),
  FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
);
