mod api;
mod error;

use colored::Colorize;
use dotenv::dotenv;
use error::{Error, ErrorKind};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenv();
    let token = env::var("OPENAI_TOKEN").expect("No key provided. export OPENAI_TOKEN and try again.");
    let query_words: Vec<String> = env::args().skip(1).collect();
    let query = query_words.join(" ");

    println!("{}", query.cyan().bold());

    let res = api::query_chat_gpt(&token, &query).await;

    println!("{}", "---".green().bold());

    let result = if let Ok(value) = res {
        let value_string = value.to_string();

        for line in value_string.split("\\n") {
            if !line.is_empty() {
                println!("{}", line.green())
            }
        }

        Ok(())
    } else {
        Err(Error {
            kind: ErrorKind::RequestFailed,
            message: None,
        })
    };

    println!("{}", "---".green());
    result
}
