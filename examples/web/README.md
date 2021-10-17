# Web example

Simple rest api example with hyper, bb8, postgres

## Deps

For this example you need to install [docker] with [docker-compose], [nightly rust]. Follow the instructions on the official sites.

[docker]: https://docs.docker.com/get-docker/
[docker-compose]: https://docs.docker.com/compose/install/
[nightly rust]: https://www.rust-lang.org/tools/install

## Running

Move to the example directory

```sh
cd examples/web
```

Run configuration for docker-compose

```sh
docker-compose -f docker-compose.dev.yml up
```

Or run postgres server manually.

Then copy `.env.example` to `.env` and edit if you needed.

```sh
cp .env.example .env
```

Now you can run server

```sh
cargo run --features dev
```

Or if you have a [cargo make]

```sh
cargo make dev
```

[cargo make]: https://github.com/sagiegurari/cargo-make
