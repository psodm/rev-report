#[derive(Clone)]
pub struct Project {
    pub project_id: String,
    pub end_customer_name: String,
    pub project_name: String,
    pub project_manager: String,
    pub account_executive: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub on_hold: bool,
    pub on_hold_comment: Option<String>,
}
