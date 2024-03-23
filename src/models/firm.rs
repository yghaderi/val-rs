use crate::models::{fs, operation};

pub struct BaseParam{
    pub financial_year: operation::FinancialYear,
    pub base_rate_change: operation::BaseRateChange
}
pub enum FirmCategory {
    Production,
}


struct Firm {
    pub base_param: BaseParam,
    pub category: FirmCategory,
    pub financial_statements: fs::FinancialStatements,
    pub cost_centers: Vec<operation::CostCenter>,
}
