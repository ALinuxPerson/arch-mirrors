#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(invalid_codeblock_attributes)]
pub mod country;
pub mod protocol;
pub mod status;
pub mod url;
pub(crate) mod utils;

pub use crate::url::Url;
pub use country::Country;
pub use protocol::Protocol;
pub use status::Status;

pub async fn get_status() -> reqwest::Result<Status> {
    Status::get().await
}
