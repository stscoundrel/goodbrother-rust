mod response;
use response::SearchResponse;

fn parse_request_url(username: &str) -> String {
    format!("https://api.github.com/search/issues?q=user:{user}+is:pr+state:open",
        user = username,)
}

fn get_client() -> reqwest::blocking::Client {
    let client = reqwest::blocking::Client::builder()
    .user_agent("GOODBROTHER_RUST")
    .build()
    .unwrap();

    return client;
}

fn get_pull_request_body(username: String) -> String {
    let client = get_client();
    let request_url = parse_request_url(&username);

    let body = client.get(request_url).send().unwrap().text().unwrap();

    return body;
}

pub fn get_pull_request_response(username: String) -> SearchResponse {
    let body = get_pull_request_body(username);
    
    let pull_requests = serde_json::from_str(&body).unwrap();

    pull_requests
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
        let result = get_pull_request_response("stscoundrel".to_string());

        println!("{}", result.total_count);

        assert_eq!(result.total_count, 17);
        assert_eq!(result.items.len(), 17);
        assert_eq!(result.items[0].title, "Fixture PR for integration tests");
    }
}