cargo-test: cargo watch --ignore '*/web/*' -s 'cargo test --all --color=always && cargo +nightly clippy --all --color=always -- -D clippy && touch .trigger'
eslint: while true; do ag -l | grep ^web/ | entr -d yarn run eslint --ext .jsx,.js --color web/src; done

webpack: yarn run webpack -- --color -w
budgetron: cargo watch --no-gitignore -w .trigger -s "cargo run --color=always -- -f data/`\ls data | tail -n1`/*.csv --serve --port $PORT"
