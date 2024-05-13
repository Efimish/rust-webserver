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

# now we should know necessary files' locations
env_example_file_path="${CURRENT_DIRECTORY}/.env.example"
env_file_path="${CURRENT_DIRECTORY}/.env"

# check if .env.example file exists
if [ ! -f "$env_example_file_path" ]; then
    echo "${RED}ERROR: $env_example_file_path not found" 1>&2
    exit 1
fi

# copy (to the EOF)
cat $env_example_file_path >> $env_file_path
echo "${GREEN}successfully copied example to $env_file_path"