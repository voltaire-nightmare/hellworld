make: src
	cargo build --release
	cp target/release/hellworld ./