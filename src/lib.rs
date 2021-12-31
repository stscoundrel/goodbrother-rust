use std::error::Error;

mod client;
mod github;

use client::{get_pull_request_response, API_URL};
use github::{from_responses, to_repository_summary};
pub use github::{Repository, PullRequest};

/// List open PRs in Github by user
/// 
/// One list item per PR.
///
///
pub fn get_pull_requests_by_user(username: &str) -> Result<Vec<PullRequest>, Box<dyn Error>> {
    let pull_request_response = get_pull_request_response(username, API_URL)?;

    Ok(from_responses(pull_request_response.items))
}

/// List open PRs in Github by user, grouped by repository
/// 
/// One list item per repository that has pull requests.
///
///
pub fn get_grouped_pull_requests_by_user(username: &str) -> Result<Vec<Repository>, Box<dyn Error>> {
    let pull_request_response = get_pull_request_response(username, API_URL)?;
    let pull_requests = from_responses(pull_request_response.items);

    Ok(to_repository_summary(pull_requests))
}