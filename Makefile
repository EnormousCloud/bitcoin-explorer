.PHONY: indexer

btc-start:
	docker run -d --name bitcoind-node \
	    -p 8332:8332 \
		-p 8333:8333 \
		-v bitcoind-data:/bitcoin/.bitcoin \
		kylemanna/bitcoind

db-start:
    # start postgres instance (for local testing only)
	docker run -d \
		--net host \
		--name postgres \
		-e POSTGRES_USER=postgres \
		-e POSTGRES_PASSWORD=password \
		-e POSTGRES_DB=btcexplorer \
		-v btcexplorer-data:/var/lib/postgresql/data \
		postgres

db-stop:	
	docker container rm -f postgres

db-init:
	cat ./db/btcexplorer.sql | docker exec -i postgres psql -U postgres -d btcexplorer

db-shell:
	docker exec -it postgres psql -U postgres -d btcexplorer

indexer:
	cd indexer && RUST_BACKTRACE=1 cargo run && cd -