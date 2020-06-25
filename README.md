# LegendKeeper to S3

This is a simple Rust cron job to on a regular basis request a JSON export of a LK project and store it in S3.

It requires manual rotation of your JWT and access to your email via IMAP in order to get the export link.

## To Use

Copy `config.example.toml` to a `config.toml` and replace the values with actual values. Environment variables will be used if no `config.toml` exists.

Ensure your environment has AWS credentials stored in a standard location, either as environment variables or in `~/.aws`.

Build the project with `cargo build --release` ([more details on Cargo can be found here](https://doc.rust-lang.org/cargo/)).

Run the project.

## Dockerfile

A Dockerfile exists to run the project on ARM hardware (read: Raspberry Pis).

## TODO

- [ ] Replace SSL with rustls
- [ ] Settle on either using reqwest or ureq
- [ ] Investigate automated JWT fetching
