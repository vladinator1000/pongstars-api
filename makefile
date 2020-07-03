setup:
	docker volume create pongdata 
	cargo install systemfd cargo-watch

db: setup
	docker-compose up -d db
	
dev:
	systemfd --no-pid -s http::3000 -- cargo watch -x run