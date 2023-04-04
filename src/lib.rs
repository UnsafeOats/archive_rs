use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::time::Duration;
use urlencoding::encode;

pub struct ArchiveSesh {
    pub submit_id: String,
}

impl ArchiveSesh {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let url = "https://archive.is";

        // Make the GET request
        let client = Client::new();
        let response = client.get(url).send().await?;

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(format!("Request to {} failed with status: {}", url, response.status()).into());
        }

        // Parse the HTML response
        let body = response.text().await?;
        let document = Html::parse_document(&body);

        // Extract the submitid value from the HTML
        let selector = Selector::parse(r#"input[name="submitid"]"#)?;
        let submit_id_element = document.select(&selector).next().ok_or("submitid not found")?;
        let submit_id = submit_id_element.value().attr("value").ok_or("submitid value not found")?.to_string();

        Ok(ArchiveSesh { submit_id })
    }

    pub async fn archive_url(&self, url: &str) -> Result<Option<String>, Box<dyn Error>> {
        let encoded_submit_id = encode(&self.submit_id);
        let encoded_url = encode(url);
        let archive_url = format!(
            "https://archive.is/submit/?submitid={}&url={}",
            encoded_submit_id, encoded_url
        );

        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/65.0.3325.162 Safari/537.36")
            .timeout(Duration::from_secs(10))
            .build()?;
        let response = client.get(&archive_url).send().await?;
        println!("\nResponse: {:?}\n", &response);
        let extracted_url = match response.url().host_str() {
            Some(host) => match response.url().query() {
                Some(query) => Some(format!("{}://{}{}?{}", response.url().scheme(), host, response.url().path(), query)),
                None => Some(format!("{}://{}{}", response.url().scheme(), host, response.url().path())),
            },
            None => None,
        };
        println!("Response URL: {:?}", extracted_url);

        Ok(extracted_url)
    }

    pub async fn wait_for_archive(&self, url: &str, iter_wait: Duration, max_iter: usize) -> Result<Option<String>, Box<dyn Error>> {
        let mut iter = 0;
        loop {
            let archive_url = self.archive_url(url).await?;
            if (!archive_url.clone().unwrap_or("submitid".to_string()).contains("submitid")) || (iter >= max_iter) {
                return Ok(archive_url);
            }
            iter += 1;
            println!("Waiting for next iteration... ({} of {})", iter, max_iter);
            tokio::time::sleep(iter_wait).await;
        }
    }
}

