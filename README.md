To reproduce:
```sh
git clone git@github.com:roignpar/rdkafka_tx_commit_error.git
cd rdkafka_tx_commit_error
docker-compose up -d
RUST_LOG=trace cargo run
```
