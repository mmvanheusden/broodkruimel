#!/usr/bin/env bash
# shellcheck disable=SC2059

# Usage: ./push_location.sh <optional: uuid>
# If no uuid is provided, a new user will be created and the uuid will be returned

if [ "$#" -eq 1 ]; then
    UUID=$1
    printf "Using provided UUID: $UUID\n"
else
    UUID=$(curl -X POST -d '{"device_id": "Johns_PC6H76GB5G1"}' -H "Content-type: application/json" http://0.0.0.0:8080/create_user)
    printf "Created new UUID on server: $UUID\n"
fi

# Generate a random latitude and longitude
LATITUDE=$((RANDOM%181-90))
LONGITUDE=$((RANDOM%361-180))

# Push the location to the server
curl -X POST -d "{\"uuid\": \"$UUID\", \"latitude\": $LATITUDE, \"longitude\": $LONGITUDE}" -H "Content-type: application/json" http://0.0.0.0:8080/push_location