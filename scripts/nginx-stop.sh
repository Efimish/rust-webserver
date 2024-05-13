#!/bin/sh

# colored output
RED='\033[1;31m'
GREEN='\033[1;32m'

# graceful stop, allows ongoing requests to complete
if pgrep nginx > /dev/null 2>&1; then
    sudo nginx -s quit
    echo "${GREEN}successfully stopped nginx"
    exit 1
fi
echo "${RED}nginx is already stopped" 1>&2