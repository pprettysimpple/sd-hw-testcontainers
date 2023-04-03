#### How to run services
- In the root directory of the project:
- `cargo build --example account_server`
- `cargo build --example exchange_server`
- `docker-compose build --parallel` (this may take a while)
- `docker-compose up`
- Enjoy your services are running on:
  - account_server on `localhost:4321`
  - exchange_server on `localhost:1337`

#### How to run tests
- In the root directory of the project:
- Set up the services with the above steps
- `cargo test --all`
- To run it once again:
- `docker-compose down`
- `docker-compose up`
- `cargo test --all`

#### To access services via repl
- In the root directory of the project:
- `cargo run --example account_repl` for main frontend methods
- `cargo run --example exchange_repl` for "admin" methods

Type `help` in repl to see the available commands
