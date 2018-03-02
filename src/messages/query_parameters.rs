use serde_json::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryParameters {
    pub fq: Option<FilterQuery>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterQuery {
    Or(OrExpr),
    Value(Value),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OrExpr {
    #[serde(rename = "$or")]
    values: Vec<FilterQuery>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn parse_empty_fq() {
        let query = r#"{}"#;
        let params: QueryParameters = serde_json::from_str(&query).unwrap();
        assert_eq!(params.fq, None);
    }

    #[test]
    fn parse_single_field_fq() {
        let query = r#"{
            "fq": {
                "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2"
            }
        }"#;

        let params: QueryParameters = serde_json::from_str(&query).unwrap();
        let fq: Value = serde_json::to_value(params.fq).unwrap();
        let room_id = fq["room_id"].as_str();

        assert!(room_id.is_some());
        assert_eq!(room_id.unwrap(), "050b7c6f-795c-4cb4-aeea-5ee3f9083de2");
    }
}
