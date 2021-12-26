use serde::{Deserialize, Serialize};

/// Pull request in Github
/// 
/// Dataset for base information of PR.
///
#[derive(Serialize, Deserialize)]
pub struct PullRequest {
    pub id: String,
    pub name: String,
    pub link: String,
    pub is_dependabot: boolean,
    pub repository: String,
}