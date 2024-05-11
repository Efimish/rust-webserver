#!/bin/sh

source .env

docker-compose run --rm \
    certbot certonly \
    --webroot \
    --webroot-path /var/www/certbot/ \
    --dry-run \
    -d $DOMAIN \
    --email $EMAIL \
    --agree-tos