# Goodbrother

List open PRs in Github by user. Rust port of original TypeScript library.

## Motivation

If you have many repos that receive regular updades with Dependabot, you're simply likely to miss some of them. I occasionally only found PRs when I get notification that they were closed in favor of a even newer version.

Goodbrother is there to let me know if I still have some open.

### Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
goodbrother = "1.0.0"
```

### Usage

Listing all pull requests:

```rust
use goodbrother::{get_pull_requests_by_user, PullRequest};

// Fetch list of open PRs by user.
let username = "stscoundrel";

// Returns Result, which may be error due to Github API connections.
let result = get_pull_requests_by_user(username).unwrap();

// Result is a vector of PullRequest structs. Eg:
// {
//     id: 1068208284,
//     name: Bump eslint-config-airbnb-base from 14.2.1 to 15.0.0,
//     link: https://github.com/stscoundrel/gatsby-source-plugin-zoega/pull/18,
//     is_dependabot: true,
//     repository: stscoundrel/gatsby-source-plugin-zoega,
// }
```

Listing pull requests grouped by repos:

```rust
use goodbrother::{get_grouped_pull_requests_by_user, Repository};

// Fetch list of open PRs by user.
let username = "stscoundrel";

// Returns Result, which may be error due to Github API connections.
let result = get_grouped_pull_requests_by_user(username).unwrap();

// Result is a vector of Repository structs. Eg:
// {
//     name: goodbrother
//     pull_requests: PullRequest[],
// }
```