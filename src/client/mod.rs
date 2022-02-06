use std::error::Error;

mod api;
mod response;
pub use response::{SearchResponse, PullRequestResponse, PullRequestUser};

pub const API_URL: &str = "https://api.github.com";
const MAX_REQUESTS: i8 = 5;

pub fn get_pull_request_response(username: &str, api_url: &str) -> Result<SearchResponse, Box<dyn Error>> {
    let mut requests = 1;
    let mut has_more_results = true;
    let mut result = SearchResponse {
        total_count: 0,
        items: vec![],
    };

    while has_more_results && requests <= MAX_REQUESTS {
        let request_url = api::parse_request_url(username, api_url, requests);
        let response = api::get_response(request_url)?;
        let body = response.text()?;
        
        let mut pull_requests: SearchResponse = serde_json::from_str(&body)?;

        result.total_count = pull_requests.total_count;
        result.items.append(&mut pull_requests.items);

        if result.items.len() as i32 >= pull_requests.total_count {
            has_more_results = false;
        }

        requests += 1;
    }

    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

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
                        "html_url": "https://github.com/stscoundrel/goodbrother-rs/pulls/666",
                        "repository_url": "https://github.com/stscoundrel/goodbrother-rs",
                    },
                    {
                        "id": 654321,
                        "title": "Secondary Unit Test PR",
                        "user": {
                            "login": "RustyMock",
                        },
                        "html_url": "https://github.com/stscoundrel/goodbrother-rs/pulls/667",
                        "repository_url": "https://github.com/stscoundrel/goodbrother-rs",
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

        let result = get_pull_request_response("stscoundrel", &server.base_url()).unwrap();

        // Assert JSON reply was parsed.
        assert_eq!(result.total_count, 2);
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.items[0].title, "Unit Test PR");
        assert_eq!(result.items[0].user.login, "RustyMock");
        assert_eq!(result.items[0].html_url, "https://github.com/stscoundrel/goodbrother-rs/pulls/666");
        assert_eq!(result.items[0].repository_url, "https://github.com/stscoundrel/goodbrother-rs");

        // Assert it came from the mock server.
        github_mock.assert()
    }

    #[test]
    fn paginates_pull_request_responses() {
        // Setup mock response for API to send.
        let individual_pr = json!(
            {
                "id": 123456,
                "title": "Unit Test PR",
                "user": {
                    "login": "RustyMock",
                },
                "html_url": "https://github.com/stscoundrel/goodbrother-rs/pulls/666",
                "repository_url": "https://github.com/stscoundrel/goodbrother-rs",
            }
        );

        let mut hundred_prs = vec![];

        for _ in 0..100 {
            hundred_prs.push(&individual_pr)
        }

        let mock_response = json!(
            {
                "total_count": 203,
                "items": hundred_prs
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

        let result = get_pull_request_response("stscoundrel", &server.base_url()).unwrap();

        // Assert JSON reply was parsed.
        assert_eq!(result.total_count, 203);

        // Due to total_count of 203, it should take three requests to get "all"
        // As responses use same fixture, final number will be 300 instead of real 203.
        assert_eq!(result.items.len(), 300);

        // Assert we called search API three times
        github_mock.assert_hits(3);
    }
}