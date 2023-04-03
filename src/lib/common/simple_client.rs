use std::str::FromStr;
use hyper::{Body, Client, Request, Uri};
use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use log::debug;

#[derive(Clone, Debug)]
pub struct SimpleClient {
    client: Client<HttpConnector>,
    pub(crate) host: String,
}

impl SimpleClient {
    pub fn new(host: String) -> SimpleClient {
        SimpleClient {
            client: Client::new(),
            host,
        }
    }

    pub async fn send_get_with_body_and_parse<T>(&self, uri: Uri, body: String) -> Result<T, String>
        where T: serde::de::DeserializeOwned {
        debug!("Sending request to {} with body {}", uri, body);

        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::from(body))
            .unwrap();

        let mut body = self.client.request(req)
            .await
            .map_err(|e| e.to_string())?
            .into_body();

        let body_bytes = body.data().await.ok_or("Failed to get body data")?.map_err(|e| e.to_string())?;
        debug!("Got response with body {}", String::from_utf8(body_bytes.to_vec()).unwrap());
        serde_json::from_slice::<T>(body_bytes.as_ref()).map_err(|e| e.to_string())
    }

    pub async fn send_get_and_parse<T>(&self, uri: Uri) -> Result<T, String>
        where T: serde::de::DeserializeOwned {
        self.send_get_with_body_and_parse(uri, String::new()).await
    }
}

pub fn make_query(host: &String, method: &str, query: String) -> Result<Uri, String> {
    Uri::from_str(&format!("http://{}{}?{}", host, method, query)).map_err(|e| e.to_string())
}
