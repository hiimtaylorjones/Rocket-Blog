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

### Via Heroku stacks and Buildpacks

Once you have the repo downloaded, you'll want to run the following:

```
heroku create --buildpack https://github.com/emk/heroku-buildpack-rust
heroku addons:create heroku-postgresql:hobby-dev
```

This will create a new Heroku app and install the [Heroku Rust Buildpack](https://github.com/emk/heroku-buildpack-rust). It will also provision a free version of Heroku Postgres - which we need for a database url. Right now, we need to install the `master` branch of the buildpack in order for Diesel to work properly.

### Via Dockerfile

