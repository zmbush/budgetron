test: cargo watch -x test --ignore '*/web/*' -s 'touch .trigger'
server: cargo watch --no-gitignore -w .trigger -s "cargo run --color=always -- -f data/`\ls data | tail -n1`/*.csv --serve"
webpack: cd web; yarn run webpack -- --color -w
