use rocket::{
    get,
    response::stream::{Event, EventStream},
    routes,
    serde::json::Json,
    tokio::time::{self, Duration},
};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use serde::Serialize;
use std::{error::Error, str::FromStr};

#[derive(Serialize)]
struct Comment {
    author: String,
    text: String,
}

#[derive(Serialize)]
struct Post {
    title: String,
    body: String,
    comments: Vec<Comment>,
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::All;
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: ["Get"]
            .iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .attach(cors)
        .mount("/", routes![post, comments])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
async fn post() -> Json<Post> {
    Json(Post {
        title: "Hello, world!".to_string(),
        body: "Hello, world! My first blog post!".to_string(),
        comments: vec![Comment {
            author: "Rando Jackson".to_string(),
            text: "First!".to_string(),
        }],
    })
}

#[get("/comments_stream")]
async fn comments() -> EventStream![] {
    let comments = vec![
        Comment {
            author: "John Doe".to_string(),
            text: "My first comment!".to_string(),
        },
        Comment {
            author: "Jane Doe".to_string(),
            text: "My first comment, too!".to_string(),
        },
        Comment {
            author: "Joe Schmoe".to_string(),
            text: "I was first!".to_string(),
        },
        Comment {
            author: "John Doe".to_string(),
            text: "@JoeSchmoe: no you weren't!".to_string(),
        },
        Comment {
            author: "Jane Doe".to_string(),
            text: "Play nicely, you two!".to_string(),
        },
    ];
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(2));
        let mut comments_iter = comments.iter();
        while let Some(comment) = comments_iter.next() {
            interval.tick().await;
            yield Event::json(&comment).event("comment");
        }
        yield Event::data("terminate").event("close");
    }
}
