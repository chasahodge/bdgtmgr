CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  account_name VARCHAR NOT NULL,
  occurrance VARCHAR NOT NULL,
  amount NUMERIC NOT NULL
)