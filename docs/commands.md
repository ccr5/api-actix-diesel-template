# Resources

## libpq

### Setting
```bash
sudo apt-get install libpq-dev
```

## psql

- Interacte with Postgres
```bash
psql 'postgres://root:postgres@localhost/postgres'
```

- to list all tables
```bash
\dt 
```

## diesel

### From zero
```bash
diesel setup
diesel migration generate cash_in
```

### From migrations
```bash
diesel migration run
```

# VS Code Extension

### CodeLLDB to debug
- Allow break point everywhere
```bash
$ "debug.allowBreakpointsEverywhere": true
```
