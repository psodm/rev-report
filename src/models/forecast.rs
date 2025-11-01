use serde::{Deserialize, Deserializer, Serialize, de};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Forecast {
    #[serde(rename = "Sold to Company")]
    pub sold_to_company: String,
    #[serde(rename = "Project Manager")]
    pub project_manager: String,
    #[serde(rename = "Project")]
    pub project_name: String,
    #[serde(rename = "ID")]
    pub project_id: String,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Contract Start Date")]
    pub start_date: String,
    #[serde(rename = "Contract Finish Date")]
    pub finish_date: String,
    #[serde(rename = "Contract Total Value", deserialize_with = "deserialize_float_from_empty_string")]
    pub contract_total_value: f64,
    #[serde(rename = "Contract Remaining Value", deserialize_with = "deserialize_float_from_empty_string")]
    pub contract_remaining_value: f64,
    #[serde(rename = "Currency")]
    pub currency: String,
    // Note: Labor Rev Committ columns are parsed manually in csv_loader
    // since there are multiple columns with the same name
    pub month1_labor_revenue_commit: f64,
    pub month2_labor_revenue_commit: f64,
    pub month3_labor_revenue_commit: f64,
    pub month4_labor_revenue_commit: f64,
    pub month5_labor_revenue_commit: f64,
    pub month6_labor_revenue_commit: f64,
}

fn deserialize_float_from_empty_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(0.0) // Or handle as an error, or return a default value
    } else {
        s.parse().map_err(de::Error::custom)
    }
}
