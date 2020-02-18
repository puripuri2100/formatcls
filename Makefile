build:
	cargo build

test:
	target/debug/formcls -o t/test.satyh -c t/t.json
	satysfi t/t.saty -o t/t.pdf
	target/debug/formcls -d t/default.json