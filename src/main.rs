#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arch = std::env::consts::ARCH;
    println!("Hello, {arch}!\n");

    let url = "https://httpbin.org/get";

    let client = reqwest::Client::default();
    let status = client
        .get("https://httpbin.org/get")
        .send()
        .await
        .ok()
        .map(|res| res.status());

    println!(
        "I can also do HTTPS\n\nGET {url}: {:#?}",
        status.unwrap_or_default()
    );

    Ok(())
}
