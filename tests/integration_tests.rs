use goodbrother::{get_pull_requests_by_user, get_grouped_pull_requests_by_user};
use goodbrother::{PullRequest, Repository};

#[test]
fn gets_pull_requets_by_user() {
    let username = "stscoundrel";
    let result = get_pull_requests_by_user(username).unwrap();

    assert!(result.len() > 0);

    // Should always contain test PR for Goodbrother.
    let goodbrother_prs: Vec<&PullRequest> = result
        .iter()
        .filter(|pr| pr.repository.eq("stscoundrel/goodbrother-rust"))
        .collect();

    assert!(goodbrother_prs.len() > 0);

    assert!(goodbrother_prs.last().unwrap().name.eq("Fixture PR for integration tests"));
    assert!(goodbrother_prs.last().unwrap().link.eq("https://github.com/stscoundrel/goodbrother-rust/pull/16"));
    assert_eq!(goodbrother_prs.last().unwrap().is_dependabot, false);
}

#[test]
fn gets_grouped_pull_requests_by_user() {
    let username = "stscoundrel";
    let result = get_grouped_pull_requests_by_user(username).unwrap();

    assert!(result.len() > 0);

    // Should always contain prs for Goodbrother
    let goodbrother_prs: Vec<&Repository> = result
        .iter()
        .filter(|repo| repo.name.eq("stscoundrel/goodbrother-rust"))
        .collect();

    assert!(goodbrother_prs.len() > 0);

    assert!(goodbrother_prs[0].name.eq("stscoundrel/goodbrother-rust"));
    assert!(goodbrother_prs[0].pull_requests.last().unwrap().name.eq("Fixture PR for integration tests"));
    assert!(goodbrother_prs[0].pull_requests.last().unwrap().link.eq("https://github.com/stscoundrel/goodbrother-rust/pull/16"));
    assert_eq!(goodbrother_prs[0].pull_requests.last().unwrap().is_dependabot, false);
}