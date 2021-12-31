use serde::{Deserialize, Serialize};

/// Pull request in Github
/// 
/// Dataset for base information of PR.
///
#[derive(Serialize, Deserialize)]
pub struct PullRequest {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub is_dependabot: bool,
    pub repository: String,
}