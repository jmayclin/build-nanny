use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_codebuild::operation::list_build_batches_for_project::ListBuildBatchesForProjectOutput;
use aws_sdk_codebuild::operation::start_build_batch::StartBuildBatchOutput;
use aws_sdk_codebuild::types::BuildBatchFilter;
use aws_sdk_codebuild::Client;

pub const JOBS: [(&'static str, &'static str); 6] = [
    // Integration tests
    ("Integv2NixBatchBF1FB83F-7tcZOiMDWPH0", "us-east-2"),
    ("AddressSanitizer", "us-west-2"),
    ("S2nIntegrationV2SmallBatch", "us-west-2"),
    ("s2nFuzzBatch", "us-west-2"),
    ("s2nGeneralBatch", "us-west-2"),
    ("s2nUnitNix", "us-west-2"),
];

async fn build_client(region: &'static str) -> Client {
    let region = RegionProviderChain::first_try(region);

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .load()
        .await;

    Client::new(&config)
}

/// https://docs.aws.amazon.com/codebuild/latest/APIReference/API_StartBuildBatch.html
pub async fn start_codebuild_project(
    region: &'static str,
    project_name: &str,
    source_version: &str,
) -> Result<StartBuildBatchOutput, aws_sdk_codebuild::Error> {
    let client = build_client(region).await;

    let request = client
        .start_build_batch()
        .project_name(project_name)
        .source_version(source_version)
        .send()
        .await?;

    Ok(request)
}

/// https://docs.aws.amazon.com/codebuild/latest/APIReference/API_ListBuildBatchesForProject.html
pub async fn find_failed_batch(
    region: &'static str,
    project_name: &str,
    //source_version: &str,
) -> Result<Option<Vec<String>>, aws_sdk_codebuild::Error> {
    let client = build_client(region).await;

    let request = client
        .list_build_batches_for_project()
        .project_name(project_name)
        .filter(
            BuildBatchFilter::builder()
                .set_status(Some(aws_sdk_codebuild::types::StatusType::Failed))
                .build(),
        )
        .send()
        .await?;

    Ok(request.ids)
}

/// https://docs.aws.amazon.com/codebuild/latest/APIReference/API_BatchGetBuildBatches.html
pub async fn filter_to_project(
    ids: Vec<String>,
    region: &'static str,
    source_version: &str,
) -> Result<Option<String>, aws_sdk_codebuild::Error> {
    let client = build_client(region).await;

    let request = client.batch_get_build_batches().set_ids(Some(ids)).send().await?;
    let batches = match request.build_batches {
        Some(batches) => batches,
        None => {
            println!("no batches were found");
            return Ok(None);
        }
    };
    // I'm assuming that this gives the most recent result
    let build = batches
        .into_iter()
        .filter(|build| build.source_version == Some(source_version.to_owned()))
        .map(|b| b.id.unwrap())
        .next();
    return Ok(build)
}

// https://docs.aws.amazon.com/codebuild/latest/APIReference/API_RetryBuildBatch.html
// pub async fn retry_failed_builds(
//     id: String,
//     region: &'static str,
// ) -> Result<
