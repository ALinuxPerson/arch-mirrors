use crate::Country;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Raw {
    url: String,
    protocol: String,
    last_sync: Option<String>,
    completion_pct: f64,
    duration_avg: Option<f64>,
    duration_stddev: Option<f64>,
    score: Option<f64>,
    active: bool,
    country: String,
    country_code: String,
    isos: bool,
    ipv4: bool,
    ipv6: bool,
    details: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Url {
    pub url: url::Url,
    pub protocol: crate::Protocol,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub completion_pct: f64,
    pub duration_average: Option<f64>,
    pub duration_stddev: Option<f64>,
    pub score: Option<f64>,
    pub active: bool,
    pub country: crate::Country,
    pub isos: bool,
    pub ipv4: bool,
    pub ipv6: bool,
    pub details: String,
}

impl From<Raw> for Url {
    fn from(raw: Raw) -> Self {
        let url: url::Url = raw
            .url
            .parse()
            .expect("failed to parse url field from raw url");
        let protocol: crate::Protocol = raw
            .protocol
            .parse()
            .expect("failed to parse protocol field from raw url");
        let last_sync = raw.last_sync.map(|raw| {
            raw.parse::<chrono::DateTime<chrono::Utc>>()
                .expect("failed to parse last_sync field from raw url")
        });
        let country = Country::new(&raw.country, &raw.country_code);

        Self {
            url,
            protocol,
            last_sync,
            completion_pct: raw.completion_pct,
            duration_average: raw.duration_avg,
            duration_stddev: raw.duration_stddev,
            score: raw.score,
            active: raw.active,
            country,
            isos: raw.isos,
            ipv4: raw.ipv4,
            ipv6: raw.ipv6,
            details: raw.details,
        }
    }
}
