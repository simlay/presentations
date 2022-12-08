

use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();

    let value = std::str::from_utf8(record.value.as_ref())?.to_uppercase();

    Ok((key, value.into()))
}



