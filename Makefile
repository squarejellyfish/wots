install:
	touch ~/.wots-global-ignore
	echo ".gitignore\n.git/" > ~/.wots-global-ignore
	cargo build --release
	./target/release/wots ./target/release/wots -t /usr/local/bin/ -f

release:
	cargo build --release

manual:
	touch ~/.wots-global-ignore
	echo ".gitignore\n.git/" > ~/.wots-global-ignore
	cargo build --release
