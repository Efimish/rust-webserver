#!/bin/sh

url="https://raw.githubusercontent.com/ua-parser/uap-core/master/regexes.yaml"
destination="./regexes.yaml"

curl -L $url -o $destination