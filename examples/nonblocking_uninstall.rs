use helm_wrapper_rs::nonblocking::{DefaultHelmExecutor, HelmExecutor};
use non_blank_string_rs::NonBlankString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let helm_executor = DefaultHelmExecutor::new();

    let namespace: NonBlankString = "whoami".parse().unwrap();
    // This release name should match the one used in examples/nonblocking_install_or_upgrade.rs
    let release_name: NonBlankString = "whoami-release".parse().unwrap();

    println!(
        "Attempting to uninstall release '{}' from namespace '{}'...",
        release_name, namespace
    );

    helm_executor.uninstall(&namespace, &release_name).await?;

    println!(
        "Successfully uninstalled release '{}' from namespace '{}'",
        release_name, namespace
    );

    Ok(())
}
