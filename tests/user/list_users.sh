#!/usr/bin/env bash

# Get a JSON list of all users on the server

curl -w '\n' http://127.0.0.1:8080/list_users