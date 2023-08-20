build:
    docker build -t ghcr.io/kolatra/honeypot .

push:
    docker push ghcr.io/kolatra/honeypot

db-init:
    diesel setup
