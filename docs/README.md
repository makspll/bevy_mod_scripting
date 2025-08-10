# BMS Docs

## How To

### Run Tests

To run the tests within the book do this:
``` sh
cd docs;
mdbook test -L ../target/debug/deps
```

To run tests on a particular chapter, say the "Managing Scripts" chapter, do this:
``` sh
cd docs;
mdbook test -L ../target/debug/deps -c "Managing Scripts"
```

## Troubleshooting

If there are errors that complain about different rustc versions, consider reinstalling "mdbook" using whatever toolchain BMS is built with.

``` sh
cargo uninstall mdbook
cargo +stable install mdbook
```

