ADDR = "http://127.0.0.1:8080"

.PHONY: check launch build

check:
	$(info "Checking the frontend -> wasm website")
	cd Frontend && cargo check
	$(info "Checking the backend -> actix webserver")
	cd Backend && cargo check

launch:
	$(info "Compiling")
	rm -fr Backend/static
	cd Frontend && trunk build && mv dist ../Backend/static 
	wslview "http://127.0.0.1:8080/memepoke"
	cd Backend && cargo run

build:
	cd Frontend && trunk build && mv dist ../Backend/static
	cd Backend && cargo build --release
	$(info "SUCCESS!!!")

