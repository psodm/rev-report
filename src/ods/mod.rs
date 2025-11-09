use crate::services::forecast_service::RevenueSummaryBySalesOrg;
use spreadsheet_ods::{Sheet, WorkBook, write_ods};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;
use tracing::{debug, info, instrument};

#[derive(Debug)]
pub enum OdsExportError {
    Io(std::io::Error),
    Ods(spreadsheet_ods::OdsError),
}

impl Display for OdsExportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OdsExportError::Io(err) => write!(f, "failed to determine output directory: {}", err),
            OdsExportError::Ods(err) => write!(f, "failed to write ODS file: {}", err),
        }
    }
}

impl Error for OdsExportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OdsExportError::Io(err) => Some(err),
            OdsExportError::Ods(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for OdsExportError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<spreadsheet_ods::OdsError> for OdsExportError {
    fn from(value: spreadsheet_ods::OdsError) -> Self {
        Self::Ods(value)
    }
}

#[instrument(skip(summaries))]
pub fn save_forecast_by_sales_org(
    summaries: &[RevenueSummaryBySalesOrg],
) -> Result<PathBuf, OdsExportError> {
    let mut workbook = WorkBook::new_empty();
    let mut sheet = Sheet::new("Forecast by Sales Org");

    let headers = [
        "Sales Org",
        "Month 1",
        "Month 2",
        "Month 3",
        "Month 4",
        "Month 5",
        "Month 6",
    ];

    for (col_index, header) in headers.iter().enumerate() {
        let col =
            u32::try_from(col_index).expect("column index should fit into u32 for ODS export");
        sheet.set_value(0, col, (*header).to_owned());
    }

    for (row_index, summary) in summaries.iter().enumerate() {
        let row =
            u32::try_from(row_index + 1).expect("row index should fit into u32 for ODS export");

        sheet.set_value(row, 0, summary.sales_org.as_str());
        sheet.set_value(row, 1, summary.month1_total);
        sheet.set_value(row, 2, summary.month2_total);
        sheet.set_value(row, 3, summary.month3_total);
        sheet.set_value(row, 4, summary.month4_total);
        sheet.set_value(row, 5, summary.month5_total);
        sheet.set_value(row, 6, summary.month6_total);
    }

    workbook.push_sheet(sheet);

    let output_path = std::env::current_dir()?.join("forecast_by_sales_org.ods");
    debug!(?output_path, "Saving Sales Org forecast ODS report");
    write_ods(&mut workbook, &output_path)?;
    info!(?output_path, "Saved Sales Org forecast ODS report");

    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_summaries() -> Vec<RevenueSummaryBySalesOrg> {
        vec![
            RevenueSummaryBySalesOrg {
                sales_org: "ORG-A".to_string(),
                month1_total: 1000.0,
                month2_total: 2000.0,
                month3_total: 3000.0,
                month4_total: 4000.0,
                month5_total: 5000.0,
                month6_total: 6000.0,
            },
            RevenueSummaryBySalesOrg {
                sales_org: "ORG-B".to_string(),
                month1_total: 1500.0,
                month2_total: 2500.0,
                month3_total: 3500.0,
                month4_total: 4500.0,
                month5_total: 5500.0,
                month6_total: 6500.0,
            },
        ]
    }

    #[test]
    fn test_save_forecast_by_sales_org_creates_file() {
        let path =
            save_forecast_by_sales_org(&sample_summaries()).expect("ODS export should succeed");

        assert!(path.exists(), "expected ODS file to be created");

        let metadata = std::fs::metadata(&path).expect("expected metadata for ODS file");
        assert!(metadata.len() > 0, "expected ODS file to contain data");

        std::fs::remove_file(path).expect("expected to remove ODS file after test");
    }
}
