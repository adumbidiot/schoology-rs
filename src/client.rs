use crate::{
    SchoologyError,
    SchoologyRealm,
    SchoologyRealmList,
    SchoologyRealmObject,
    SchoologyRealmObjectList,
    SchoologyResult,
};
use hyper::{
    header::Authorization,
    net::HttpsConnector,
    status::StatusCode,
    Client as HyperClient,
};
use hyper_native_tls::NativeTlsClient;
use rand::Rng;
use std::{
    borrow::Cow,
    io::Read,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

fn get_nonce() -> String {
    rand::thread_rng().gen_ascii_chars().take(11).collect()
}

fn get_auth_header(token: &str, secret: &str) -> String {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // TODO: HandleError
        .as_secs();
    let nonce = get_nonce();
    format!(r#"OAuth realm="Schoology API", oauth_consumer_key="{}", oauth_token="", oauth_nonce="{}", oauth_timestamp="{}", oauth_signature_method="PLAINTEXT", oauth_version="1.0", oauth_signature="{}%26""#, token, nonce, time, secret)
}

pub struct Client {
    handle: HyperClient,
    token: String,
    secret: String,
}

impl Client {
    pub fn new(token: String, secret: String) -> Self {
        let ssl = NativeTlsClient::new().unwrap(); // TODO: Return result
        let connector = HttpsConnector::new(ssl);
        Client {
            token,
            secret,
            handle: HyperClient::with_connector(connector),
        }
    }

    pub fn send_request(&self, req: Request) -> SchoologyResult<String> {
        // println!("{:#?}", req);
        let mut res = self
            .handle
            .get(&*req.url)
            .header(Authorization(get_auth_header(&self.token, &self.secret)))
            .send()
            .map_err(|_| SchoologyError::Network)?;

        if !res.status.is_success() {
            return match res.status {
                StatusCode::Unauthorized => Err(SchoologyError::UnauthorizedRequest),
                _ => Err(SchoologyError::InvalidStatusCode(res.status.to_u16())),
            };
        }

        // println!("{:#?}", res);

        let mut body = String::new();
        res.read_to_string(&mut body)
            .map_err(|_| SchoologyError::InvalidBody)?;
        // println!("{}", body);
        Ok(body)
    }

    pub fn get_realm<T>(&self, data: &str) -> SchoologyResult<T>
    where
        T: SchoologyRealm,
    {
        serde_json::from_str(&self.send_request(T::get(data))?)
            .map_err(|err| SchoologyError::InvalidJson(format!("{}", err)))
    }

    pub fn get_realms<T>(&self, start: usize, length: usize) -> SchoologyResult<T>
    where
        T: SchoologyRealmList,
    {
        serde_json::from_str(&self.send_request(T::get_many(start, length))?)
            .map_err(|err| SchoologyError::InvalidJson(format!("{}", err)))
    }

    pub fn get_realm_object<T, U>(&self, realm: &U, id: &str) -> SchoologyResult<T>
    where
        U: SchoologyRealm,
        T: SchoologyRealmObject<Parent = U>,
    {
        serde_json::from_str(&self.send_request(T::get(&realm, id))?)
            .map_err(|err| SchoologyError::InvalidJson(format!("{}", err)))
    }

    pub fn get_realm_objects<T, U>(
        &self,
        realm: &U,
        start: usize,
        length: usize,
    ) -> SchoologyResult<T>
    where
        U: SchoologyRealm,
        T: SchoologyRealmObjectList<Parent = U>,
    {
        serde_json::from_str(&self.send_request(T::get_many(&realm, start, length))?)
            .map_err(|err| SchoologyError::InvalidJson(format!("{}", err)))
    }
}

#[derive(Debug)]
pub struct Request {
    pub url: Cow<'static, str>,
}
