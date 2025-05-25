use helm_wrapper_rs::blocking::{DefaultHelmExecutor, HelmExecutor};
use non_blank_string_rs::NonBlankString;
use log::LevelFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger to see detailed error output
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Info) // Or LevelFilter::Debug for more details
        .try_init();

    let helm_executor = DefaultHelmExecutor::new();

    let namespace: NonBlankString = "whoami".parse().unwrap();
    // This release name should match the one used in examples/blocking_install_or_upgrade.rs
    let release_name: NonBlankString = "whoami-release-blocking".parse().unwrap();

    println!(
        "Attempting to uninstall release '{}' from namespace '{}'...",
        release_name, namespace
    );

    helm_executor.uninstall(&namespace, &release_name)?;

    println!(
        "Successfully uninstalled release '{}' from namespace '{}'",
        release_name, namespace
    );

    Ok(())
}
