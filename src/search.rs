use reqwest::Error;
use serde_json::Value;

pub async fn buscar_imagen(titulo: &str, api_key: &str, cx: &str) -> Result<String,Error>{
    let url = format!(
    "https://www.googleapis.com/customsearch/v1?q={}&cx={}&key={}&searchType=image&num=1",
    titulo,
    cx,
    api_key
    );

    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.json::<Value>().await?;

    let imagen_url = res["items"][0]["link"].as_str().unwrap_or_default().to_string();

    Ok(imagen_url)
}