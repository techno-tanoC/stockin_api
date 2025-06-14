#!/bin/sh
set -eu

mkdir -p db
litestream restore -config ./litestream.yaml -if-db-not-exists -if-replica-exists db/database.sqlite3
sqlite3def -f schema.sql --enable-drop db/database.sqlite3
exec litestream replicate -config ./litestream.yaml -exec "./api"
