use serde::{Deserialize, Deserializer, Serialize, de};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForecastRow {
    pub sold_to_company: String,
    pub project_manager: String,
    pub project_name: String,
    pub project_id: String,
    pub class: String,
    pub start_date: String,
    pub finish_date: String,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub contract_total_value: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub contract_remaining_value: f64,
    pub currency: String,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month1_labor_revenue_commit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month2_labor_revenue_commit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month3_labor_revenue_commit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month4_labor_revenue_commit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month5_labor_revenue_commit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
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
