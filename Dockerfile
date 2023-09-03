FROM rust:1.71
WORKDIR /code
COPY . /code/app/
WORKDIR /code/app
RUN cargo install --path .
RUN cargo install sqlx-cli