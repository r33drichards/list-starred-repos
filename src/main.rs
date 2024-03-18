use serde::{Deserialize, Serialize};

use gitlab::Gitlab;
use gitlab::api::{self, projects::ProjectsBuilder, Query};

use octocrab::{models, Octocrab, Page};



#[derive(Debug, Serialize, Deserialize)]
struct Project {
    id: u64,
    http_url_to_repo: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Spawn a blocking task for the synchronous `gl_stars` function
    tokio::task::spawn_blocking(|| {
        gl_stars();
    }).await?;
    gh_stars().await;



    Ok(())
}

fn gl_stars(){
        // let gitlab_access_token = env::var("GITLAB_ACCESS_TOKEN").expect("GITLAB_ACCESS_TOKEN must be set");
    // read from /GITLAB_ACCESS_TOKEN
    let gitlab_access_token = std::fs::read_to_string("/GITLAB_ACCESS_TOKEN").expect("gitlab")
        .trim() // This will remove the newline character
        .to_string();
    let gitlab_api_endpoint = "gitlab.com";

    let client = Gitlab::new(gitlab_api_endpoint, gitlab_access_token).expect("Failed to create Gitlab client");


    let pageable_endpoint = ProjectsBuilder::default().starred(true).owned(true).build().unwrap();
    let projects: Vec<Project> = api::paged(pageable_endpoint, api::Pagination::Limit(200)).query(&client).unwrap();

    for project in &projects {
        println!("{}", project.http_url_to_repo);
    }
}


async fn gh_stars (){
    let gitlab_access_token = std::fs::read_to_string("/GITHUB_ACCESS_TOKEN").expect("msg")
    .trim() // This will remove the newline character
    .to_string();

    let octocrab = Octocrab::builder()
    .personal_token(gitlab_access_token)
    .build().unwrap();


    let mut count = 0;
    let mut n = 0;
    let mut page = octocrab.current().list_repos_starred_by_authenticated_user().send().await.unwrap();
    while let current_page = page {
        for repo in &current_page.items {
            if let Some(u) = &repo.clone_url {
                println!("github.com{}", u.path())

            }
            count += 1;
            if count >= 200 {
                break;
            }
        }

        if count >= 200 {
            break;
        }
        n += 1;

        let p =  octocrab.current().list_repos_starred_by_authenticated_user().page(n).send().await;

        match p {
            Ok(p) => page = p,
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
        
    }

}