#!/bin/sh

# colored output
RED='\033[1;31m'
GREEN='\033[1;32m'

# get the current directory
CURRENT_DIRECTORY=$(pwd)

# if we are inside /scipts directory, move one directory up
if [ "$(basename $CURRENT_DIRECTORY)" = "scripts" ]; then
    CURRENT_DIRECTORY=${CURRENT_DIRECTORY%/scripts}
fi

# now we should know .env file location
env_file_path="${CURRENT_DIRECTORY}/.env"

# check if .env file exists
if [ ! -f "$env_file_path" ]; then
    echo "${RED}ERROR: $env_file_path not found" 1>&2
    exit 1
fi

# read .env file
source $env_file_path

# get certificates
docker-compose run --rm \
    certbot certonly \
    --webroot \
    --webroot-path /var/www/certbot/ \
    -d $DOMAIN \
    --email $EMAIL \
    --agree-tos