use std::collections::HashSet;

use crate::github::pull_request::PullRequest;
use serde::{Deserialize, Serialize};

/// Repository in Github
/// 
/// Dataset for summary information of repository PRs.
///
#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub link: String,
    pub count: usize,
    pub pull_requests: Vec<PullRequest>,
}

pub fn to_repository_summary(prs: Vec<PullRequest>) -> Vec<Repository> {
    let mut repositories = HashSet::new();

    for pr in prs.iter() {
        repositories.insert(pr.repository.to_string());
    }

    repositories.iter().map(|repo| {
        let repo_prs: Vec<PullRequest> = prs
            .clone()
            .into_iter()
            .filter(|pr| pr.repository.eq(repo))
            .collect();

        Repository {
            name: repo.to_string(),
            link: "https://github.com/".to_string() + repo,
            count: repo_prs.len(),
            pull_requests: repo_prs
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_pull_requests_to_repository_summary() {
        let pull_request_1 = PullRequest {
            id: 1234,
            name: "First pr".to_string(),
            link: "https://github.com/stscoundrel/goodbrother-rs/pulls/666".to_string(),
            repository: "stscoundrel/goodbrother-rs".to_string(),
            is_dependabot: false
        };

        let pull_request_2 = PullRequest {
            id: 4321,
            name: "Second pr".to_string(),
            link: "https://github.com/stscoundrel/goodbrother-rs/pulls/667".to_string(),
            repository: "stscoundrel/goodbrother-rs".to_string(),
            is_dependabot: false
        };

        let pull_request_3 = PullRequest {
            id: 987654321,
            name: "Third pr".to_string(),
            link: "https://github.com/stscoundrel/goodbrother/pulls/668".to_string(),
            repository: "stscoundrel/goodbrother".to_string(),
            is_dependabot: true
        };

        // Rust does not seem to quarantee order of items in vector.
        let pull_requests = vec![pull_request_1.clone(), pull_request_2.clone(), pull_request_3.clone()];

        let result = to_repository_summary(pull_requests);

        assert_eq!(result.len(), 2);

        let repo1: Vec<&Repository> = result
            .iter()
            .filter(|repo| repo.name.eq("stscoundrel/goodbrother-rs"))
            .collect();

        let repo2: Vec<&Repository> = result
            .iter()
            .filter(|repo| repo.name.eq("stscoundrel/goodbrother"))
            .collect();

        assert_eq!(repo1[0].pull_requests.len(), 2);
        assert_eq!(repo1[0].count, 2);
        assert_eq!(repo1[0].link, "https://github.com/stscoundrel/goodbrother-rs");

        assert_eq!(repo2[0].pull_requests.len(), 1);
        assert_eq!(repo2[0].count, 1);
        assert_eq!(repo2[0].link, "https://github.com/stscoundrel/goodbrother");

        assert_eq!(repo1[0].pull_requests[0].id, pull_request_1.id);
        assert_eq!(repo1[0].pull_requests[0].name, pull_request_1.name);
        assert_eq!(repo1[0].pull_requests[0].link, pull_request_1.link);
        assert_eq!(repo1[0].pull_requests[0].repository, pull_request_1.repository);
        assert_eq!(repo1[0].pull_requests[0].is_dependabot, false);

        assert_eq!(repo1[0].pull_requests[1].id, pull_request_2.id);
        assert_eq!(repo1[0].pull_requests[1].name, pull_request_2.name);
        assert_eq!(repo1[0].pull_requests[1].link, pull_request_2.link);
        assert_eq!(repo1[0].pull_requests[1].repository, pull_request_2.repository);
        assert_eq!(repo1[0].pull_requests[1].is_dependabot, false);

        assert_eq!(repo2[0].pull_requests[0].id, pull_request_3.id);
        assert_eq!(repo2[0].pull_requests[0].name, pull_request_3.name);
        assert_eq!(repo2[0].pull_requests[0].link, pull_request_3.link);
        assert_eq!(repo2[0].pull_requests[0].repository, pull_request_3.repository);
        assert_eq!(repo2[0].pull_requests[0].is_dependabot, true);
    }
}