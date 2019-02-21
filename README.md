# schoology-rs

A basic sync (incomplete) api for schoology. Uses hyper v10 (the old one).

### Installing

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
schoology = { git = "https://github.com/adumbidiot/schoology-rs" }
```

and to the top of your `main.rs`:

```rust
extern crate schoology;
```

## Getting Started

```rust
extern crate schoology;
use schoology::Api;

fn main(){
	let api = Api::new(String::from("Token"), String::from("Secret")); //Replace with your own token and secret;
	let groups = api.get_groups(4, 10).expect("Error getting groups"); //Get groups from 4 to 14. 
	//May return shorter if they don't exist.
	println!("{:#?}", groups); //Returned types implement debug.
}
```


## Testing
Create two files: "token.txt" and "secret.txt" on the top level project directory. Put in your token and secret in the appropriate files, then run: 
```bash
cargo test
```

## Contributing
I literally have no standards so anything you contribute will be an improvement. Just open a pull request.

## Authors
adumbidiot (Nathaniel Daniel)

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details