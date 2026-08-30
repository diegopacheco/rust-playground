PRAGMA foreign_keys = ON;

CREATE TABLE authors (
  id      INTEGER PRIMARY KEY AUTOINCREMENT,
  name    TEXT NOT NULL UNIQUE,
  born    INTEGER
);

CREATE TABLE books (
  id        INTEGER PRIMARY KEY AUTOINCREMENT,
  author_id INTEGER NOT NULL REFERENCES authors(id) ON DELETE CASCADE,
  title     TEXT NOT NULL,
  year      INTEGER NOT NULL,
  copies    INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE members (
  id        INTEGER PRIMARY KEY AUTOINCREMENT,
  name      TEXT NOT NULL,
  joined_at TEXT NOT NULL
);

CREATE TABLE loans (
  id        INTEGER PRIMARY KEY AUTOINCREMENT,
  book_id   INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
  member_id INTEGER NOT NULL REFERENCES members(id) ON DELETE CASCADE,
  taken_at  TEXT NOT NULL,
  returned_at TEXT
);

CREATE INDEX idx_books_author ON books(author_id);
CREATE INDEX idx_loans_open ON loans(member_id) WHERE returned_at IS NULL;

CREATE VIEW open_loans AS
  SELECT l.id, b.title, m.name AS member, l.taken_at
  FROM loans l
  JOIN books b ON b.id = l.book_id
  JOIN members m ON m.id = l.member_id
  WHERE l.returned_at IS NULL;

INSERT INTO authors (name, born) VALUES
  ('Ursula K. Le Guin', 1929),
  ('Italo Calvino', 1923),
  ('Clarice Lispector', 1920),
  ('Stanislaw Lem', 1921);

INSERT INTO books (author_id, title, year, copies) VALUES
  (1, 'The Dispossessed', 1974, 3),
  (1, 'A Wizard of Earthsea', 1968, 2),
  (2, 'Invisible Cities', 1972, 4),
  (2, 'If on a Winter''s Night a Traveler', 1979, 1),
  (3, 'The Hour of the Star', 1977, 2),
  (4, 'Solaris', 1961, 5),
  (4, 'The Cyberiad', 1965, 2);

INSERT INTO members (name, joined_at) VALUES
  ('Bruno Alves', '2024-05-02'),
  ('Nina Costa', '2025-01-18'),
  ('Yuki Mori', '2025-09-30');

INSERT INTO loans (book_id, member_id, taken_at, returned_at) VALUES
  (1, 1, '2026-01-05', '2026-01-26'),
  (3, 1, '2026-02-10', NULL),
  (6, 2, '2026-02-14', NULL),
  (5, 3, '2026-03-02', '2026-03-20'),
  (7, 3, '2026-03-21', NULL);
