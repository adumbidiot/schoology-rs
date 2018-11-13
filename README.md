# Schoology-rs

A basic sync api for schoology.

## Getting Started


### Installing

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
schoology = { git = "https://github.com/adumbidiot/schoology-rs" }
```

and to the top of your `main.rs`:

```rust,ignore
extern crate schoology;
```

## Running the tests
Create two files: "token.txt" and "secret.txt" on the top level. Put in your token and secret, then run "cargo test".


## Authors

**adumbidiot** 

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details