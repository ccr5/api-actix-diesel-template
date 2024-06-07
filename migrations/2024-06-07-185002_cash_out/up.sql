CREATE TABLE cash_out (
    id SERIAL PRIMARY KEY,
    amount DECIMAL(10, 2) NOT NULL,
    description VARCHAR(255) NOT NULL,
    cash_out_type VARCHAR(255) NOT NULL,
    installment_number INT,
    total_installments INT,
    date DATE NOT NULL,
    status VARCHAR(255) NOT NULL,
    observation TEXT
);