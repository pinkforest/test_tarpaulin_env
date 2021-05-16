test_tarpaulin_env
==================

- Issue tracker: https://github.com/xd009642/tarpaulin/issues/761
- Install: cargo install tarpaulin and clone this repo
- Doc adjustment: https://github.com/xd009642/tarpaulin/pull/760
- Fix code PR: maybe?

Expectation is that cargo tarpaulin makes env CARGO_MANIFEST_PATH available under test.

## Limitations

- Run all the cargo commands from the path where manifest is.
- Adjust the absolute manifest path manually (painful it is)

## cargo test

```bash
env cargo test
```

## tarpaulin Tests Without --manifest-path

```bash
env cargo tarpaulin --run-types Tests -v
```

## tarpaulin Doctest Without --manifest-path

```bash
env cargo tarpaulin --run-types Doctests -v
```

## tarpaulin Doctest With relative --manifest-path

```bash
env cargo tarpaulin --manifest-path=./Cargo.toml --run-types Doctests -v
```

## tarpaulin Doctest With absolute --manifest-path

```bash
env cargo tarpaulin --manifest-path=/home/foobar/testing-rust/test_tarpaulin_env/Cargo.toml --run-types Doctests -v
```

## tarpaulin Examples Without --manifest-path

```bash
env cargo tarpaulin --run-types Examples -v
```

## tarpaulin Alltargets Without --manifest-path

```bash
env cargo tarpaulin --run-types AllTargets -v
```


