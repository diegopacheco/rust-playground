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
curl -s http://localhost:8080/news | jq .
curl -s -X PUT "http://localhost:8080/news/facebook/faceboo.com"
```
