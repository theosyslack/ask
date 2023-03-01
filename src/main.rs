mod api;
mod error;

use colored::Colorize;
use dotenv::dotenv;
use error::{Error, ErrorKind};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenv();
    let token = env::var("OPENAI_TOKEN")?;
    let query_words: Vec<String> = env::args().skip(1).collect();
    let query = query_words.join(" ");

    println!("{}", query.cyan().bold());

    let res = api::query_chat_gpt(&token, &query).await;

    println!("{}", "---".green().bold());

    let result = if let Ok(value) = res {
        let mut value_string = value.to_string();
        value_string.retain(|c| c != '"');
        // let updated_string = value_string.replace('\n', "");
        // let string = format!("{}", value_string);
        for line in value_string.split("\\n") {
            if !line.is_empty() {
                println!("{}", line.green())
            }
        }

        // println!("{}", value_string);
        // print!("{string}");
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
