use std::error::Error;
use reqwest::blocking::{Client, Response};

mod response;
use response::SearchResponse;

const GOODBROTHER_USER_AGENT: &str = "GOODBROTHER_RUST";

fn parse_request_url(username: &str) -> String {
    format!("https://api.github.com/search/issues?q=user:{user}+is:pr+state:open",
        user = username,)
}

fn get_client() -> Result<Client, Box<dyn Error>> {
    let client = Client::builder()
    .user_agent(GOODBROTHER_USER_AGENT)
    .build()?;

    Ok(client)
}

fn get_response(request_url: String) -> Result<Response, Box<dyn Error>> {
    let client = get_client()?;
    let response = client.get(request_url).send()?;

    Ok(response)
}

pub fn get_pull_request_response(username: String) -> Result<SearchResponse, Box<dyn Error>> {
    let request_url = parse_request_url(&username);
    let response = get_response(request_url)?;
    let body = response.text()?;
    
    let pull_requests = serde_json::from_str(&body)?;

    Ok(pull_requests)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request_url() {
        let result1 = parse_request_url("stscoundrel");
        let result2 = parse_request_url("someoneelse");

        assert_eq!(result1, "https://api.github.com/search/issues?q=user:stscoundrel+is:pr+state:open");
        assert_eq!(result2, "https://api.github.com/search/issues?q=user:someoneelse+is:pr+state:open");
    }

    #[test]
    fn gets_pull_request_response() {
        let result = get_pull_request_response("stscoundrel".to_string()).unwrap();

        println!("{}", result.total_count);

        assert_eq!(result.total_count, 17);
        assert_eq!(result.items.len(), 17);
        assert_eq!(result.items[0].title, "Fixture PR for integration tests");
        assert_eq!(result.items[0].user.login, "stscoundrel");
    }
}