# Rocket-Blog
A Rust Based Blog made with Rocket!


## Setup

### Docker

This project is dockerized and ready for portability. However, there's a certain flow you'll
need to follow to configure the application properly.

First up, create the docker containers:

```
docker-compose build web
```

This will create and configure the Docker containers needed to help the app run.

### Diesel

Diesel requires a live Postgres installation to run commands and configuration against.
To accomplish this, we needed to build our containers first. To actually start modeling the database
itself, run:

```
docker-compose run web diesel setup
```

This will configure all of our database concerns. You may also have to run:

```
docker-compose run web diesel migration run
```

to properly migrate all pending database concerns.

## Running in Development

To launch the web app locally, you simply can run:

```
docker-compose up
```

This will orchestrate all of our elements together and create a functional app
environment.

