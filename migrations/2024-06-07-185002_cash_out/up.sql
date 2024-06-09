CREATE TABLE cash_out (
    id SERIAL PRIMARY KEY,
    description VARCHAR(255) NOT NULL,
    installment_number INT,
    total_installments INT,
    status VARCHAR(255) NOT NULL,
    observation TEXT
);