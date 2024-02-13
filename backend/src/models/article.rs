use diesel::{self, prelude::*, result::QueryResult};
use serde::{Deserialize, Serialize};

mod schema {
    table! {
        articles {
            id -> Nullable<Integer>,
            title -> VarChar,
            content -> VarChar,
            date -> BigInt
        }
    }
}

use self::schema::articles;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = articles)]
pub struct Article {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub date: i64
}

#[derive(Deserialize)]
pub struct NewArticle {
    pub title: String,
    pub content: String,
    pub date: i64
}

impl Article {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Article>> {
        conn.run(|c| articles::table.order(articles::id.desc()).load(c))
            .await
    }

    pub async fn insert(article: NewArticle, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Article {
                id: None,
                title: article.title,
                content: article.content,
                date: article.date
            };
            diesel::insert_into(articles::table).values(&c).execute(conn)
        })
        .await
    }

    pub async fn update(article: Article, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_article = diesel::update(articles::table.filter(articles::id.eq(article.id)));
            updated_article
                .set((
                    articles::title.eq(article.title),
                    articles::content.eq(article.content),
                    articles::date.eq(article.date),
                ))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(articles::table)
                .filter(articles::id.eq(id))
                .execute(c)
        })
        .await
    }
}
