use std::env;
use std::time::Duration;

pub fn post_issue(owner: &str, repo_name: &str, issue_id: usize, message: &String) {
    println!("{}/{}", owner, repo_name);
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not defined.");
    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    let send_string = format!("{{\"body\" : \"{}\" }}", message);
    let response = agent.post(
        format!(
            "https://api.github.com/repos/{}/{}/issues/{}/comments",
            owner,
            repo_name,
            issue_id
        ).as_str())
        .set("Accept", "application/vnd.github.v3+json")
        .set("Authorization", format!("token {}", token).as_str())
        .send_string(send_string.as_str());
    println!("{}", response.is_ok());
}
