use serde::{Serialize, Deserialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct SimplePost {
    pub title: String,
    pub body: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

use diesel::prelude::*;
use diesel::{PgConnection, r2d2::{ConnectionManager, PooledConnection}};

impl Post {
    pub fn slugify(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }

    pub fn create_post<'a> (
        mut conn: PooledConnection<ConnectionManager<diesel::PgConnection>>,
        post: &NewPostHandler
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(&post.title.clone());
        let new_post = NewPost {
            title: &post.title,
            body: &post.body,
            slug: &slug
        };

        diesel::insert_into(post::table)
            .values(&new_post)
            .get_result::<Post>(&mut conn)
    }
}

use super::schema::post;

#[derive(Insertable)]
#[table_name="post"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str,
}
