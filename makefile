setup_min_reqs:
	rustup toolchain install nightly
	rustup component add rust-src --toolchain nightly

build_min_mac_m:
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=aarch64-apple-darwin --release