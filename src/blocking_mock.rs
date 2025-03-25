use crate::{blocking::HelmExecutor, error::HelmWrapperError, HelmDeployStatus, HelmListItem};

pub struct SuccessMockHelmExecutor(Vec<HelmListItem>, HelmDeployStatus);

impl SuccessMockHelmExecutor {
    pub fn new(
        list_result: Vec<HelmListItem>,
        install_or_upgrade_result: HelmDeployStatus,
    ) -> Self {
        Self(list_result, install_or_upgrade_result)
    }
}

impl HelmExecutor for SuccessMockHelmExecutor {
    fn list(
        &self,
        _namespace: Option<&non_blank_string_rs::NonBlankString>,
    ) -> Result<Vec<HelmListItem>, HelmWrapperError> {
        Ok(self.0.clone())
    }

    fn install_or_upgrade(
        &self,
        _namespace: &non_blank_string_rs::NonBlankString,
        _release_name: &non_blank_string_rs::NonBlankString,
        _chart_name: &non_blank_string_rs::NonBlankString,
        _chart_version: Option<&non_blank_string_rs::NonBlankString>,
        _values_overrides: Option<
            &std::collections::HashMap<non_blank_string_rs::NonBlankString, String>,
        >,
        _values_file: Option<&std::path::Path>,
        _helm_options: Option<&Vec<non_blank_string_rs::NonBlankString>>,
    ) -> Result<HelmDeployStatus, HelmWrapperError> {
        Ok(self.1.clone())
    }

    fn uninstall(
        &self,
        _namespace: &non_blank_string_rs::NonBlankString,
        _release_name: &non_blank_string_rs::NonBlankString,
    ) -> Result<(), HelmWrapperError> {
        Ok(())
    }
}
