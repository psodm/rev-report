use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
pub struct Project {
    #[serde(rename = "Project ID")]
    pub project_id: String,
    #[serde(rename = "Sales Org")]
    pub sales_org: String,
    #[serde(rename = "End Customer Name")]
    pub end_customer_name: String,
    #[serde(rename = "Project Name")]
    pub project_name: String,
    #[serde(rename = "Manager")]
    pub project_manager: String,
    #[serde(rename = "Other Stakeholder")]
    pub account_executive: Option<String>,
    #[serde(rename = "Start")]
    pub start_date: String,
    #[serde(rename = "Finish")]
    pub end_date: String,
    #[serde(rename = "On Hold", deserialize_with = "deserialize_bool_from_string")]
    pub on_hold: bool,
    #[serde(rename = "On Hold Comment")]
    pub on_hold_comment: Option<String>,
}

fn deserialize_bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.to_lowercase() == "true")
}
