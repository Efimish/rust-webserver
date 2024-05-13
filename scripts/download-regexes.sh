#!/bin/sh

# get the current directory
CURRENT_DIRECTORY=$(pwd)

# if we are inside /scipts directory, move one directory up
if [ "$(basename $CURRENT_DIRECTORY)" = "scripts" ]; then
    CURRENT_DIRECTORY=${CURRENT_DIRECTORY%/scripts}
fi

# now we should know regexes file location
regexes_file_path="${CURRENT_DIRECTORY}/regexes.yaml"

url="https://raw.githubusercontent.com/ua-parser/uap-core/master/regexes.yaml"

curl -L $url -o $regexes_file_path