DROP TABLE IF EXISTS regions;
DROP TABLE IF EXISTS orders;

CREATE TABLE regions (
  id INT PRIMARY KEY,
  name VARCHAR(50)
);

CREATE TABLE orders (
  id INT PRIMARY KEY,
  region_id INT,
  gift_name VARCHAR(50),
  quantity INT
);