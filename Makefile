.PHONY: build install test clean demo

build: src
	cargo fmt
	cargo build
	find . -name "*.rs" -print0 | wc --files0-from=-

install: src
	cargo install --path .

test: t
	cargo run -- -o t/test.satyh -c t/t.json
	satysfi --debug-show-bbox --debug-show-space --debug-show-block-bbox --debug-show-block-space t/t.saty -o t/t.pdf
	cargo run -- -d t/default.json
	cargo run -- -o t/default.satyh -c t/default.json
	satysfi t/default.saty -o t/default.pdf
	cargo test

clean:
	@rm -rf target t

demo: demo/demo.json demo/demo.saty
	formatcls -c demo/demo.json -o demo/demo.satyh
	satysfi demo/demo.saty -o demo/demo.pdf
