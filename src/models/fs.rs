use chrono::{NaiveDate};
pub trait Base {
    fn total(&self) -> i64;
    // fn struct_to_vec(&self) -> Vec<i64> {
    //     let json_value: Value = serde_json::to_value(&self).unwrap();
    //     let fields = json_value.as_object().unwrap();
    //     fields.values().filter_map(|v| v.as_i64()).collect()
    // }
}

pub struct CurrentAssets {
    pub cash_and_cash_equivalents: i64,
    pub marketable_securities: i64,
    pub accounts_receivable_net: i64,
    pub vendor_non_trade_receivables: i64,
    pub inventories: i64,
    pub other_current_assets: i64,
}

impl Base for CurrentAssets {
    fn total(&self) -> i64 {
        vec![
            self.cash_and_cash_equivalents,
            self.marketable_securities,
            self.accounts_receivable_net,
            self.vendor_non_trade_receivables,
            self.inventories,
            self.other_current_assets,
        ]
        .iter()
        .sum()
    }
}

struct NonCurrentAssets {
    marketable_securities: i64,
    property_plant_and_equipment_net: i64,
    other_non_current_assets: i64,
}

impl Base for NonCurrentAssets {
    fn total(&self) -> i64 {
        vec![
            self.marketable_securities,
            self.property_plant_and_equipment_net,
            self.other_non_current_assets,
        ]
        .iter()
        .sum()
    }
}
struct CurrentLiabilities {
    accounts_payable: i64,
    other_current_liabilities: i64,
    deferred_revenue: i64,
    commercial_paper: i64,
    term_debt: i64,
}
impl Base for CurrentLiabilities {
    fn total(&self) -> i64 {
        vec![
            self.accounts_payable,
            self.other_current_liabilities,
            self.deferred_revenue,
        ]
        .iter()
        .sum()
    }
}
struct NonCurrentLiabilities {
    term_debt: i64,
    other_non_current_liabilities: i64,
}
impl Base for NonCurrentLiabilities {
    fn total(&self) -> i64 {
        vec![self.term_debt, self.other_non_current_liabilities]
            .iter()
            .sum()
    }
}
struct ShareholdersEquity {
    common_stock: i64,
    retained_earnings: i64,
}
impl Base for ShareholdersEquity {
    fn total(&self) -> i64 {
        vec![self.common_stock, self.retained_earnings].iter().sum()
    }
}

struct BalanceSheet {
    current_assets: CurrentAssets,
    none_current_assets: NonCurrentAssets,
    current_liabilities: CurrentLiabilities,
    non_current_liabilities: NonCurrentLiabilities,
    shareholders_equity: ShareholdersEquity,
}

impl BalanceSheet {
    fn total_assets(&self) -> i64 {
        self.current_assets.total() + self.none_current_assets.total()
    }
    fn total_liabilities(&self) -> i64 {
        self.current_liabilities.total() + self.non_current_liabilities.total()
    }
    fn total_liabilities_and_shareholders_equity(&self) -> i64 {
        self.total_liabilities() + self.shareholders_equity.total()
    }
}

struct IncomeStatements{}
struct FinancialStatements{
    financial_year: NaiveDate,
    balance_sheet: BalanceSheet,
    income_statements: IncomeStatements
}
