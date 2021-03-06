RUST_CFG=

compile:
	rustc --opt-level=3 ./src/lib.rs

install:
	cargo-lite install

ctags:
	ctags --recurse --options=ctags.rust --languages=Rust

docs:
	rm -rf doc
	rustdoc -L . --test src/lib.rs
	rustdoc src/lib.rs
	# WTF is rustdoc doing?
	chmod 755 doc
	in-dir doc fix-perms
	rscp ./doc/* gopher:~/www/burntsushi.net/rustdoc/

test: test-runner
	RUST_TEST_TASKS=1 RUST_LOG=quickcheck,csv ./test-runner

test-runner: src/lib.rs src/test.rs src/bench.rs
	rustc -L . --test src/lib.rs -o test-runner

test-examples:
	(cd ./examples && ./test)

bench: bench-runner
	RUST_TEST_TASKS=1 RUST_LOG=quickcheck,csv ./bench-runner --bench

bench-prof: bench-runner
	RUST_TEST_TASKS=1 RUST_LOG=quickcheck,csv valgrind --tool=callgrind ./bench-runner --bench

bench-runner: src/lib.rs src/test.rs src/bench.rs
	rustc -g -L . --opt-level=3 -Z lto --test $(RUST_CFG) src/lib.rs -o bench-runner

test-clean:
	rm -rf ./test-runner ./bench-runner

clean: test-clean
	rm -f *.rlib *.so

push:
	git push origin master
	git push github master

