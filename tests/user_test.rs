extern crate schoology;

mod common;
use self::common::get_api;

#[test]
fn get_user() {
	let user_ids = vec![
			"6369219", 
			"5627013"
		];// 403 for some reason: "636922"
	
	let api = get_api();
	
	for id in user_ids {
		let user = api.get_user(id).unwrap();
		assert_eq!(user.uid, id);
	}
}

#[test]
pub fn list_users(){
	let api = get_api();
	let users = api.get_users(0, 3).unwrap();
	assert!(users.user.len() > 0);
}