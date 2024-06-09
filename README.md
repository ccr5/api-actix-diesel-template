# api-actix-diesel-template
A API using Actix and Diesel ORM

- libpq
sudo apt-get install libpq-dev

- Extension CodeLLDB to debug
- Allow break point everywhere
```bash
$ "debug.allowBreakpointsEverywhere": true
```

- pgAdmin

- psql

psql 'postgres://postgres:postgres@localhost/actix_web'
\dt -> to list all tables

- diesel

diesel setup
diesel migration generate cash_in
diesel migration generate cash_out
