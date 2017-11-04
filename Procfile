test: cargo watch --ignore '*/web/*' -s 'cargo test --all --color=always && cargo +nightly clippy --all --color=always -- -D clippy && touch .trigger'
server: cargo watch --no-gitignore -w .trigger -s "cargo run --color=always -- -f data/`\ls data | tail -n1`/*.csv --serve"
webpack: yarn run webpack -- --color -w
