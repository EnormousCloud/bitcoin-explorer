db-start:
    # start postgres instance (for local testing only)
	docker run -d \
		--net host \
		--name postgres \
		-e POSTGRES_USER=postgres \
		-e POSTGRES_PASSWORD=password \
		-e POSTGRES_DB=btcexplorer \
		postgres

db-stop:	
	docker container rm -f postgres

db-init:
	cat ./db/tables.sql | docker exec -i postgres psql -U postgres -d btcexplorer

db-shell:
	docker exec -it postgres psql -U postgres -d btcexplorer
