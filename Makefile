DATABASE ?= db/database.sqlite3

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
