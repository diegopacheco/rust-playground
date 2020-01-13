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
curl -s http://localhost:8080/news/0dae39e4-fca2-b076-4f88-617dd3352d11 | jq .
curl -s -X DELETE "http://localhost:8080/news/0dae39e4-fca2-b076-4f88-617dd3352d11"
```
