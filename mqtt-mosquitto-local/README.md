### Install Mosquitto
```bash
sudo apt-get install mosquitto mosquitto-clients
```
### Send/Read Messages
```bash
mosquitto_pub -h localhost -t test -m "hello world"
mosquitto_sub -h localhost -t test
```
### Build / Run Rust MQTT app
```bash
cargo build
cargo run
```