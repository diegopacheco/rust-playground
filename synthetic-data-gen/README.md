# Synthetic Data Generator API

This is POC just for testing purposes, it generates synthetic data for testing and development.

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Result

```
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/main`
🚀 Synthetic Data Generator API running on http://0.0.0.0:3000
📋 Available endpoints:
   GET /health - Health check
   GET /generate?fields=first_name,last_name&count=5 - Generate data
📝 Available fields: first_name, last_name, email, dob, ssn
```

```bash
./test.sh
```

```json
{
  "data": [
    {
      "first_name": "Zach****",
      "last_name": "Pr***",
      "email": "zach***.p****@mail.com",
      "dob": "1945-02-07",
      "ssn": "220-**-*****"
    },
    {
      "first_name": "Car***",
      "last_name": "Cas*****",
      "email": "carolca*****90@yandex.com",
      "dob": "2002-06-14",
      "ssn": "199-**-****"
    }
  ],
  "count": 2
}
```
