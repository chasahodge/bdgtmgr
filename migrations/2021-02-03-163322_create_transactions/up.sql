CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  account_name VARCHAR NOT NULL,
  account_number VARCHAR NOT NULL,
  locale VARCHAR NOT NULL,
  occurrance VARCHAR NOT NULL,
  amount FLOAT NOT NULL
)
