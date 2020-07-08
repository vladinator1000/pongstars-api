setup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup toolchain install nightly
	rustup default nightly 
	docker volume create pongdata 
	cargo install diesel_cli --no-default-features --features postgres
	cargo install diesel_cli_ext
	cargo install cargo-watch

db:
	docker-compose up -d db
	diesel migration run
	
dev:
	cargo watch -x run
	