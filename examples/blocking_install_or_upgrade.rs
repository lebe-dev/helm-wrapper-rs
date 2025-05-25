use helm_wrapper_rs::blocking::{DefaultHelmExecutor, HelmExecutor};
use helm_wrapper_rs::HelmDeployStatus;
use non_blank_string_rs::NonBlankString;
use std::collections::HashMap; // If you need to provide overrides
use log::LevelFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger to see detailed error output
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Info) // Or LevelFilter::Debug for more details
        .try_init();

    let helm_executor = DefaultHelmExecutor::new();

    let namespace: NonBlankString = "whoami".parse().unwrap();
    let release_name: NonBlankString = "whoami-release-blocking".parse().unwrap();
    let chart_name: NonBlankString = "cowboysysop/whoami".parse().unwrap();
    // Optionally, specify chart version, values, etc.
    // let chart_version: Option<&NonBlankString> = Some(&"1.0.0".parse().unwrap());
    // let mut values_overrides = HashMap::new();
    // values_overrides.insert("replicaCount".parse().unwrap(), "2".to_string());

    let status: HelmDeployStatus = helm_executor.install_or_upgrade(
        &namespace,
        &release_name,
        &chart_name,
        None, // chart_version
        None, // values_overrides
        None, // values_file
        None, // helm_options (e.g., Some(&vec!["--create-namespace".parse().unwrap()]))
    )?;

    println!(
        "Install/Upgrade of chart '{}' with release name '{}' in namespace '{}' finished with status: {:?}",
        chart_name, release_name, namespace, status
    );

    Ok(())
}
