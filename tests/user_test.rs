extern crate schoology;

mod common;
use self::common::get_api;

#[test]
fn get_user() {
    let user_ids = vec!["6369219", "5627013"];

    let api = get_api();

    for id in user_ids {
        let user = api.get_user(id).unwrap();
        println!("{:#?}", user);
        assert_eq!(user.uid, id);
    }
}

#[test]
#[should_panic] // TODO: Match Error
fn unauthorized_access() {
    let id = "636922"; // 403 for some reason: "636922"
    let api = get_api();
    let user = api.get_user(id).unwrap();
    println!("{:#?}", user);
}

#[test]
pub fn list_users() {
    let api = get_api();
    let users = api.get_users(0, 3).unwrap();
    println!("{:#?}", users);
    assert!(users.user.len() > 0);
}
