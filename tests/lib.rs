extern crate json_request;

extern crate iron;
#[macro_use]
extern crate router;
extern crate bodyparser;

extern crate hyper;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::Protocol;

use json_request::{request, Method};

struct PingServer;

impl PingServer {
    pub fn build() -> Iron<Chain> {
        Iron::new(Chain::new(router!(
            post "/ping" => PingServer::post
        )))
    }

    fn post(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "{\"pong\": true}")))
    }
}

#[derive(Debug, RustcEncodable)]
struct RequestData {
    ping: bool
}

#[derive(Debug, RustcDecodable)]
struct ResponseData {
    pong: bool
}

const HOST: &'static str = "0.0.0.0:12345";
fn url(frag: &str) -> String {
    format!("http://{}{}", HOST, frag)
}

struct StackListener {
    server: ::hyper::server::Listening
}

impl StackListener {
    pub fn new() -> StackListener {
        StackListener {
            server: PingServer::build().listen_with(HOST, 1, Protocol::Http).unwrap()
        }
    }
}

impl Drop for StackListener {
    fn drop(&mut self) {
        self.server.close().unwrap();
    }
}

#[test]
#[allow(unused_variables)]
fn ping_pong() {
    let server = StackListener::new();

    let req = RequestData { ping: true };

    // When this fails, the error I get it "called Option::unwrap() on a None value" which is not
    // helpful for resolving what the problem is.
    let res: ResponseData = request(Method::Post, &(url("/ping"))[..], Some(req)).unwrap().unwrap();
}
