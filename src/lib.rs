use std::{collections::HashMap, path::Path, process::Command};

use error::HelmWrapperError;
use log::{debug, error, info};
use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

pub mod error;

#[cfg(test)]
pub mod tests;

pub trait HelmExecutor {
    /// List installed helm charts
    /// - `namespace` - namespace (optional)
    fn list(
        &self,
        namespace: Option<&NonBlankString>,
    ) -> Result<Vec<HelmListItem>, HelmWrapperError>;

    /// Install or upgrade helm chart in such way:
    /// helm upgrade --install <RELEASE-NAME> <CHART-NAME> [-v CHART-VERSION] [-f VALUES-FILE] [--set <OVERRIDE_A>=<OVERRIDE_A_VALUE>]
    /// - `namespace` - target namespace
    /// - `release_name` - release name. For example: myapp
    /// - `chart_name` - helm chart name. For example: cowboysysop/whoami
    /// - `chart_version` - helm chart version. For example: 1.2.3 (optional)
    /// - `values_overrides` - values overrides, pass to helm as --set NAME=VALUE (optional)
    /// - `values-file` - path to values file (optional)
    /// - `helm_options` - any other options for helm. for example '--dry-run' (optional)
    fn install_or_upgrade(
        &self,
        namespace: &NonBlankString,
        release_name: &NonBlankString,
        chart_name: &NonBlankString,
        chart_version: Option<&NonBlankString>,
        values_overrides: Option<&HashMap<NonBlankString, NonBlankString>>,
        values_file: Option<&Path>,
        helm_options: Option<&Vec<NonBlankString>>,
    ) -> Result<HelmUpgradeStatus, HelmWrapperError>;

    /// - `helm_options` - any other options for helm. for example '--dry-run' (optional)
    fn uninstall(
        &self,
        namespace: &NonBlankString,
        release_name: &NonBlankString,
    ) -> Result<(), HelmWrapperError>;
}

#[derive(Deserialize, Debug)]
pub struct HelmListItem {
    pub name: String,
    pub namespace: String,
    pub revision: String,
    pub updated: String,
    pub status: String,
    pub chart: String,
    pub app_version: String,
}

#[derive(Deserialize, Debug)]
pub struct HelmUpgradeResponse {
    pub info: HelmUpgradeResponseInfo,
}

#[derive(Deserialize, Debug)]
pub struct HelmUpgradeResponseInfo {
    pub status: HelmUpgradeStatus,
}

#[derive(PartialEq, Deserialize, Debug)]
pub enum HelmUpgradeStatus {
    #[serde(rename = "deployed")]
    Deployed,
    #[serde(rename = "pending-install")]
    PendingInstall,
    #[serde(rename = "pending-upgrade")]
    PendingUpgrade,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Clone, Debug)]
pub struct DefaultHelmExecutor(String, u16, bool, bool);

impl DefaultHelmExecutor {
    /// Create executor instance with predefined option values:
    /// - Helm path: helm
    /// - Timeout: 15 (secs)
    /// - Debug: false
    /// - unsafe_mode: false - print overridden values to log
    pub fn new() -> Self {
        Self("helm".to_string(), 15, false, false)
    }

    /// Create execute with options:
    /// - `helm_path` - path to helm executable
    /// - `timeout` - timeout for helm command execution
    /// - `debug` - debug mode, more verbose output from helm
    /// - `unsafe_mode` - print overridden values to log
    pub fn new_with_opts(
        helm_path: &NonBlankString,
        timeout: u16,
        debug: bool,
        unsafe_mode: bool,
    ) -> Self {
        Self(helm_path.to_string(), timeout, debug, unsafe_mode)
    }

    pub fn get_helm_path(&self) -> &str {
        &self.0
    }

    pub fn get_timeout(&self) -> u16 {
        self.1
    }

    pub fn get_debug(&self) -> bool {
        self.2
    }

    pub fn get_unsafe_mode(&self) -> bool {
        self.3
    }

    fn remove_double_spaces_and_trim(&self, input: &str) -> String {
        let result = input.replace("  ", " ");
        result.trim().to_string()
    }
}

impl HelmExecutor for DefaultHelmExecutor {
    fn list(
        &self,
        namespace: Option<&NonBlankString>,
    ) -> Result<Vec<HelmListItem>, HelmWrapperError> {
        info!("get list of installed helm charts..");

        debug!("helm executable path '{}'", self.get_helm_path());
        debug!("timeout {}s", self.get_timeout());

        let mut command_args = format!("ls");

        if let Some(namespace) = namespace {
            info!("- namespace '{namespace}'");
            command_args.push_str(&format!(" -n {} -o json ", namespace));
        }

        if self.get_debug() {
            command_args.push_str(" --debug ");
        }

        command_args = self.remove_double_spaces_and_trim(&command_args);

        let command_args: Vec<&str> = command_args.split(" ").collect();

        match Command::new(&self.get_helm_path())
            .args(command_args)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8(output.stdout)?;

                    if self.get_unsafe_mode() {
                        debug!("<stdout>");
                        debug!("{}", stdout);
                        debug!("</stdout>");
                    }

                    let helm_response: Vec<HelmListItem> = serde_json::from_str(&stdout)?;

                    info!("response: {:?}", helm_response);

                    Ok(helm_response)
                } else {
                    error!("command execution error");
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    error!("<stderr>");
                    error!("{}", stderr);
                    error!("</stderr>");

                    Err(HelmWrapperError::Error)
                }
            }
            Err(e) => {
                error!("execution error: {}", e);
                Err(HelmWrapperError::ExecutionError(e))
            }
        }
    }

    fn install_or_upgrade(
        &self,
        namespace: &NonBlankString,
        release_name: &NonBlankString,
        chart_name: &NonBlankString,
        chart_version: Option<&NonBlankString>,
        values_overrides: Option<&HashMap<NonBlankString, NonBlankString>>,
        values_file: Option<&Path>,
        helm_options: Option<&Vec<NonBlankString>>,
    ) -> Result<HelmUpgradeStatus, HelmWrapperError> {
        info!(
            "installing helm chart '{}' with release name '{}' to namespace '{}'..",
            chart_name, release_name, namespace
        );

        debug!("helm executable path '{}'", self.get_helm_path());
        debug!("timeout {}s", self.get_timeout());

        let mut command_args = format!(
            "upgrade --install -n {} {} {}",
            namespace, release_name, chart_name
        );

        if let Some(chart_version) = chart_version {
            info!("- chart version '{chart_version}'");
            command_args.push_str(&format!(" --version {} ", chart_version));
        }

        if let Some(values_file) = values_file {
            info!("- values file '{}'", values_file.display());
            command_args.push_str(&format!(" -f {} ", values_file.display()));
        }

        if let Some(overrides) = values_overrides {
            if !self.get_unsafe_mode() {
                info!("overriden chart values won't be mentioned in log because of safe mode");
            }

            for (k, v) in overrides.iter() {
                if self.get_unsafe_mode() {
                    info!("- value override '{}': '{}'", k, v);
                }
                command_args.push_str(&format!(" --set {}={} ", k, v));
            }
        }

        if let Some(helm_options) = helm_options {
            for helm_option in helm_options {
                info!("- helm option '{helm_option}'");
                command_args.push_str(&format!(" {helm_option} "));
            }
        }

        if self.get_debug() {
            command_args.push_str(" --debug ");
        }

        command_args.push_str(&format!(" -o json --timeout {}s ", self.get_timeout()));

        let command_args = command_args.replace("  ", " ");
        let command_args = command_args.trim();

        if self.get_unsafe_mode() {
            debug!("command args: '{command_args}'")
        }

        let command_args: Vec<&str> = command_args.split(" ").collect();

        match Command::new(&self.get_helm_path())
            .args(command_args)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8(output.stdout)?;

                    if self.get_unsafe_mode() {
                        debug!("<stdout>");
                        debug!("{}", stdout);
                        debug!("</stdout>");
                    }

                    let helm_response: HelmUpgradeResponse = serde_json::from_str(&stdout)?;

                    info!("response: {:?}", helm_response);

                    Ok(helm_response.info.status)
                } else {
                    error!("command execution error");
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    error!("<stderr>");
                    error!("{}", stderr);
                    error!("</stderr>");

                    Err(HelmWrapperError::Error)
                }
            }
            Err(e) => {
                error!("execution error: {}", e);
                Err(HelmWrapperError::ExecutionError(e))
            }
        }
    }

    fn uninstall(
        &self,
        namespace: &NonBlankString,
        release_name: &NonBlankString,
    ) -> Result<(), HelmWrapperError> {
        info!(
            "uninstalling helm release '{}', namespace '{}'..",
            namespace, release_name
        );

        let command_args = format!(
            "uninstall -n {} {} --timeout={}s --debug",
            namespace,
            release_name,
            self.get_timeout()
        );

        let command_args: Vec<&str> = command_args.split(" ").collect();

        match Command::new(&self.get_helm_path())
            .args(command_args)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8(output.stdout)?;

                    if self.get_unsafe_mode() {
                        debug!("<stdout>");
                        debug!("{}", stdout);
                        debug!("</stdout>");
                    }

                    Ok(())
                } else {
                    error!("command execution error");
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    error!("<stderr>");
                    error!("{}", stderr);
                    error!("</stderr>");

                    Err(HelmWrapperError::Error)
                }
            }
            Err(e) => {
                error!("execution error: {}", e);
                Err(HelmWrapperError::ExecutionError(e))
            }
        }
    }
}

#[cfg(test)]
mod helm_command_tests {
    use std::{collections::HashMap, path::Path};

    use non_blank_string_rs::NonBlankString;

    use crate::{
        tests::{
            get_test_chart_name, get_test_helm_options, get_test_namespace, get_test_release_name,
            init_logging,
        },
        DefaultHelmExecutor, HelmExecutor, HelmUpgradeStatus,
    };

    #[test]
    fn install_or_upgrade_helm_chart_with_invalid_syntax_values() {
        init_logging();

        let executor = DefaultHelmExecutor::new_with_opts(&"helm".parse().unwrap(), 15, true, true);

        let helm_options: Vec<NonBlankString> = get_test_helm_options();

        let namespace: NonBlankString = get_test_namespace();
        let release_name: NonBlankString = get_test_release_name();
        let chart_name: NonBlankString = get_test_chart_name();

        let values_file = Path::new("test-data").join("whoami-invalid-syntax.yml");

        assert!(executor
            .install_or_upgrade(
                &namespace,
                &release_name,
                &chart_name,
                None,
                None,
                Some(&values_file),
                Some(&helm_options),
            )
            .is_err());
    }

    #[test]
    fn install_or_upgrade_helm_chart() {
        init_logging();

        let executor = DefaultHelmExecutor::new_with_opts(&"helm".parse().unwrap(), 15, true, true);

        let helm_options: Vec<NonBlankString> = get_test_helm_options();

        let namespace: NonBlankString = get_test_namespace();
        let release_name: NonBlankString = get_test_release_name();
        let chart_name: NonBlankString = get_test_chart_name();

        let mut values_overrides: HashMap<NonBlankString, NonBlankString> = HashMap::new();

        values_overrides.insert(
            "startupProbe.enabled".parse().unwrap(),
            "false".parse().unwrap(),
        );
        values_overrides.insert("replicaCount".parse().unwrap(), "2".parse().unwrap());

        let values_file = Path::new("test-data").join("whoami-values.yml");

        let result = executor
            .install_or_upgrade(
                &namespace,
                &release_name,
                &chart_name,
                Some(&"5.2.0".parse().unwrap()),
                Some(&values_overrides),
                Some(&values_file),
                Some(&helm_options),
            )
            .unwrap();

        assert_eq!(HelmUpgradeStatus::Deployed, result);

        let releases = executor.list(Some(&namespace)).unwrap();

        assert!(!releases.is_empty());

        assert!(executor.uninstall(&namespace, &release_name).is_ok());

        let releases = executor.list(Some(&namespace)).unwrap();

        assert!(releases.is_empty());
    }
}
