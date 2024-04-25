#![allow(unused)]

//!
//! BASICS
//! ------
//!
//! Axum is a web application framework that focuses on ergonomics, modularity,
//! and performance.
//!
//! In this section, you will learn the basics of building web applications
//! using the Axum framework. Although many of the specifics that you learn
//! will be Axum-specific, the concepts that you learn will be applicable to
//! other web frameworks as well.
//!  

use axum::{
    body::Body,
    http::{Method, Request},
    response::Html,
    routing::*,
    Json, Router,
    extract::Path,
};
use bitcoin::Block;
use http_body_util::BodyExt;

///
/// In this "hello world" example, you can see the core elements of an Axum
/// web application:
///
/// 1. A router, which is used for specifying routes.
/// 2. A single route, defined with a path and a handler.
/// 3. A handler, which is an asynchronous function that returns a response.
/// 4. A listener, which is used to listen for incoming connections.
/// 5. A call to `axum::serve`, which starts the server.
///
pub async fn hello_world() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

/// Use the `Html` constructor to construct an HTML response that includes the
/// text "Hello, World!".
///
/// Run the hello_world() function to ensure that you can browse the `/` route
/// and that it properly serves the static HTML.
///
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::test]
async fn test_hello_world() {
    let Html(s) = handler().await;

    assert!(s.contains("Hello, World!"));
}

/// Extract path examples

async fn get_block(Path(block_height): Path<u64>) -> String {
    use colored::*;
    println!("{} {}", "Path extracted automatically".yellow() ,block_height);
    "".to_owned()
}

#[tokio::test]
async fn test_path_extractor() {
    use axum::http::StatusCode;
    // for Body::collect
    // use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let _app = Router::new().route("/get_block/:block_height", get(get_block));

    let _req: Request<Body> = Request::builder()
        .method(Method::GET)
        .uri("/get_block/100idsda")
        .body(Body::empty())
        .unwrap();

    let response = _app.oneshot(_req).await.unwrap();
    println!("{}", response.status());
}

/// To factor out duplication across route paths, you can use the `nest` method
/// on Router. This method takes a path prefix and a router, and returns a new
/// router that has the path prefix applied to all of the routes in the nested
/// router.
///
/// In the following example, use the `nest` method to nest all of the user
/// routes under the `/users` path prefix of the specified router.
///
fn nest_router<S: Clone + Send + Sync + 'static>(_router: Router<S>) -> Router<S> {
    let _user_routes = Router::<S>::new()
        .route("/", get(handler))
        .route("/:id", get(handler))
        .route("/", post(handler))
        .route("/:id", put(handler))
        .route("/:id", delete(handler));

    _router.nest("/users", _user_routes)
}

#[tokio::test]
async fn test_nest_router() {
    use axum::http::StatusCode;
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let _app = Router::new().route("/", get(|| async {"ROOT"}));
    let _app = nest_router(_app);

    let _req: Request<Body> = Request::builder()
        .method(Method::GET)
        .uri("/users")
        .body(Body::empty())
        .unwrap();

    let response = _app.oneshot(_req).await.unwrap();
    println!("{}", response.status());
    dbg!("{}", &response);
    dbg!("{}", response.into_body().collect().await.unwrap().to_bytes());
}


/// Being able to test your routes without spinning up a server is very important for
/// performance and determinism. Fortunately, Axum is built on Tower, which provides a
/// convenient way to test your routes (`oneshot`).
///
/// Use `Request::builder` to construct a `Request` that makes the following unit test
/// pass. Try to pay attention to how to use `oneshot` and which imports are needed and
/// for what reasons.
///
#[tokio::test]
async fn test_routes() {
    use axum::http::StatusCode;
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let _app = Router::new().route("/users", get(identity_handler));

    let _req: Request<Body> = Request::builder()
        .method(Method::GET)
        .uri("/users1")
        .body(Body::empty())
        .unwrap();

    let response = _app.oneshot(_req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "/users");
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
struct Person {
    name: String,
    age: u8,
}

/// Axum makes it easy for your handlers to return JSON responses. To do so, you
/// can use the `Json` wrapper type, which implements `From<T>` for any type `T`
/// that implements `serde::Serialize`.
///
/// Create a `struct` and be sure to derive Serialize, and then use your struct
/// in the following test and handler.
///
#[tokio::test]
async fn test_basic_json() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users/jdoe", get(return_json_hello_world));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users/jdoe")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, r#"{"name":"John Doe","age":43}"#);
}

async fn return_json_hello_world() -> Json<Person> {
    Json(Person {
        name: "John Doe".to_string(),
        age: 43,
    })
}
async fn identity_handler(request: Request<Body>) -> Body {
    Body::from(request.uri().path().to_string())
}

/// A handler may accept a `axum::Json<A>` for any type `A` that has an implementation of
/// the `serde::Deserialize` trait. Create a `Person` data structure with a single field
/// `name` of type `String` and implement `serde::Deserialize` for it. Then, modify the
/// handler `json_handler` to return the name of the person.
///
#[tokio::test]
async fn json_handler_test() {
    // for Body::collect
    use http_body_util::BodyExt;
    /// for ServiceExt::oneshot
    use tower::util::ServiceExt;

    let app = Router::<()>::new().route("/users/jdoe", get(json_handler));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/users/jdoe")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"name": "John Doe"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_as_string = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_as_string, "John Doe");
}

async fn json_handler(Json(Person { name, age }): Json<Person>) -> String {
    name
}
