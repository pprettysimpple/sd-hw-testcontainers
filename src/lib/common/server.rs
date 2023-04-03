use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Uri};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use log::{error, info};
use url::Url;
use crate::account::account::Account;
use crate::common::share::Share;

pub fn get_from_params<T>(key: &str, uri: &Uri) -> Result<T, String>
    where T: std::str::FromStr {
    Ok(Url::parse(&format!("http://localhost{}", uri))
        .map_err(|e| e.to_string())?
        .query_pairs()
        .find(|(k, _)| k == key)
        .ok_or(format!("Failed to get {} from params", key))?
        .1
        .to_string()
        .parse::<T>()
        .or(Err(format!("Failed to parse {} from params", key)))?)
}

pub fn ser_share(share: Share) -> Result<String, String> {
    Ok(serde_json::to_string(&share)
        .map_err(|e| e.to_string())?)
}

pub fn des_share(body: String) -> Result<Share, String> {
    Ok(serde_json::from_str(&body)
        .map_err(|e| e.to_string())?)
}

pub fn ser_shares(shares: Vec<Share>) -> Result<String, String> {
    Ok(serde_json::to_string(&shares)
        .map_err(|e| e.to_string())?)
}

pub fn ser_account(account: Account) -> Result<String, String> {
    Ok(serde_json::to_string(&account)
        .map_err(|e| e.to_string())?)
}

pub fn des_account(body: String) -> Result<Account, String> {
    Ok(serde_json::from_str(&body)
        .map_err(|e| e.to_string())?)
}

pub fn ser_str(s: String) -> Result<String, String> {
    Ok(serde_json::to_string(s.as_str())
        .map_err(|e| e.to_string())?)
}

pub fn des_str(body: String) -> Result<String, String> {
    Ok(serde_json::from_str(&body)
        .map_err(|e| e.to_string())?)
}

async fn handle<TContext, F, Fut>(
    context: TContext,
    _addr: SocketAddr,
    req: Request<Body>,
    handle_impl: F,
) -> Result<Response<Body>, Infallible>
    where TContext: Clone + Send + Sync + 'static,
          F: Fn(TContext, Request<Body>) -> Fut + Send + Sync + Clone + 'static,
          Fut: Future<Output=Result<Response<Body>, String>> + Send {
    match handle_impl(context, req).await {
        Ok(resp) => Ok(resp),
        Err(e) => {
            error!("Error: {}", e);
            Ok(Response::builder().status(500).body(Body::from(e)).unwrap())
        }
    }
}

pub async fn run_server<TContext, F, Fut>(context: TContext, handle_impl: F, port: u16)
    where
        TContext: Clone + Send + Sync + 'static,
        F: Fn(TContext, Request<Body>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output=Result<Response<Body>, String>> + Send + 'static{
    let make_service = make_service_fn(move |conn: &AddrStream| {
        let addr = conn.remote_addr();
        let context = context.clone();
        let handle_impl = handle_impl.clone();

        let service = service_fn(move |req| {
            handle(context.clone(), addr, req, handle_impl.clone())
        });

        async move { Ok::<_, String>(service) }
    });

    let addr = ([0, 0, 0, 0], port).into();
    let server = Server::bind(&addr)
        .serve(make_service);

    info!("Listening on {}", addr);
    info!("Press Ctrl+C to stop");

    server
        .await
        .expect("Server error");
}
