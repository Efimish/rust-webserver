#!/bin/sh

# get the current directory
CURRENT_DIRECTORY=$(pwd)

# if we are inside /scipts directory, move one directory up
if [ "$(basename $CURRENT_DIRECTORY)" = "scripts" ]; then
    CURRENT_DIRECTORY=${CURRENT_DIRECTORY%/scripts}
fi

# now we should know necessary files' locations
docker_compose_file_path="${CURRENT_DIRECTORY}/docker-compose.yaml"
docker_compose_prod_file_path="${CURRENT_DIRECTORY}/docker-compose.prod.yaml"

# start the api
docker-compose -f $docker_compose_file_path down
docker-compose -f $docker_compose_prod_file_path down
docker-compose -f $docker_compose_file_path build
docker-compose -f $docker_compose_file_path up -d