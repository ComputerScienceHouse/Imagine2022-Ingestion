#!/bin/bash
mongo_uname=imaginerit
mongo_pw=ligma123
podman run --rm -d -e MONGO_INITDB_ROOT_USERNAME=$mongo_uname -e MONGO_INITDB_ROOT_PASSWORD=$mongo_pw -v mongo_data:/data/db -p 27017:27017 --name mongo mongo

# MONGO_URL=mongodb://imaginerit:ligma123@<ip addr>
