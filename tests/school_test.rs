extern crate schoology;

mod common;

use schoology::{
    client::Request,
    realms::School,
};

use self::common::get_client;

#[test]
pub fn get_school() {
    let school_ids = vec!["47301321"];

    let client = get_client();

    for id in school_ids {
        let school: School = client.get_realm(id).unwrap();
        println!("{:#?}", school);
    }

    // let data = client.send_request(Request {
    // url: format!("https://api.schoology.com/v1/users/82444544/documents").into(),
    // }).unwrap();
    // println!("{}", data);
}
