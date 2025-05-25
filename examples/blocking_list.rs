use helm_wrapper_rs::blocking::{DefaultHelmExecutor, HelmExecutor};
use helm_wrapper_rs::HelmListItem;
use non_blank_string_rs::NonBlankString;
use log::LevelFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger to see detailed error output from the helm-wrapper-rs library
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .try_init();

    let helm_executor = DefaultHelmExecutor::new();

    // Example: List releases in "whoami" namespace
    let namespace_str: NonBlankString = "whoami".parse().unwrap();
    let releases: Vec<HelmListItem> = helm_executor.list(Some(&namespace_str))?;
    println!("Found releases in namespace '{}': {:?}", namespace_str, releases);

    // Example: Uninstall a release
    // let namespace_str: NonBlankString = "default".parse().unwrap();
    // let release_name_str: NonBlankString = "my-release".parse().unwrap();
    // helm_executor.uninstall(&namespace_str, &release_name_str)?;

    // Example: Install or upgrade a chart
    // let namespace_install: NonBlankString = "my-namespace".parse().unwrap();
    // let release_install: NonBlankString = "my-app".parse().unwrap();
    // let chart_install: NonBlankString = "nginx/nginx".parse().unwrap();
    // helm_executor.install_or_upgrade(
    //     &namespace_install,
    //     &release_install,
    //     &chart_install,
    //     None, // chart_version
    //     None, // values_overrides
    //     None, // values_file
    //     None, // helm_options
    // )?;

    Ok(())
}
