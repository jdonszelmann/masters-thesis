#[derive(Debug, serde::Serialize)]
#[serde(rename_all="snake_case")]
pub enum InitialTraceSetting {
    Off,

    Messages,

    Verbose,
}
