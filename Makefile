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

deploy-sandbox:
	docker build -t us-west1-docker.pkg.dev/sandbox-310216/stockin/app:latest -f docker/app/Dockerfile --platform linux/x86_64 --push .
	gcloud run deploy stockin --image=us-west1-docker.pkg.dev/sandbox-310216/stockin/app:latest --region=us-west1

deploy-main:
	docker build -t us-west1-docker.pkg.dev/main-282614/stockin/app:latest -f docker/app/Dockerfile --platform linux/x86_64 --push .
	gcloud run deploy stockin --image=us-west1-docker.pkg.dev/main-282614/stockin/app:latest --region=us-west1
