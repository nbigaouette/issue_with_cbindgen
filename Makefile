export RUST_LOG := trace

cli:
	rm -f b/my_header_from_cli.h
	cd b && cbindgen --config cbindgen.toml --crate b --output my_header_from_cli.h -vvvv

build:
	rm -f b/my_header_from_build.h
	cargo build

diff:
	diff -Naur b/my_header_from_cli.h b/my_header_from_build.h

logs:
	cat target/debug/build/b-*/stderr

clean:
	rm -f b/my_header_from_cli.h
	rm -f b/my_header_from_build.h
