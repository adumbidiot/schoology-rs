extern crate schoology;
use schoology::Api;

fn main() {
    let api = Api::new(String::from("Token"), String::from("Secret")); // Replace with your own token and secret;
    let groups = api.get_groups(4, 10).expect("Error getting groups"); // Get groups from 4 to 14. May return shorter if they don't exist.
    println!("{:#?}", groups); // Returned types implement debug.
}
