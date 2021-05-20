use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Raw {
    cutoff: u32,
    last_check: String,
    num_checks: u32,
    check_frequency: u32,
    urls: Vec<crate::url::Raw>,
    version: u32,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Status {
    pub cutoff: u32,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub num_checks: u32,
    pub check_frequency: u32,
    pub urls: Vec<crate::Url>,
    pub version: u32,
}

impl Status {
    pub const URL: &'static str = "https://archlinux.org/mirrors/status/json";

    pub async fn get() -> reqwest::Result<Self> {
        let response = reqwest::get(Self::URL).await?;
        let raw: Raw = response
            .json()
            .await
            .expect("failed to parse response to json");

        Ok(Self::from(raw))
    }
}

impl From<Raw> for Status {
    fn from(raw: Raw) -> Self {
        let last_check: chrono::DateTime<chrono::Utc> = raw
            .last_check
            .parse()
            .expect("failed to parse last_check field from raw status");
        let urls: Vec<crate::Url> = raw.urls.into_iter().map(crate::Url::from).collect();

        Self {
            cutoff: raw.cutoff,
            last_check,
            num_checks: raw.num_checks,
            check_frequency: raw.check_frequency,
            urls,
            version: raw.version,
        }
    }
}
