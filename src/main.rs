use fantoccini::{Client, Locator};

extern crate tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c = Client::new("http://localhost:4444").await?;

    c.goto("http://localhost:8000").await?;

    let e = c.find(Locator::Id("inlineFrameExample")).await?;

    let frame_client = e.frame().await?;

    Ok(())
}



