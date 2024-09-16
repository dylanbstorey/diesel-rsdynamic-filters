# Examples of dynamic filtering with [diesel-rs](https://github.com/diesel-rs/diesel)

First - this repo is **HEAVILY** based on the examples in [diesel-rs-dynamic-filters](https://github.com/andreievg/diesel-rs-dynamic-filters), and by "heavily" I mean - its all their work, i just needed to restructure it to make it fit how I think about the software architectures I tend to build. So thank you Andrei for the original work!




## Setting up and running the examples

1. Clone the repo.
2. Start a postgres instance using Docker
```
docker run --name postgres-test \
  -e POSTGRESQL_PASSWORD=password \
  -e POSTGRESQL_DATABASE=postgres \
  -p 5432:5432 \
  -d bitnami/postgresql:latest
```
3. Run the integration tests via `cargo test --test integration`

