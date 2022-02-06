use std::error::Error;
use reqwest::blocking::{Client, Response};

const GOODBROTHER_USER_AGENT: &str = "GOODBROTHER_RUST";

pub fn parse_request_url(username: &str, api_url: &str, page: i8) -> String {
    format!("{api}/search/issues?q=user:{user}+is:pr+state:open&per_page=100&page={page}",
        api = api_url,
        user = username,
        page = page)
}

fn get_client() -> Result<Client, Box<dyn Error>> {
    let client = Client::builder()
    .user_agent(GOODBROTHER_USER_AGENT)
    .build()?;

    Ok(client)
}

pub fn get_response(request_url: String) -> Result<Response, Box<dyn Error>> {
    let client = get_client()?;
    let response = client.get(request_url).send()?;

    Ok(response)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request_url() {
        let result1 = parse_request_url("stscoundrel", "https://api.github.com", 1);
        let result2 = parse_request_url("someoneelse", "https://api.github.com", 2);

        assert_eq!(result1, "https://api.github.com/search/issues?q=user:stscoundrel+is:pr+state:open&per_page=100&page=1");
        assert_eq!(result2, "https://api.github.com/search/issues?q=user:someoneelse+is:pr+state:open&per_page=100&page=2");
    }
}