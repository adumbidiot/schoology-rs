extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rand;

pub mod types;
#[cfg(test)]
mod tests;

use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Read;

use rand::Rng;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper::header::Authorization;
use hyper::header::Headers;

use hyper_native_tls::NativeTlsClient;

pub use self::types::*;

fn get_nonce() -> String{
	return rand::thread_rng()
    .gen_ascii_chars()
    .take(11)
    .collect();
}

fn get_auth_header(token: &str, secret: &str) -> String{
	let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
	let nonce = get_nonce();
	return format!(r#"OAuth realm="Schoology API", oauth_consumer_key="{}", oauth_token="", oauth_nonce="{}", oauth_timestamp="{}", oauth_signature_method="PLAINTEXT", oauth_version="1.0", oauth_signature="{}%26""#, token, nonce, time, secret);
}

pub struct Api{
	token: String,
	secret: String,
	client: Client,
}

impl Api{
	pub fn new(token: String, secret: String) -> Self{
		let ssl = NativeTlsClient::new().unwrap();
		let connector = HttpsConnector::new(ssl);
		let a = Api {
			token: token, 
			secret: secret,
			client: Client::with_connector(connector),
		};
		return a;
		
	}
	
	pub fn send_request(&self, req: ApiRequest) -> ApiResult<String>{
		let mut headers = Headers::new();
		headers.set(Authorization(get_auth_header(&self.token, &self.secret)));
		
		let mut res = self.client
			.get(&req.url)
			.headers(headers)
			.send()
			.map_err(|_|{
				return ApiError::Fetch;
			})?;
			
		if res.status != hyper::status::StatusCode::Ok{
			return Err(ApiError::Fetch);
		}
		//println!("{:#?}", res.headers);
		
		let mut body = String::new();
		res.read_to_string(&mut body).map_err(|_|{
			return ApiError::Parse;
		})?;
		println!("{}", body);
		return Ok(body);
	}
	
	pub fn get_group(&self, id: &str) -> ApiResult<Group>{
		let req = ApiRequest{
			url: format!("https://api.schoology.com/v1/groups/{}", id),
		};
		
		let group: Group = serde_json::from_str(&self.send_request(req)?)
			.map_err(|_|{
				return ApiError::Parse
			})?;
		return Ok(group);
	}
	
	pub fn get_groups(&self, start: u32, limit: u32) -> ApiResult<GroupList>{
		let req = ApiRequest{
			url: format!("https://api.schoology.com/v1/groups?start={}&limit={}", start, limit),
		};
		
		let groups: GroupList = serde_json::from_str(&self.send_request(req)?)
			.map_err(|_|{
				return ApiError::Parse
			})?;
		return Ok(groups);
	}
	
	pub fn get_user(&self, id: &str) -> ApiResult<User>{
		let req = ApiRequest{
			url: format!("https://api.schoology.com/v1/users/{}", id),
		};
		let groups: User = serde_json::from_str(&self.send_request(req)?)
			.map_err(|err|{
				return ApiError::JsonParse(err);
			})?;
		return Ok(groups);
	}
	
	pub fn get_users(&self, start: usize, limit: usize) -> ApiResult<UserList>{
		let req = ApiRequest{
			url: format!("https://api.schoology.com/v1/users?start={}&limit={}", start, limit),
		};
		let groups: UserList = serde_json::from_str(&self.send_request(req)?)
			.map_err(|err|{
				return ApiError::JsonParse(err);
			})?;
		return Ok(groups);
	}
	
	pub fn get_group_update(&self, group_id: &str, id: &str) -> ApiResult<Update>{
		let req = ApiRequest{
			url: format!("https://api.schoology.com/v1/group/{}/updates/{}?start={}&limit={}", group_id, id, 0, 3),
		};
		let data = self.send_request(req)?;
		println!("{}", &data);
		let groups: Update = serde_json::from_str(&data)
			.map_err(|err|{
				return ApiError::JsonParse(err);
			})?;
		return Ok(groups);
	}
	
	pub fn get_group_updates(&self, id: &str) -> ApiResult<UpdateList>{
		let req = ApiRequest{
			url: format!("https://api.schoology.com/v1/group/{}/updates?start={}&limit={}", id, 0, 3),
		};

		let groups: UpdateList = serde_json::from_str(&self.send_request(req)?)
			.map_err(|err|{
				return ApiError::JsonParse(err);
			})?;
		return Ok(groups);
	}
}

type ApiResult<T> = Result<T, ApiError>;

pub struct ApiRequest {
	pub url: String
}

#[derive(Debug)]
pub enum ApiError{
	Fetch,
	Parse,
	JsonParse(serde_json::error::Error),
}