DATABASE ?= db/database.sqlite3

replicate:
	litestream replicate -config ./litestream.yaml

run-replicate:
	mkdir -p ./db
	litestream restore -config ./litestream.yaml -if-db-not-exists -if-replica-exists $(DATABASE)
	litestream replicate -config ./litestream.yaml -exec "cargo run"

seed:
	cargo run --bin seed

sqlite:
	sqlite3 $(DATABASE)

create:
	sqlite3 $(DATABASE) ""

drop:
	rm -rf db
	mkdir db

apply:
	sqlite3def -f schema.sql $(DATABASE)

setup: drop create apply
reset: setup seed
