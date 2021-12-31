use std::error::Error;
use reqwest::blocking::{Client, Response};

mod response;
pub use response::{SearchResponse, PullRequestResponse, PullRequestUser};

pub const API_URL: &str = "https://api.github.com";
const GOODBROTHER_USER_AGENT: &str = "GOODBROTHER_RUST";

fn parse_request_url(username: &str, api_url: &str) -> String {
    format!("{api}/search/issues?q=user:{user}+is:pr+state:open",
        api = api_url,
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

pub fn get_pull_request_response(username: String, api_url: &str) -> Result<SearchResponse, Box<dyn Error>> {
    let request_url = parse_request_url(&username, api_url);
    let response = get_response(request_url)?;
    let body = response.text()?;
    
    let pull_requests = serde_json::from_str(&body)?;

    Ok(pull_requests)
}


#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[test]
    fn parses_request_url() {
        let result1 = parse_request_url("stscoundrel", "https://api.github.com");
        let result2 = parse_request_url("someoneelse", "https://api.github.com");

        assert_eq!(result1, "https://api.github.com/search/issues?q=user:stscoundrel+is:pr+state:open");
        assert_eq!(result2, "https://api.github.com/search/issues?q=user:someoneelse+is:pr+state:open");
    }

    #[test]
    fn gets_pull_request_response() {
        // Setup mock response for API to send.
        let mock_response = json!(
            {
                "total_count": 2,
                "items": [
                    {
                        "id": 123456,
                        "title": "Unit Test PR",
                        "user": {
                            "login": "RustyMock",
                        },
                        "html_url": "https://github.com/stscoundrel/goodbrother-rust/pulls/666",
                        "repository_url": "https://github.com/stscoundrel/goodbrother-rust",
                    },
                    {
                        "id": 654321,
                        "title": "Secondary Unit Test PR",
                        "user": {
                            "login": "RustyMock",
                        },
                        "html_url": "https://github.com/stscoundrel/goodbrother-rust/pulls/667",
                        "repository_url": "https://github.com/stscoundrel/goodbrother-rust",
                    }
                ]
            }
        );


        let server = MockServer::start();
        let github_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/search/issues")
                .query_param("q", "user:stscoundrel+is:pr+state:open");
            then.status(200)
                .header("content-type", "text/html")
                .json_body(mock_response);
        });

        let result = get_pull_request_response("stscoundrel".to_string(), &server.base_url()).unwrap();

        // Assert JSON reply was parsed.
        assert_eq!(result.total_count, 2);
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.items[0].title, "Unit Test PR");
        assert_eq!(result.items[0].user.login, "RustyMock");
        assert_eq!(result.items[0].html_url, "https://github.com/stscoundrel/goodbrother-rust/pulls/666");
        assert_eq!(result.items[0].repository_url, "https://github.com/stscoundrel/goodbrother-rust");

        // Assert it came from the mock server.
        github_mock.assert()
    }
}