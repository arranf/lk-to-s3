# LegendKeeper to S3

This is a simple Rust cron job to on a regular basis request a JSON export of a LK project and store it in S3.

It requires manual rotation of your JWT and access to your email via IMAP in order to get the export link.

## To Use

Either setup a dotenv file by copying `.env.example` to `.env` and replacing the values with actual values, _or_ set the environment variables listed in `.env.example` manually.

Ensure your environment has AWS credentials stored in a standard location, either as environment variables ( `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` ) or in `~/.aws`.

Build the project with `cargo build --release` ([more details on Cargo can be found here](https://doc.rust-lang.org/cargo/)).

Run the project.

## Dockerfile

A Dockerfile exists to run the project on ARM hardware (read: Raspberry Pis).

## TODO

- [ ] Settle on either using reqwest or ureq
- [ ] Investigate automated JWT fetching
