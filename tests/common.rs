use schoology::Api;

fn load_file(path: &str) -> String{
	let data = std::fs::read(path).expect("Error opening file");
	return String::from_utf8(data).expect("Error converting to string");
}

pub fn get_api() -> Api{
	return Api::new(load_file("token.txt"), load_file("secret.txt"));
}