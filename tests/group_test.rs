extern crate schoology;

mod common;
use self::common::get_api;

#[test]
pub fn get_groups() {
    let group_ids = vec!["818498677", "787109203", "834592471"];

    let api = get_api();

    for id in group_ids {
        let group = api.get_group(id).unwrap();
        println!("{:#?}", group);
        assert_eq!(group.id, id);
    }
}

#[test]
pub fn list_groups() {
    let api = get_api();
    let groups = api.get_groups(0, 3).unwrap();
    println!("{:#?}", groups);
    assert!(groups.group.len() > 0);
}
