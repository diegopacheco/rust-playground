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
=== Cedar Policy S3 Access Check ===

User 'alice' -> s3:GetObject on 'my-data-bucket': ALLOW
User 'alice' -> s3:PutObject on 'my-data-bucket': ALLOW
User 'alice' -> s3:DeleteObject on 'my-data-bucket': DENY
User 'bob' -> s3:GetObject on 'public-bucket': ALLOW
User 'bob' -> s3:DeleteObject on 'public-bucket': DENY
User 'charlie' -> s3:GetObject on 'my-data-bucket': DENY
```
