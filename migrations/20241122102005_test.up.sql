CREATE TYPE test_enum AS enum ('test_example', 'another_example');

CREATE TABLE test (
  id SERIAL PRIMARY KEY,
  value test_enum NOT NULL
);
