use crate::{error::HelmWrapperError, HelmDeployStatus, HelmExecutor, HelmListItem};

pub struct MockHelmExecutor(
    Result<Vec<HelmListItem>, HelmWrapperError>,
    Result<HelmDeployStatus, HelmWrapperError>,
    Result<(), HelmWrapperError>,
);

impl MockHelmExecutor {
    pub fn new(
        list_result: Result<Vec<HelmListItem>, HelmWrapperError>,
        install_or_upgrade_result: Result<HelmDeployStatus, HelmWrapperError>,
        uninstall_result: Result<(), HelmWrapperError>,
    ) -> Self {
        Self(
            list_result.clone(),
            install_or_upgrade_result.clone(),
            uninstall_result,
        )
    }
}

impl HelmExecutor for MockHelmExecutor {
    fn list(
        &self,
        _namespace: Option<&non_blank_string_rs::NonBlankString>,
    ) -> Result<Vec<HelmListItem>, HelmWrapperError> {
        self.0.clone()
    }

    fn install_or_upgrade(
        &self,
        _namespace: &non_blank_string_rs::NonBlankString,
        _release_name: &non_blank_string_rs::NonBlankString,
        _chart_name: &non_blank_string_rs::NonBlankString,
        _chart_version: Option<&non_blank_string_rs::NonBlankString>,
        _values_overrides: Option<
            &std::collections::HashMap<
                non_blank_string_rs::NonBlankString,
                non_blank_string_rs::NonBlankString,
            >,
        >,
        _values_file: Option<&std::path::Path>,
        _helm_options: Option<&Vec<non_blank_string_rs::NonBlankString>>,
    ) -> Result<HelmDeployStatus, HelmWrapperError> {
        self.1.clone()
    }

    fn uninstall(
        &self,
        _namespace: &non_blank_string_rs::NonBlankString,
        _release_name: &non_blank_string_rs::NonBlankString,
    ) -> Result<(), HelmWrapperError> {
        self.2.clone()
    }
}
