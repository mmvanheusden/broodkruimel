#!/usr/bin/env bash

# Get user info

if [ "$#" -eq 1 ]; then
    UUID=$1
    printf "Using provided UUID: $UUID\n\n"
else
    UUID="gr4hroijreoigjmokre"
    printf "Using invalid UUID: $UUID\n\n"
fi

# Get user info
curl -w '\n--------------------\nTotal: %{time_total}s\n' http://0.0.0.0:8765/api/users/"$UUID"
