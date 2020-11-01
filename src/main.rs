use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String,
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("First link"),
            link: String::from("https://link1.com"),
            author: String::from("A"),
        },
        Post {
            title: String::from("Second link"),
            link: String::from("https://link2.com"),
            author: String::from("B"),
        },
    ];

    data.insert("title", "Hacker Clone");
    data.insert("posts", &posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
