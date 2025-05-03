use askama::Template;
use axum::{
    Form, Router,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};
use uuid::Uuid;

/// Templates ///

#[derive(Template)]
#[template(path = "index.html")]
struct BlogIndexTemplate<'a> {
    blog_posts: &'a Vec<&'a GetPost>,
}

#[derive(Template)]
#[template(path = "blog_input.html")]
struct BlogInputTemplate {}

/// Data Structures / Value Objects ///

#[derive(Deserialize)]
struct AddPost {
    blog_title: String,
    blog_text: String,
}

#[derive(Deserialize)]
struct GetPost {
    blog_title: String,
    blog_text: String,
}

/// Storage /// temporary storage for blog posts, just a HashMap
fn blog_post_storage() -> &'static Mutex<HashMap<Uuid, GetPost>> {
    static BLOG_POSTS: OnceLock<Mutex<HashMap<Uuid, GetPost>>> = OnceLock::new();
    BLOG_POSTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Main function ///

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_blog_posts))
        .route("/blog", get(get_blog_posts))
        .route("/blog", post(add_blog_post))
        .route("/blog/input", get(get_blog_post_input));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

/// GET's all the blog posts
async fn get_blog_posts() -> Html<String> {
    let blog_posts = blog_post_storage().lock().unwrap();
    let blog_posts: Vec<&GetPost> = blog_posts.values().collect();
    let template = BlogIndexTemplate {
        blog_posts: &blog_posts,
    };
    Html(template.render().unwrap())
}

/// POST's a new Blog Post
async fn add_blog_post(Form(input): Form<AddPost>) -> Result<Html<String>, StatusCode> {
    if input.blog_title.trim().is_empty() || input.blog_text.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut blog_posts = blog_post_storage().lock().unwrap();

    let post = GetPost {
        blog_title: input.blog_title,
        blog_text: input.blog_text,
    };
    blog_posts.insert(Uuid::new_v4(), post);
    let blog_posts: Vec<&GetPost> = blog_posts.values().collect();
    let template = BlogIndexTemplate {
        blog_posts: &blog_posts,
    };
    template
        .render()
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// GET's a blog formular to input a new blog
async fn get_blog_post_input() -> Html<String> {
    let template = BlogInputTemplate {};
    Html(template.render().unwrap())
}
