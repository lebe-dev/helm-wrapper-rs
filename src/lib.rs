use serde::Deserialize;

#[cfg(feature = "nonblocking")]
pub mod nonblocking;

#[cfg(feature = "blocking")]
pub mod blocking;

pub mod error;

#[cfg(feature = "blocking-mock")]
pub mod blocking_mock;

#[cfg(feature = "nonblocking-mock")]
pub mod nonblocking_mock;

#[cfg(test)]
pub mod tests;

#[derive(Deserialize, Debug, Clone)]
pub struct HelmListItem {
    pub name: String,
    pub namespace: String,
    pub revision: String,
    pub updated: String,
    pub status: HelmDeployStatus,
    pub chart: String,
    pub app_version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HelmUpgradeResponse {
    pub info: HelmUpgradeResponseInfo,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HelmUpgradeResponseInfo {
    pub status: HelmDeployStatus,
}

#[derive(PartialEq, Deserialize, Debug, Clone)]
pub enum HelmDeployStatus {
    #[serde(rename = "deployed")]
    Deployed,
    #[serde(rename = "pending-install")]
    PendingInstall,
    #[serde(rename = "pending-upgrade")]
    PendingUpgrade,
    #[serde(rename = "failed")]
    Failed,
}
