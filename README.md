# Rocket POST as DELETE Fairing

[![Crates.io Version](https://img.shields.io/crates/v/rocket-post-as-delete.svg)](https://crates.io/crates/rocket-post-as-delete)

`rocket-post-as-delete` is a Fairing for [Rocket](https://rocket.rs) rewriting
requests such as `POST foo/bar/delete` into `DELETE foo/bar`.

This is useful when you have web forms (which, unless you use javascript, only
support POST and GET) for deleting stuff, and want to write those routes with
(the more correct) `DELETE` verb.

## Installing

Add to your `Cargo.toml`:
```
rocket-post-as-delete = "0.1"
```

## Usage

```rust
use rocket_post_as_delete::PostAsDelete;

#[rocket::main]
async fn main() {
   rocket::build()
       .attach(PostAsDelete)
       .mount("/", routes![delete_foo_bar])
       .launch()
       .await;
}

#[delete("/foo/bar")]
async fn delete_foo_bar() -> String {
    "Poof!"
}
```

Now forms such as this (`POST` verb, and submit URL suffixed by `/delete`):
```html
<form method="post" action="/foo/bar/delete">
    <button>Delete me!</button>
</form>
```

Will run the `delete_foo_bar` route as expected.
