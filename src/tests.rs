use super::*;

fn load_file(path: &str) -> String{
	let data = std::fs::read(path).expect("Error opening file");
	return String::from_utf8(data).expect("Error converting to string");
}

#[test]
fn get_user_6369229() {
	let api = Api::new(load_file("token.txt"), load_file("secret.txt"));
	let user = api.get_user("6369229").unwrap();
	assert_eq!(user.uid, "6369229");
}

#[test]
fn get_user_5627013() {
	let api = Api::new(load_file("token.txt"), load_file("secret.txt"));
	let user = api.get_user("5627013").unwrap();
	assert_eq!(user.uid, "5627013");
}

#[test]
fn get_user_6369219() {
	let api = Api::new(load_file("token.txt"), load_file("secret.txt"));
	let user = api.get_user("6369219").unwrap();
	assert_eq!(user.uid, "6369219");
}
