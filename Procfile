cargo-test: cargo watch --ignore '*/web/*' -s 'cargo test --all --color=always && cargo clippy --all --color=always && touch .trigger'
eslint: rg --files | grep ^web/ | entr -d -r yarn run eslint --ext .jsx,.js --color web/src

webpack: yarn run webpack -- --color -w
budgetron: cargo watch --no-gitignore -w .trigger -s "cargo run --color=always -- -f data/`\ls data | tail -n1`/*.csv --serve --port $PORT"
