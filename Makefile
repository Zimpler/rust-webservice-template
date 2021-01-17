.PHONY: dev schema

docker-compose.yml: dc.dhall service.dhall
	dhall-to-yaml --file $< --output $@
	zdc-fix-svc-tags

.env: docker-compose.yml .env.in
	zdc-dot-env

dev: .env

schema: SHELL = bash
schema:  ## Runs pg_dump to produce schema.sql
	docker-compose up -d postgres
	( source .env; \
	  docker-compose exec postgres pg_dump --schema-only --no-privileges --no-owner \
	    --dbname="postgresql://localhost/$${POSTGRES_DATABASE}?user=$${POSTGRES_USER}&password=$${POSTGRES_PASSWORD}" | \
	    dos2unix > schema.sql \
	)
