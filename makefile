setup:
	docker volume create pongdata 
	cargo install diesel_cli --no-default-features --features postgres
	cargo install cargo-watch

db:
	docker-compose up -d db
	
dev:
	cargo watch -x run