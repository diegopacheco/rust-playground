### Build
```bash
cargo build
```
### Run
```bash
./run-docker-postgress.sh
RUST_LOG=info cargo run --bin news-service
```
### Test it
```bash
curl http://localhost:8080/news
```
