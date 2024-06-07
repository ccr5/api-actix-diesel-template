CREATE TABLE cash_in (
    id SERIAL PRIMARY KEY,
    amount DECIMAL(10, 2) NOT NULL,
    description VARCHAR(300) NOT NULL,
    date TIMESTAMP NOT NULL,
    status VARCHAR(50) NOT NULL
);