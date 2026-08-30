PRAGMA foreign_keys = ON;

CREATE TABLE customers (
  id      INTEGER PRIMARY KEY AUTOINCREMENT,
  name    TEXT NOT NULL,
  email   TEXT NOT NULL UNIQUE,
  country TEXT NOT NULL
);

CREATE TABLE products (
  id       INTEGER PRIMARY KEY AUTOINCREMENT,
  sku      TEXT NOT NULL UNIQUE,
  name     TEXT NOT NULL,
  price    REAL NOT NULL,
  in_stock INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE orders (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  customer_id INTEGER NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
  placed_at   TEXT NOT NULL,
  status      TEXT NOT NULL
);

CREATE TABLE order_items (
  order_id   INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  product_id INTEGER NOT NULL REFERENCES products(id),
  quantity   INTEGER NOT NULL,
  PRIMARY KEY (order_id, product_id)
);

CREATE INDEX idx_orders_customer ON orders(customer_id);
CREATE INDEX idx_orders_status ON orders(status);

CREATE VIEW open_orders AS
  SELECT o.id, c.name AS customer, o.placed_at
  FROM orders o JOIN customers c ON c.id = o.customer_id
  WHERE o.status <> 'shipped';

INSERT INTO customers (name, email, country) VALUES
  ('Ana Ribeiro', 'ana@example.com', 'BR'),
  ('Lars Holm', 'lars@example.com', 'DK'),
  ('Mei Tanaka', 'mei@example.com', 'JP'),
  ('Omar Haddad', 'omar@example.com', 'MA'),
  ('Sofia Duarte', 'sofia@example.com', 'PT');

INSERT INTO products (sku, name, price, in_stock) VALUES
  ('KB-01', 'Mechanical Keyboard', 129.90, 42),
  ('MS-02', 'Trackball Mouse', 79.50, 17),
  ('MN-27', '27 inch Monitor', 349.00, 8),
  ('CH-05', 'Ergonomic Chair', 610.00, 3),
  ('HP-09', 'Studio Headphones', 199.99, 25),
  ('DK-11', 'Standing Desk', 780.00, 0);

INSERT INTO orders (customer_id, placed_at, status) VALUES
  (1, '2026-01-14 09:12:00', 'shipped'),
  (1, '2026-02-02 17:40:00', 'packing'),
  (2, '2026-02-11 11:05:00', 'shipped'),
  (3, '2026-03-01 08:30:00', 'new'),
  (4, '2026-03-07 14:22:00', 'packing'),
  (5, '2026-03-19 19:58:00', 'new');

INSERT INTO order_items (order_id, product_id, quantity) VALUES
  (1, 1, 1), (1, 2, 2),
  (2, 3, 1),
  (3, 5, 1), (3, 2, 1),
  (4, 4, 1),
  (5, 6, 1), (5, 1, 1),
  (6, 5, 3);
