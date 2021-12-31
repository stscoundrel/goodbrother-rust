use serde::{Deserialize, Serialize};
use crate::client::PullRequestResponse;

/// Pull request in Github
/// 
/// Dataset for base information of PR.
///
#[derive(Serialize, Deserialize, Clone)]
pub struct PullRequest {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub is_dependabot: bool,
    pub repository: String,
}

pub fn from_response(pr: &PullRequestResponse) -> PullRequest {
    PullRequest {
        id: pr.id,
        name: pr.title.to_string(),
        link: pr.html_url.to_string(),
        is_dependabot: pr.user.login.contains("dependabot"),
        repository: pr.repository_url.replace("https://api.github.com/repos/", ""),
    }
}

pub fn from_responses(prs: Vec<PullRequestResponse>) -> Vec<PullRequest> {
    let pull_requests: Vec<PullRequest> = prs.iter().map(from_response).collect();

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
            repository_url: "https://api.github.com/repos/stscoundrel/goodbrother-rust".to_string()
        };

        let pull_request_2 = PullRequestResponse {
            id: 5431,
            title: "Second pr".to_string(),
            user: PullRequestUser {
                login: "dependabot".to_string()
            },
            html_url: "https://github.com/stscoundrel/goodbrother-rust/pulls/667".to_string(),
            repository_url: "https://api.github.com/repos/stscoundrel/goodbrother-rust".to_string()
        };

        let pull_requests = vec![pull_request_1.clone(), pull_request_2.clone()];

        let result = from_responses(pull_requests);

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].id, pull_request_1.id);
        assert_eq!(result[0].name, pull_request_1.title);
        assert_eq!(result[0].link, pull_request_1.html_url);
        assert_eq!(result[0].repository, "stscoundrel/goodbrother-rust");
        assert_eq!(result[0].is_dependabot, false);

        assert_eq!(result[1].id, pull_request_2.id);
        assert_eq!(result[1].name, pull_request_2.title);
        assert_eq!(result[1].link, pull_request_2.html_url);
        assert_eq!(result[1].repository, "stscoundrel/goodbrother-rust");
        assert_eq!(result[1].is_dependabot, true);
    }
}