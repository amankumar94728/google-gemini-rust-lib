use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    
    message: String,
}

pub async fn get_gemini_data() -> Result<ApiResponse, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://api.google.dev/competition") // Replace with the actual API URL
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio; 

    #[tokio::test]
    async fn test_get_gemini_data() {

        let result = get_gemini_data().await;

        match result {
            Ok(data) => {
                println!("{:?}", data);

                assert!(data.message.contains("expected substring"));
            }
            Err(e) => panic!("API call failed: {:?}", e),
        }
    }
}
