use archive_is::ArchiveSesh;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    let url_to_archive = "https://github.com/UnsafeOats/shrtcut/tree/master/src";

    match ArchiveSesh::new().await {
        Ok(archive_sesh) => {
            match archive_sesh.wait_for_archive(url_to_archive, Duration::from_secs(15), 80).await {
                Ok(Some(location)) => println!("Archived URL location: {}", location),
                Ok(None) => println!("No location or refresh field found in the response."),
                Err(e) => eprintln!("Error archiving URL: {}", e),
            }
        }
        Err(e) => eprintln!("Error creating ArchiveSesh: {}", e),
    }
}
