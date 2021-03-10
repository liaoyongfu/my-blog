use actix_web::{Responder, HttpResponse, get, post, Result};
use std::collections::HashMap;
use std::any::Any;
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
struct Article<'a> {
    title: &'a str,
    date: &'a str,
    brief: &'a str,
    id: &'a str
}

#[derive(Serialize, Deserialize)]
struct Articles<'a> {
    code: u32,
    #[serde(borrow)]
    data: Vec<Article<'a>>
}

#[get("/getArticles")]
pub async fn get_articles() -> Result<HttpResponse> {
    let c = vec![
        Article {
            title: "Rust 动态类型",
            date: "2021-3-8 17:31:25",
            brief: "最近在做读取数据的过程中，需要有一定灵活度。所以尝试着实现动态类型。",
            id: "1"
        }
    ];
    Ok(
        HttpResponse::Ok().content_type("application/json").json(Articles {
            code: 1200,
            data: c
        })
    )
}
