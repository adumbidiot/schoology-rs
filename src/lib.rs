extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rand;

pub mod client;
pub mod realms;

pub use crate::client::Client;
use crate::{
    client::Request,
    realms::{
        Group,
        GroupList,
        User,
        UserList,
    },
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

pub type SchoologyResult<T> = Result<T, SchoologyError>;

#[derive(Debug)]
pub enum SchoologyError {
    Network,     // Add speceficity
    InvalidBody, // Add specifics? Lossy unicode conversion instead?
    InvalidStatusCode(u16),
    InvalidJson(String), // TODO: More presice Error? Return Serde Error Directly?
    UnauthorizedRequest, /* Use Invalid Status instead? This is more specific and probably more common? */
}

pub trait SchoologyRealm: DeserializeOwned {
    fn get_url() -> &'static str;
    fn get(id: &str) -> Request;
    fn get_id(&self) -> &str;
}

pub trait SchoologyRealmList: DeserializeOwned {
    fn get_many(start: usize, end: usize) -> Request;
}

pub trait SchoologyRealmObject: DeserializeOwned {
    type Parent;
    fn get(realm_id: &Self::Parent, id: &str) -> Request;
}

pub trait SchoologyRealmObjectList: DeserializeOwned {
    type Parent;
    fn get_many(realm: &Self::Parent, start: usize, length: usize) -> Request;
}

#[derive(Deserialize, Debug)]
pub struct Update {
    pub id: u64,
    pub body: String,
    pub uid: u32,
    pub created: u32,
    pub likes: u32,
    pub user_like_action: bool,
    pub realm: String,
    pub group_id: u32,
    pub num_comments: u32,
    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

impl SchoologyRealmObject for Update {
    type Parent = Group;
    fn get(realm: &Self::Parent, id: &str) -> Request {
        Request {
            url: format!(
                "https://api.schoology.com/v1/{}/{}/updates/{}",
                Self::Parent::get_url(),
                realm.get_id(),
                id
            )
            .into(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateList {
    pub update: Vec<Update>,
}

impl SchoologyRealmObjectList for UpdateList {
    type Parent = Group;
    fn get_many(realm: &Self::Parent, start: usize, end: usize) -> Request {
        Request {
            url: format!(
                "https://api.schoology.com/v1/{}/{}/updates?start={}&limit={}",
                Self::Parent::get_url(),
                realm.get_id(),
                start,
                end
            )
            .into(),
        }
    }
}

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new(token: String, secret: String) -> Self {
        Api {
            client: Client::new(token, secret),
        }
    }

    pub fn get_group(&self, id: &str) -> ApiResult<Group> {
        self.client.get_realm(id).map_err(|_| ApiError::Fetch)
    }

    pub fn get_groups(&self, start: usize, limit: usize) -> ApiResult<GroupList> {
        self.client
            .get_realms(start, limit)
            .map_err(ApiError::SchoologyError)
    }

    pub fn get_user(&self, id: &str) -> ApiResult<User> {
        self.client.get_realm(id).map_err(ApiError::SchoologyError)
    }

    pub fn get_users(&self, start: usize, limit: usize) -> ApiResult<UserList> {
        self.client
            .get_realms(start, limit)
            .map_err(ApiError::SchoologyError)
    }

    pub fn get_group_update(&self, group_id: &str, id: &str) -> ApiResult<Update> {
        let mut group = Group::default(); // TODO: Unpopulated flag, access vars through functions, update values function
        group.id = group_id.to_string(); // TODO: Cow?
        self.client
            .get_realm_object(&group, id)
            .map_err(ApiError::SchoologyError)
    }

    pub fn get_group_updates(
        &self,
        id: &str,
        start: usize,
        length: usize,
    ) -> ApiResult<UpdateList> {
        let mut group = Group::default();
        group.id = id.to_string();
        self.client
            .get_realm_objects(&group, start, length)
            .map_err(ApiError::SchoologyError)
    }
}

type ApiResult<T> = Result<T, ApiError>;

struct ApiRequest {
    pub url: String,
}

impl From<ApiRequest> for Request {
    fn from(req: ApiRequest) -> Request {
        Request {
            url: req.url.into(),
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    Fetch,
    Parse,
    JsonParse(serde_json::error::Error),
    StatusCode(hyper::status::StatusCode, String),
    SchoologyError(SchoologyError),
}
