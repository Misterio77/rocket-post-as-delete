//! `rocket-post-as-delete` is a Fairing for [Rocket](https://rocket.rs) rewriting
//! requests such as `POST foo/bar/delete` into `DELETE foo/bar`.
//!
//! This is useful when you have web forms (which, unless you use javascript, only
//! support POST and GET) for deleting stuff, and want to write those routes with
//! (the more correct) `DELETE` verb.
//!
//! # Example
//!
//! ```rust
//! use rocket::{launch, delete, routes};
//! use rocket_post_as_delete::PostAsDelete;
//! 
//! #[launch]
//! fn rocket() -> _ {
//!    rocket::build()
//!        .attach(PostAsDelete)
//!        .mount("/", routes![delete_foo_bar])
//! }
//! 
//! #[delete("/foo/bar")]
//! async fn delete_foo_bar() -> String {
//!     "Poof!".into()
//! }
//! 
//! # use rocket::local::blocking::Client;
//! # use rocket::http::Status;
//! 
//! # #[test]
//! # fn rewrites() {
//! #     let client = Client::tracked(rocket()).expect("valid rocket instance");
//! #     let response = client.post("/foo/bar/delete").dispatch();
//! #     assert_eq!(response.status(), Status::Ok);
//! #     assert_eq!(response.into_string().unwrap(), "Poof!");
//! # }
//! ```
//!
//! Now forms such as this (`POST` verb, and submit URL suffixed by `/delete`):
//! ```html
//! <form method="post" action="/foo/bar/delete">
//!     <button>Delete me!</button>
//! </form>
//! ```
//!
//! Will run the `delete_foo_bar` route as expected.

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Method,
    Data, Request,
};

pub struct PostAsDelete;

#[rocket::async_trait]
impl Fairing for PostAsDelete {
    fn info(&self) -> Info {
        let kind = Kind::Request;
        Info {
            kind,
            name: "POST as DELETE",
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let last = request.uri().path().segments().last();
        if request.method() == Method::Post && last == Some("delete") {
            request.set_method(Method::Delete);
            request.set_uri(
                request
                    .uri()
                    .map_path(|p| p.strip_suffix("/delete").unwrap_or(p))
                    .unwrap(),
            );
        }
    }
}
