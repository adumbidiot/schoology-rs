use crate::{
    client::Request,
    SchoologyRealm,
    SchoologyRealmList,
};
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::HashMap,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Group {
    pub id: String,
    pub title: String,
    pub description: String,
    pub website: String,
    pub access_code: Option<String>,
    pub category: String,
    pub group_code: String,
    pub privacy_level: String,
    pub picture_url: String,
    pub school_id: String,
    pub building_id: String,
    pub admin: Option<u32>,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

impl SchoologyRealm for Group {
    fn get_url() -> &'static str {
        "groups"
    }

    fn get(id: &str) -> Request {
        Request {
            url: format!("https://api.schoology.com/v1/{}/{}", Self::get_url(), id).into(),
        }
    }

    fn get_id(&self) -> Cow<str> {
        (&self.id).into()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub uid: String,
    pub id: u64,
    pub name_title: String,
    pub name_title_show: u32,
    pub name_first: String,
    pub name_first_preferred: String,
    pub use_preferred_first_name: String,
    pub name_middle: Option<String>,
    pub name_middle_show: u32,
    pub name_last: String,
    pub picture_url: String,
    pub gender: Option<String>,
    pub position: Option<String>,
    pub child_uids: Option<String>,
    pub building_id: Option<u32>,
    pub language: Option<String>,
    pub name_display: Option<String>,
    pub primary_email: Option<String>,
    pub school_id: Option<u64>,
    pub send_message: Option<u32>,
    pub synced: Option<u32>,

    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

impl SchoologyRealm for User {
    fn get_url() -> &'static str {
        "users"
    }

    fn get(id: &str) -> Request {
        Request {
            url: format!(
                "https://api.schoology.com/v1/{}/{}?extended=true",
                Self::get_url(),
                id
            )
            .into(),
        }
    }

    fn get_id(&self) -> Cow<str> {
        self.id.to_string().into()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserList {
    pub user: Vec<User>,
    pub total: String,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

impl SchoologyRealmList for UserList {
    fn get_many(start: usize, length: usize) -> Request {
        Request {
            url: format!(
                "https://api.schoology.com/v1/users?start={}&limit={}",
                start, length
            )
            .into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupList {
    pub group: Vec<Group>,
    pub total: u32,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

impl SchoologyRealmList for GroupList {
    fn get_many(start: usize, length: usize) -> Request {
        Request {
            url: format!(
                "https://api.schoology.com/v1/groups?start={}&limit={}",
                start, length
            )
            .into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct School {
    id: String,
    country: String,
    address1: String,
    address2: String,
    fax: String,
    city: String,
    postal_code: String,
    building_code: String,
    website: String,
    title: String,
    picture_url: String,
    state: String,
    phone: String,

    #[serde(flatten)]
    unknown: HashMap<String, Value>,
}

impl SchoologyRealm for School {
    fn get_url() -> &'static str {
        "schools"
    }

    fn get(id: &str) -> Request {
        Request {
            url: format!(
                "https://api.schoology.com/v1/{}/{}?extended=true",
                Self::get_url(),
                id
            )
            .into(),
        }
    }

    fn get_id(&self) -> Cow<str> {
        self.id.to_string().into()
    }
}
