use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_codebuild::operation::start_build_batch::StartBuildBatchOutput;
use aws_sdk_codebuild::Client;
use std::collections::HashMap;

const JOBS: [(&'static str, &'static str); 6] = [
    // Integration tests
    ("Integv2NixBatchBF1FB83F-7tcZOiMDWPH0", "us-east-2"),
    ("AddressSanitizer", "us-west-2"),
    ("S2nIntegrationV2SmallBatch", "us-west-2"),
    ("s2nFuzzBatch", "us-west-2"),
    ("s2nGeneralBatch", "us-west-2"),
    ("s2nUnitNix", "us-west-2"),
];

async fn start_codebuild_project(
    region: &'static str,
    project_name: &str,
    source_version: &str,
) -> Result<StartBuildBatchOutput, aws_sdk_codebuild::Error> {
    let region = RegionProviderChain::first_try(region);

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .load()
        .await;

    let client = Client::new(&config);

    let request = client
        .start_build_batch()
        .project_name(project_name)
        .source_version(source_version)
        .send()
        .await?;

    Ok(request)
}

#[tokio::main]
async fn main() {
    let pr_number = 4611;
    let source_version = format!("pr/{}", pr_number);

    for (project_name, region) in JOBS {
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
