use anyhow::Result;
use prettytable::Table;
use prettytable::{cell, format, row};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct API {
    quote_response: QuoteResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuoteResponse {
    result: Vec<Quote>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Quote {
    symbol: String,
    display_name: String,
    regular_market_price: f32,
    regular_market_change: f32,
    regular_market_change_percent: f32,
    financial_currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stocks {
    stocks: Vec<String>,
}

impl API {
    async fn get(symbol: &String) -> Result<API> {
        let url_string = format!("https://query1.finance.yahoo.com/v7/finance/quote?lang=en-US&region=US&corsDomain=finance.yahoo.com&symbols={}",
            symbol,
        );

        let url = Url::parse(&*url_string)?;
        let res = reqwest::get(url).await?.json::<API>().await?;

        Ok(res)
    }
}

impl Stocks {
    fn get(&self) -> Result<String> {
        let mut s = "".to_string();
        for (i, a) in self.stocks.iter().enumerate() {
            if i == self.stocks.len() - 1 {
                s.push_str(&format!("{}", a));
            } else {
                s.push_str(&format!("{},", a));
            }
        }

        Ok(s)
    }
}

fn display_stock_price(stock: &Quote) -> String {
    format!("$ {:.2} ({})", stock.regular_market_price, stock.financial_currency)  
}

fn display_stock_change(stock: &Quote) -> String {
    if stock.regular_market_change < 0.0 {
        format!("($ {:.2})", stock.regular_market_change.abs())
    } else {
        format!(" $ {:.2} ", stock.regular_market_change)
    }
}

fn read_config() -> Result<String> {
    let f = std::fs::File::open(".ticker.yml")?;
    let data: Stocks = serde_yaml::from_reader(f)?;

    Ok(data.get()?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let symbol: String;

    if args.len() < 2 {
        symbol = read_config()?;
    } else {
        symbol = args[1].clone();
    }

    let res = API::get(&symbol).await?;
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["STOCK", "SYMBOL", "COST", "CHANGE", "% CHANGE"]);
    for stock in res.quote_response.result.iter() {
        table.add_row(row![
            stock.display_name,
            stock.symbol,
            display_stock_price(stock),
            display_stock_change(stock),
            format!("{:.2}%", stock.regular_market_change_percent),
        ]);
    }

    table.printstd();

    Ok(())
}
