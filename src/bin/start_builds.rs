use codebuild_launcher::start_codebuild_project;


#[tokio::main]
async fn main() {
    let pr_number = 4611;
    let source_version = format!("pr/{}", pr_number);

    for (project_name, region) in codebuild_launcher::JOBS {
        match start_codebuild_project(region, project_name, &source_version).await {
            Ok(build_id) => {
                println!(
                    "Started build for project {} in region {}",
                    project_name, region
                );
                println!("Build Info: {:?}", build_id);
            }
            Err(e) => {
                eprintln!(
                    "Failed to start build for project {} in region {}: {:?}",
                    project_name, region, e
                );
            }
        }
    }
}
