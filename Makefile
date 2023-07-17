SHELL := /bin/bash
ENV=source .env &&
DB_CONTAINER_NAME := "lnk_db"

build:
	npx tailwindcss -i ./tailwind.css -o src/tailwind.generated.css
	cargo build

dev:
	npx concurrently --names 'tailwind,cargo' \
		'npx tailwindcss -i ./tailwind.css -s src/tailwind.generated.css' \
		'cargo watch'



start-db:
	$(ENV) docker run \
        --name $(DB_CONTAINER_NAME) \
        -e POSTGRES_DATABASE="$$POSTGRES_DB" \
        -e POSTGRES_USER="$$POSTGRES_USER" \
        -e POSTGRES_PASSWORD="$$POSTGRES_PASSWORD" \
        -v $(PWD)/initdb.sql:/docker-entrypoint-initdb.d/initdb.sql \
        -p 5432:5432 \
        -d \
        postgres:15

stop-db:
	docker kill $(DB_CONTAINER_NAME) || true
	docker rm $(DB_CONTAINER_NAME) || true

reset-db: stop-db
	make start-db

watch-db:
	docker logs -f $(DB_CONTAINER_NAME)

shell-db:
	$(ENV) PGPASSWORD=$$POSTGRES_PASSWORD \
		psql -U "$$POSTGRES_USER" -h 0.0.0.0 $$POSTGRES_DB
