CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  account_name VARCHAR NOT NULL,
  account_holder TEXT NOT NULL,
  balance NUMERIC NOT NULL DEFAULT 0.00
)
