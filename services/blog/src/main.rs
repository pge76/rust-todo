use askama::Template;
use axum::{
    Form, Router,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use serde::Deserialize;
use std::sync::{Mutex, OnceLock};

#[derive(Template)]
#[template(path = "index.html")]
struct BlogIndexTemplate<'a> {
    blog_posts: &'a Vec<String>,
}

#[derive(Deserialize)]
struct AddPost {
    blog_post: String,
}

fn blog_post_storage() -> &'static Mutex<Vec<String>> {
    static BLOG_POSTS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    BLOG_POSTS.get_or_init(|| Mutex::new(vec![]))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_blog_posts))
        .route("/     add", post(add_blog_post));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

/// GET's all the blog posts
async fn get_blog_posts() -> Html<String> {
    let blog_posts = blog_post_storage().lock().unwrap();
    let template = BlogIndexTemplate {
        blog_posts: &blog_posts,
    };
    Html(template.render().unwrap())
}

/// POST's a new Blog Post
async fn add_blog_post(Form(input): Form<AddPost>) -> StatusCode {
    let mut blog_posts = blog_post_storage().lock().unwrap();
    blog_posts.push(input.blog_post);
    StatusCode::SEE_OTHER
}
