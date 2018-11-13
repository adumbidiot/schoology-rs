extern crate schoology;

mod common;
use self::common::get_api;

#[test]
pub fn get_group_updates(){
	let update_ids = vec![
			"1904820846", 
			"1898699704",
			"1895989525"
		];
	
	let api = get_api();
	
	for id in update_ids {
		let update = api.get_group_update("818498677", id).unwrap();
		assert_eq!(update.id.to_string(), id);
	}
}

#[test]
pub fn list_group_updates(){
	let api = get_api();
	let updates = api.get_group_updates("818498677", 0, 3).unwrap();
	assert!(updates.update.len() > 0);
}

