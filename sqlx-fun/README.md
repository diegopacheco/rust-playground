# Real World SQLx

## Usage
Declare the database URL.

```
export DATABASE_URL="postgres://postgres:docker@172.17.0.2/postgres"
```

Insert data
```
insert into users values(1,now(),now(),'diego.pacheco@gmail.com','diego','diego');
```

Load the database schema.
```
psql -d "$DATABASE_URL" -f ./schema.sql
``` 

Run.
```
cargo run
```
