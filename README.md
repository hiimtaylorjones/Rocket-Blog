# Rocket-Blog
A Rust Based Blog made with Rocket!


## Requirements

* Rust Nightly
* Postgres
* A Heroku Account
* A little bit of patience 

## Setup

```
cargo build
```

### Diesel

Diesel requires a live Postgres installation to run commands and configuration against.
To accomplish this, we needed to build our containers first. To actually start modeling the database
itself, run:

```
 diesel setup
```

This will configure all of our database concerns. You may also have to run:

```
diesel migration run
```

to properly migrate all pending database concerns.

## Running in Development

To launch the web app locally, you simply can run:

```
cargo run
```

## Deploying to Heroku


