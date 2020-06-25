use serde::Serialize;
use Default;

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub world_id: String,
    pub format: String,
    pub print_friendly: bool,
}

impl Variables {
    fn new(world_id: String, format: &str, print_friendly: bool) -> Self {
        Self {
            world_id,
            format: format.to_owned(),
            print_friendly,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRequest {
    pub operation_name: String,
    pub variables: Variables,
    pub query: String,
}

impl Default for ExportRequest {
    fn default() -> Self {
        Self {
            query: "mutation exportWorld($worldId: CUID!, $format: DocumentFormat!, $printFriendly: Boolean) {\n  world(id: $worldId) {\n    id\n    export(format: $format, printFriendly: $printFriendly)\n    __typename\n  }\n}\n".to_owned(),
            operation_name: "exportWorld".to_owned(),
            variables: Variables::default()
        }
    }
}

impl ExportRequest {
    pub fn new(world_id: String) -> Self {
        Self {
            variables: Variables::new(world_id, "JSON", false),
            ..Default::default()
        }
    }
}
