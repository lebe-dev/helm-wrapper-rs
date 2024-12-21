use log::LevelFilter;
use non_blank_string_rs::NonBlankString;

pub fn get_test_namespace() -> NonBlankString {
    "whoami".parse().unwrap()
}

pub fn get_test_release_name() -> NonBlankString {
    "whoami".parse().unwrap()
}

pub fn get_test_chart_name() -> NonBlankString {
    "cowboysysop/whoami".parse().unwrap()
}

pub fn get_test_helm_options() -> Vec<NonBlankString> {
    vec!["--create-namespace".parse().unwrap()]
}

pub fn init_logging() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .is_test(true)
        .try_init();
}
