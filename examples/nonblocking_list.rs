use helm_wrapper_rs::nonblocking::{DefaultHelmExecutor, HelmExecutor};
use helm_wrapper_rs::HelmListItem;
use non_blank_string_rs::NonBlankString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let helm_executor = DefaultHelmExecutor::new();

    // Example: List releases in "whoami" namespace
    let namespace_str: NonBlankString = "whoami".parse().unwrap();
    let releases: Vec<HelmListItem> = helm_executor.list(Some(&namespace_str)).await?;
    println!("Found releases in namespace '{}': {:?}", namespace_str, releases);

    // You can also call other methods like install_or_upgrade or uninstall
    // For example, to uninstall a release (ensure you have one to uninstall or this will fail):
    // let namespace_to_uninstall: NonBlankString = "your-namespace".parse().unwrap();
    // let release_to_uninstall: NonBlankString = "your-release".parse().unwrap();
    // helm_executor.uninstall(&namespace_to_uninstall, &release_to_uninstall).await?;
    // println!("Successfully uninstalled {} from {}", release_to_uninstall, namespace_to_uninstall);

    Ok(())
}
