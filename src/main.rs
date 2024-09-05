use error_chain::error_chain;
use std::collections::HashMap;
use std::error::Error;

/* error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
} */

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let res = reqwest::get("https://jsonplaceholder.typicode.com/posts").await?;

    let status = res.status();
    let headers = res.headers();
    let body = res.text().await?;

    println!("{}", status);

    Ok(())
}
