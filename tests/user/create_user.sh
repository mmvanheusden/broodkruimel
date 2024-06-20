#!/usr/bin/env bash

# Create a new user on the server using the "/create_user" endpoint.
#curl -w '\n' -X POST -d '{"device_id": "John'\''s_PC6H76GB5G1"}' -H "Content-type: application/json" http://0.0.0.0:8765/api/users

curl -w '\n' -X POST -d '{"device_id": "Johns_PC6H76GB5G1"}' -H "Content-type: application/json" http://0.0.0.0:8765/api/users