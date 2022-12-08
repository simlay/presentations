
#![allow(clippy::unnecessary_mut_passed)]
#[path = "../../github-stars-aggregate/src/model.rs"]
mod model;
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};

use model::GithubStars;




#[smartmodule(filter_map)]
fn filter_map(record: &Record) -> Result<Option<(Option<RecordData>, RecordData)>> {
    let stars = serde_json::from_slice::<GithubStars>(record.value.as_ref())?;

    if stars.star_update {
        let stars = format!("Fluvio Github Star count is now {}", stars.stargazers_count);
        Ok(Some((record.key.clone(), stars.into())))
    } else {
        Ok(None)
    }
}
