#!/bin/bash
# Repeatedly creates and inspects users.

echo "Press [CTRL+C] to stop."
sleep 1

while :
do
	echo ------------------------------------------------------------------------------------------
	UUID=$(curl -s -w '\n' -X POST -d '{"device_id": "Johns_PC6H76GB5G1"}' -H "Content-type: application/json" http://0.0.0.0:8765/api/users)
	echo Created user on server with UUID: "$UUID". Inspecting user...
	./inspect_user.sh "$UUID"
done
