CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  date DATE NOT NULL,
  person VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  original_description VARCHAR,
  amount DOUBLE PRECISION NOT NULL,
  transaction_type VARCHAR NOT NULL,
  category VARCHAR NOT NULL,
  original_category VARCHAR NOT NULL,
  account_name VARCHAR NOT NULL,
  labels VARCHAR NOT NULL,
  notes VARCHAR NOT NULL
)
