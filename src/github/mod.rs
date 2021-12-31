mod pull_request;

use crate::client::PullRequestResponse;
use pull_request::PullRequest;

pub fn from_response(pr: &PullRequestResponse) -> PullRequest {
    return PullRequest {
        id: pr.id,
        name: pr.title.to_string(),
        link: pr.html_url.to_string(),
        is_dependabot: pr.user.login.contains("dependabot"),
        repository: pr.repository_url.to_string(),
    };
}

pub fn from_responses(prs: Vec<PullRequestResponse>) -> Vec<PullRequest> {
    let  pull_requests: Vec<PullRequest> = prs.iter().map(|pr| from_response(pr)).collect();

    pull_requests
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{PullRequestResponse, PullRequestUser};

    #[test]
    fn transforms_pr_responses_to_prs() {
        let pull_request_1 = PullRequestResponse {
            id: 1234,
            title: "First pr".to_string(),
            user: PullRequestUser {
                login: "stscoundrel".to_string()
            },
            html_url: "https://github.com/stscoundrel/goodbrother-rust/pulls/666".to_string(),
            repository_url: "https://github.com/stscoundrel/goodbrother-rust".to_string()
        };

        let pull_request_2 = PullRequestResponse {
            id: 5431,
            title: "Second pr".to_string(),
            user: PullRequestUser {
                login: "dependabot".to_string()
            },
            html_url: "https://github.com/stscoundrel/goodbrother-rust/pulls/667".to_string(),
            repository_url: "https://github.com/stscoundrel/goodbrother-rust".to_string()
        };

        let pull_requests = vec![pull_request_1.clone(), pull_request_2.clone()];

        let result = from_responses(pull_requests);

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].id, pull_request_1.id);
        assert_eq!(result[0].name, pull_request_1.title);
        assert_eq!(result[0].link, pull_request_1.html_url);
        assert_eq!(result[0].repository, pull_request_1.repository_url);
        assert_eq!(result[0].is_dependabot, false);

        assert_eq!(result[1].id, pull_request_2.id);
        assert_eq!(result[1].name, pull_request_2.title);
        assert_eq!(result[1].link, pull_request_2.html_url);
        assert_eq!(result[1].repository, pull_request_2.repository_url);
        assert_eq!(result[1].is_dependabot, true);
    }
}