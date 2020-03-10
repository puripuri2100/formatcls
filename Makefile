.PHONY: build install test clean demo

build: src
	cargo build

install: src
	cargo install --path .

test: t
	target/debug/formatcls -o t/test.satyh -c t/t.json
	satysfi t/t.saty -o t/t.pdf
	target/debug/formatcls -d t/default.json
	cargo test

clean:
	@rm -rf target t

demo: demo/demo.json demo/demo.saty
	formatcls -c demo/demo.json -o demo/demo.satyh
	satysfi demo/demo.saty -o demo/demo.pdf
