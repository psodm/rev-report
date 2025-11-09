use chrono::NaiveDate;
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
    #[serde(
        rename = "Contract Start Date",
        deserialize_with = "deserialize_date_from_dd_mm_yyyy"
    )]
    pub start_date: NaiveDate,
    #[serde(
        rename = "Contract Finish Date",
        deserialize_with = "deserialize_date_from_dd_mm_yyyy"
    )]
    pub finish_date: NaiveDate,
    #[serde(
        rename = "Contract Total Value",
        deserialize_with = "deserialize_float_from_empty_string"
    )]
    pub contract_total_value: f64,
    #[serde(
        rename = "Contract Remaining Value",
        deserialize_with = "deserialize_float_from_empty_string"
    )]
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

fn deserialize_date_from_dd_mm_yyyy<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.is_empty() {
        // Return a default date or error - using epoch as default
        return Ok(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    }

    // Parse dd/mm/yyyy format
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() != 3 {
        return Err(de::Error::custom(format!(
            "Invalid date format: expected dd/mm/yyyy, got '{}'",
            s
        )));
    }

    let day = parts[0]
        .parse::<u32>()
        .map_err(|e| de::Error::custom(format!("Invalid day in date '{}': {}", s, e)))?;
    let month = parts[1]
        .parse::<u32>()
        .map_err(|e| de::Error::custom(format!("Invalid month in date '{}': {}", s, e)))?;
    let year = parts[2]
        .parse::<i32>()
        .map_err(|e| de::Error::custom(format!("Invalid year in date '{}': {}", s, e)))?;

    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| de::Error::custom(format!("Invalid date '{}': day/month out of range", s)))
}
