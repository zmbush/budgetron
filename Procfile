cargo-test: cargo watch --ignore '*/web/*' -s 'cargo test --all --color=always && cargo clippy --all --color=always && touch .trigger'
tslint: rg --files | grep ^web/ | entr -d -r yarn tslint -c tslint.json 'web/src/**/*.ts{,x}' --exclude '**/*.scss.d.ts'

webpack: yarn run webpack -- --color -w
budgetron: cargo watch --no-gitignore -w .trigger -s "cargo run --color=always --bin budgetron --release -- -f data/`\ls data | tail -n1`/*.csv --serve --port $PORT"
