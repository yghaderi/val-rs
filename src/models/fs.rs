use chrono::NaiveDate;
pub trait Base {
    fn total(&self) -> i64;
}

pub struct CurrentAssets {
    pub prepayments: i64,
    pub inventories: i64,
    pub trade_and_other_receivables: i64,
    pub investments: i64,
    pub cash_and_cash_equivalents: i64,
    pub assets_in_disposal_groups_classified_as_held_for_sale: i64,
}

impl Base for CurrentAssets {
    fn total(&self) -> i64 {
        vec![
            self.prepayments,
            self.inventories,
            self.trade_and_other_receivables,
            self.investments,
            self.cash_and_cash_equivalents,
            self.assets_in_disposal_groups_classified_as_held_for_sale,
        ]
        .iter()
        .sum()
    }
}

pub struct NonCurrentAssets {
    pub property_plant_and_equipment: i64,
    pub investment_property: i64,
    pub intangible_assets: i64,
    pub investments_in_equity_accounted_associates: i64,
    pub investments: i64,
    pub other_assets: i64,
}

impl Base for NonCurrentAssets {
    fn total(&self) -> i64 {
        vec![
            self.property_plant_and_equipment,
            self.investment_property,
            self.intangible_assets,
            self.investments_in_equity_accounted_associates,
            self.investments,
            self.other_assets,
        ]
        .iter()
        .sum()
    }
}
pub struct CurrentLiabilities {
    pub trade_and_other_payables: i64,
    pub income_tax_payable: i64,
    pub dividends_payable: i64,
    pub loans_and_borrowings: i64,
    pub deferred_revenue: i64,
}
impl Base for CurrentLiabilities {
    fn total(&self) -> i64 {
        vec![
            self.trade_and_other_payables,
            self.income_tax_payable,
            self.dividends_payable,
            self.loans_and_borrowings,
            self.deferred_revenue,
        ]
        .iter()
        .sum()
    }
}
pub struct NonCurrentLiabilities {
    pub loans_and_borrowings: i64,
    pub notes_and_accounts_payable: i64,
    pub employee_benefit_liabilities: i64,
    pub deferred_tax_liability: i64,
}
impl Base for NonCurrentLiabilities {
    fn total(&self) -> i64 {
        vec![
            self.loans_and_borrowings,
            self.notes_and_accounts_payable,
            self.employee_benefit_liabilities,
            self.deferred_tax_liability,
        ]
        .iter()
        .sum()
    }
}
pub struct ShareholdersEquity {
    pub share_capital: i64,
    pub treasury_shares: i64,
    pub treasury_shares_surplus: i64,
    pub legal_reserve: i64,
    pub retained_earnings: i64,
}
impl Base for ShareholdersEquity {
    fn total(&self) -> i64 {
        vec![
            self.share_capital,
            self.treasury_shares,
            self.treasury_shares_surplus,
            self.legal_reserve,
            self.retained_earnings,
        ]
        .iter()
        .sum()
    }
}

pub struct BalanceSheet {
    pub current_assets: CurrentAssets,
    pub non_current_assets: NonCurrentAssets,
    pub current_liabilities: CurrentLiabilities,
    pub non_current_liabilities: NonCurrentLiabilities,
    pub shareholders_equity: ShareholdersEquity,
}

impl BalanceSheet {
    pub fn total_assets(&self) -> i64 {
        self.current_assets.total() + self.non_current_assets.total()
    }
    pub fn total_liabilities(&self) -> i64 {
        self.current_liabilities.total() + self.non_current_liabilities.total()
    }
    pub fn total_liabilities_and_shareholders_equity(&self) -> i64 {
        self.total_liabilities() + self.shareholders_equity.total()
    }
}

pub struct IncomeStatements {
    pub sales: i64,
    pub cost_of_sales: i64,
    pub selling_general_and_administrative_expense: i64,
    pub other_operating_income: i64,
    pub other_operating_expenses: i64,
    pub finance_expense: i64,
    pub net_miscellaneous_income: i64,
    pub tax_expense: i64,
    pub profit_or_loss_on_discontinued_operation_net_of_tax: i64,
}
pub struct BottomLine {
    pub gross_profit: i64,
    pub profit_from_operations: i64,
    pub profit_before_tax: i64,
    pub profit_from_continuing_operations: i64,
    pub profit: i64,
}

impl IncomeStatements {
    pub fn bottom_line(self) -> BottomLine {
        let gross_profit = self.sales + self.cost_of_sales;
        let profit_from_operations = gross_profit
            + self.selling_general_and_administrative_expense
            + self.other_operating_income
            + self.other_operating_expenses;
        let profit_before_tax =
            profit_from_operations + self.finance_expense + self.net_miscellaneous_income;

        let profit_from_continuing_operations = profit_before_tax + self.tax_expense;
        let profit = profit_from_continuing_operations
            + self.profit_or_loss_on_discontinued_operation_net_of_tax;
        BottomLine {
            gross_profit,
            profit_from_operations,
            profit_before_tax,
            profit_from_continuing_operations,
            profit,
        }
    }
}
pub struct FinancialStatements {
    pub financial_year: NaiveDate,
    pub balance_sheet: BalanceSheet,
    pub income_statements: IncomeStatements,
}
