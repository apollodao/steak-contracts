
rename_artifacts:
	mv artifacts/osmosis_steak_hub-aarch64.wasm artifacts/osmosis_steak_hub.wasm && \
	mv artifacts/cw20_steak_hub-aarch64.wasm artifacts/cw20_steak_hub.wasm

apple_m1_prod:
	sh build_apple_m1.sh && make rename_artifacts

fmt:
	cargo fmt && taplo fmt && cargo clippy --fix --allow-dirty

m1_all: apple_m1_prod fmt

schema:
	cd contracts/cw20_hub && cargo schema --target-dir .
	cd contracts/osmosis_hub && cargo schema --target-dir .

ts-types:
	cd scripts && npm ci && npm run generate

coverage:
	cargo outdated
	cargo tarpaulin --verbose --all-features --workspace --timeout 120

docs:
	cargo doc --target-dir docs --color never --no-deps --open --workspace --release
