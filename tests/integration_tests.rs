use goodbrother::{get_pull_requests_by_user};
use goodbrother::{PullRequest};

#[test]
fn gets_pull_requess_by_user() {
    let username = "stscoundrel".to_string();
    let result = get_pull_requests_by_user(username).unwrap();

    assert!(result.len() > 0);

    for repo in result.iter() {
        println!("{}", repo.repository);
    }

    // Should always contain test PR for Goodbrother.
    let goodbrother_prs: Vec<PullRequest> = result
        .iter()
        .filter(|pr| pr.repository.eq("stscoundrel/goodbrother"))
        .map(|pr| pr.clone())
        .collect();

    assert!(goodbrother_prs.len() > 0);

    assert!(goodbrother_prs[0].name.eq("Fixture PR for integration tests"));
    assert!(goodbrother_prs[0].link.eq("https://github.com/stscoundrel/goodbrother/pull/18"));
    assert_eq!(goodbrother_prs[0].is_dependabot, false);
}