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
export CURRENT_DIRECTORY

# now we should know necessary files' locations
env_file_path="${CURRENT_DIRECTORY}/.env"
nginx_conf_file_path="${CURRENT_DIRECTORY}/data/nginx/nginx.conf"
nginx_conf_template_file_path="${CURRENT_DIRECTORY}/data/nginx/nginx_template.conf"

# check if .env file exists
if [ ! -f "$env_file_path" ]; then
    echo "${RED}ERROR: $env_file_path not found" 1>&2
    exit 1
fi

# check if template file exists
if [ ! -f $nginx_conf_template_file_path ]; then
    echo "${RED}ERROR: $nginx_conf_template_file_path not found" 1>&2
    exit 1
fi

# read the variables
export $(grep -v '^#' $env_file_path | xargs)

# read the template file and replace needed variables
substituted=$(envsubst '${USER} ${PORT} ${DOMAIN} ${CURRENT_DIRECTORY}' < $nginx_conf_template_file_path)

# write the output to config file
echo "$substituted" > $nginx_conf_file_path

# stop nginx if it is already running
# graceful stop, allows ongoing requests to complete
if pgrep nginx > /dev/null 2>&1; then
    sudo nginx -s quit
    echo "${GREEN}nginx is already running, restarting..."
fi
# run nginx with our fresh config
sudo nginx -c $nginx_conf_file_path
echo "${GREEN}successfully started nginx"