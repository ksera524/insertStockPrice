use std::env;

use serde_derive::{Deserialize, Serialize};
use serde_json;
use chrono::{Duration, Local};
use reqwest::{self, Client};
use tokio;

use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::NaiveDate;

#[derive(Debug)]
struct DbStockPrice {
    stock_symbol: String,
    market: String,
    date: NaiveDate,
    price: f64,
    volume: i64,
}

impl DbStockPrice {
    fn new(stock:Stock) -> DbStockPrice {
        DbStockPrice {
            stock_symbol: match stock.symbol {
                Symbol::Int(i) => i.to_string(),
                Symbol::Str(s) => s,
            },
            market: stock.market,
            date: get_yesterday(),
            price: stock.price as f64,
            volume: stock.volume,
        }
    }
    
}

#[derive(Serialize, Deserialize, Debug)]
struct DataWrapper {
    data: Vec<Stock>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Symbol {
    Int(u32),
    Str(String),
}


#[derive(Serialize, Deserialize, Debug)]
struct Stock {
    #[serde(rename = "Symbol")]
    symbol: Symbol,
    #[serde(rename = "Price")]
    price: f32,
    #[serde(rename = "Volume")]
    volume: i64,
    #[serde(rename = "Market")]
    market: String,
}

#[derive(Serialize)]
struct Password {
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    println!("記録日時: {:?}", get_yesterday());

    println!("Fetching stock prices from the spreadsheet");
    let spreadsheet_url = env::var("SPREADSHEET_URL").unwrap();

    let password = Password {
        password: env::var("SPREADSHEET_PASSWORD").unwrap(),
    };

    let client = Client::new();

    let res = client
        .post(spreadsheet_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&password).unwrap())
        .send()
        .await?;

    let body = res.text().await?;

    let parsed: DataWrapper = serde_json::from_str(&body).unwrap();

    println!("Fetched stock prices: {:?}", parsed.data.len());

    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    //DB insert
    println!("Inserting stock prices into the database");
    for stock in parsed.data {
        let db_stock = DbStockPrice::new(stock);
        let _result = sqlx::query!(
            r#"
            INSERT INTO stock_prices (stock_symbol, market, date, price, volume)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            db_stock.stock_symbol,
            db_stock.market,
            db_stock.date,
            db_stock.price,
            db_stock.volume
        )
        .execute(&pool)
        .await?;
    }
    
    println!("Inserted stock prices into the database");

    Ok(())
}

fn get_yesterday() -> NaiveDate {
    let today = Local::now().date_naive();
    today - Duration::days(1)
}
