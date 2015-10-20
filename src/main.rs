extern crate iron;
extern crate router;

use std::io::prelude::*;
use std::fs::File;

use iron::Iron;
use iron::IronResult;
use iron::Request;
use iron::Response;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;

use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", handler);
    router.get("/:query", handler);

    Iron::new(router).http("0.0.0.0:9023").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("index.html");

        for d in std::fs::read_dir("/etc/").unwrap() {
            println!("{:?}", d.unwrap().path());
        }
        let mut f = match File::open(&format!("/etc/{}", query)) {
            Ok(f) => f,
            Err(e) => { let d = format!("{:?}", e); return Err(iron::error::IronError::new(e, d)) },
        };
        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => (),
            Err(e) => { return Err(iron::error::IronError::new(e, "nope")) },
        }

        Ok(Response::with((Header(ContentType::html()), status::Ok, s)))
    }
}
