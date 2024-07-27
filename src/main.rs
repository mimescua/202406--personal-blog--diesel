#[macro_use] // Enable using macro
extern crate diesel; // Enable intensive use of Diesel

pub mod schema;
pub mod models;

use diesel::associations::HasTable;
use dotenv::dotenv;
use std::env;

// use log::{error, info};
// use log::{debug, error, log_enabled, info, Level};

use diesel::{connection, prelude::*};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use tera::Tera;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

use self::models::{Post, NewPost, NewPostHandler, SimplePost};
use self::schema::post;
use self::schema::post::dsl::*;

// #[get("/tera")]
// async fn tera_test(template_manager: web::Data<tera::Tera>) -> impl Responder {
//     let mut context = tera::Context::new();

//     HttpResponse::Ok()
//     .content_type("text/html")
//     .body(template_manager.render("index.html", &context).unwrap())
// }

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let mut conn = pool.get().expect("Data connection errors");

    match web::block(move || {post.load::<Post>(&mut conn)}).await {
        Ok(data) => {
            let mut context = tera::Context::new();

            let data = data.unwrap();
            context.insert("posts", &data);

            // return HttpResponse::Ok().body(format!("{:?}", data));
            // HttpResponse::Ok().body(format!("{:?}", data))
            HttpResponse::Ok()
            .content_type("text/html")
            .body(template_manager.render("index.html", &context).unwrap())
        },
        Err(err) => HttpResponse::Ok().body("Errors getting data!")
    }
}

#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
    blog_slug: web::Path<String>
) -> impl Responder {
    let mut conn = pool.get().expect("Data connection errors");
    let url_slug = blog_slug.into_inner();

    match web::block(move || {post.filter(slug.eq(url_slug)).load::<Post>(&mut conn)}).await {
        Ok(data) => {
            let data = data.unwrap();
            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }
            let data = &data[0];

            let mut context = tera::Context::new();
            context.insert("post", data);

            HttpResponse::Ok()
            .content_type("text/html")
            .body(template_manager.render("post.html", &context).unwrap())
        },
        Err(err) => HttpResponse::Ok().body("Errors getting data!")
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Data connection errors");

    match web::block(move || { Post::create_post(conn, &item)})
    .await {
        Ok(data) => {
            HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(err) => HttpResponse::Ok().body("Errors getting data!")
    }
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("PORT not found!");
    let port: u16 = port.parse().unwrap();
    
    let db_url = env::var("DATABASE_URL").expect("DB URL not found!");
    let connection = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(connection).expect("Pool building failed!");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
        .service(index)
        .service(get_post)
        .service(new_post)
        // .service(tera_test)
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(tera))
        .route("/hey", web::get().to(hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
