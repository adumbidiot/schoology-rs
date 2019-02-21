use schoology::{
    Api,
    Client,
};

fn load_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Error opening file")
}

pub fn get_api() -> Api {
    Api::new(load_file("token.txt"), load_file("secret.txt"))
}

pub fn get_client() -> Client {
    Client::new(load_file("token.txt"), load_file("secret.txt"))
}
