#!/usr/bin/env bash

# Create a new user on the server using the "/create_user" endpoint.
curl -X POST -d '{"device_id": "Johns_PC6H76GB5G1"}' -H "Content-type: application/json" http://0.0.0.0:8080/create_user