CREATE TABLE hosts (
  id     INTEGER PRIMARY KEY AUTOINCREMENT,
  name   TEXT NOT NULL UNIQUE,
  region TEXT NOT NULL
);

CREATE TABLE samples (
  id      INTEGER PRIMARY KEY AUTOINCREMENT,
  host_id INTEGER NOT NULL REFERENCES hosts(id) ON DELETE CASCADE,
  metric  TEXT NOT NULL,
  value   REAL NOT NULL,
  taken_at TEXT NOT NULL
);

CREATE INDEX idx_samples_host_metric ON samples(host_id, metric);
CREATE INDEX idx_samples_taken ON samples(taken_at DESC);

CREATE VIEW latest_cpu AS
  SELECT h.name, s.value, s.taken_at
  FROM samples s JOIN hosts h ON h.id = s.host_id
  WHERE s.metric = 'cpu'
  ORDER BY s.taken_at DESC;

INSERT INTO hosts (name, region) VALUES
  ('web-01', 'us-east-1'),
  ('web-02', 'us-east-1'),
  ('db-01', 'eu-west-1'),
  ('cache-01', 'sa-east-1');

WITH RECURSIVE tick(n) AS (
  SELECT 0 UNION ALL SELECT n + 1 FROM tick WHERE n < 249
)
INSERT INTO samples (host_id, metric, value, taken_at)
SELECT h.id,
       m.metric,
       round(abs(random() % 10000) / 100.0, 2),
       datetime('2026-03-01 00:00:00', '+' || (n * 5) || ' minutes')
FROM tick
CROSS JOIN hosts h
CROSS JOIN (SELECT 'cpu' AS metric UNION ALL SELECT 'memory' UNION ALL SELECT 'latency') m;
