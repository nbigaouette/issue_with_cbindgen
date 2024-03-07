# Issue With Bindgen

Problem: C header generated with the `cbindgen` (0.26.0) CLI succeeds while using a build script (`build.rs`) fails.

Generate the header from the CLI:

```sh
❯ make cli
rm -f b/my_header_from_cli.h
cd b && cbindgen --config cbindgen.toml --crate b --output my_header_from_cli.h -vvvv
DEBUG: Parsing crate b
INFO: Take b::free_standing_function.
DEBUG: Parsing crate a
INFO: Take a::TypeA.
DEBUG: Excluding crate cbindgen
DEBUG: Excluding crate env_logger
```

Generate the header from the build script:

```sh
❯ make build
rm -f b/my_header_from_build.h
cargo build
   Compiling a v0.1.0 (/Users/nbigaouette/projects/cbindgen_issue/a)
   Compiling b v0.1.0 (/Users/nbigaouette/projects/cbindgen_issue/b)
    Finished dev [unoptimized + debuginfo] target(s) in 2.49s
```

Check the difference:

```sh
❯ make diff
diff -Naur b/my_header_from_cli.h b/my_header_from_build.h
--- b/my_header_from_cli.h	2024-03-07 16:50:55
+++ b/my_header_from_build.h	2024-03-07 16:51:09
@@ -5,9 +5,4 @@
 #include <stdint.h>
 #include <stdlib.h>

-typedef struct TypeA {
-  uint32_t aa;
-  uint32_t ab;
-} TypeA;
-
-uint32_t free_standing_function(const struct TypeA *grand_parent);
+uint32_t free_standing_function(const TypeA *grand_parent);
make: *** [diff] Error 1
```

The two headers should have been the same.

Printing the logs from the build script shows that it couldn't find the type:

```sh
❯ make logs
cat target/debug/build/b-*/stderr
[b/build.rs:10] &manifest_dir = "/Users/nbigaouette/cbindgen_issue/b"
[b/build.rs:10] &manifest_dir = "/Users/nbigaouette/cbindgen_issue/b"
[2024-03-07T22:18:44Z INFO  cbindgen::bindgen::parser] Take ffi::free_standing_function.
[2024-03-07T22:18:44Z WARN  cbindgen::bindgen::ir::ty] Can't find TypeA. This usually means that this type was incompatible or not found.
```
