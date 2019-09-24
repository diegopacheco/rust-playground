### Build & Run for Docker
```bash
./release-rust.sh
./docker-build.sh
./run.sh
```
### Build & Run for K8s
```bash
minikube start
./k8s.deploy.sh
./run-k8s.sh
curl http://localhost:8080/
```
