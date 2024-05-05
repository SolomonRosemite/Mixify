generate-proto-and-check:
	cargo check

server:
	cargo run --bin server
client:
	cargo run --bin mixify

preview:
	@file=$$(find src_snapshots -type f | fzf --preview 'cat -n --color=always {}') && \
	dot -Tpng "$$file" > target/output.png && feh target/output.png || echo "No file selected."

preview-test-10:
	dot -Tpng src_snapshots/10/* > target/output.png && feh target/output.png
