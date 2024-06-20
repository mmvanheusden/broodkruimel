#!/usr/bin/env bash

# Gets response code
response=$(curl --write-out '%{http_code}' --silent --output /dev/null 0.0.0.0:8765/ping)

if [ "$response" == "200" ]
then
  echo Pong!
else
  echo ERROR: received response code "$response"
fi