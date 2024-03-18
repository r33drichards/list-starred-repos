use serde::{Deserialize, Serialize};

use gitlab::Gitlab;
use gitlab::api::{self, projects::ProjectsBuilder, Query};



#[derive(Debug, Serialize, Deserialize)]
struct Project {
    id: u64,
    http_url_to_repo: String,
}

 fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let gitlab_access_token = env::var("GITLAB_ACCESS_TOKEN").expect("GITLAB_ACCESS_TOKEN must be set");
    // read from /GITLAB_ACCESS_TOKEN
    let gitlab_access_token = std::fs::read_to_string("/GITLAB_ACCESS_TOKEN")?
        .trim() // This will remove the newline character
        .to_string();
    let gitlab_api_endpoint = "gitlab.com";

    let client = Gitlab::new(gitlab_api_endpoint, gitlab_access_token).expect("Failed to create Gitlab client");


    let pageable_endpoint = ProjectsBuilder::default().starred(true).owned(true).build().unwrap();
    let projects: Vec<Project> = api::paged(pageable_endpoint, api::Pagination::Limit(200)).query(&client).unwrap();

    for project in &projects {
        println!("{}", project.http_url_to_repo);
    }

    Ok(())
}
