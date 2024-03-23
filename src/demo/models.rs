use crate::models::fs;
use chrono::NaiveDate;
pub fn demo_financial_statements() -> fs::FinancialStatements {
    let current_assets = fs::CurrentAssets {
        prepayments: 19_729_794,
        inventories: 38_267_237,
        trade_and_other_receivables: 17_211_092,
        investments: 0,
        cash_and_cash_equivalents: 12_506_485,
        assets_in_disposal_groups_classified_as_held_for_sale: 0,
    };
    let non_current_assets = fs::NonCurrentAssets {
        property_plant_and_equipment: 118_835_254,
        investment_property: 0,
        intangible_assets: 638_358,
        investments_in_equity_accounted_associates: 0,
        investments: 322_900,
        other_assets: 1_337_445,
    };
    let current_liabilities = fs::CurrentLiabilities {
        trade_and_other_payables: 35_571_279,
        deferred_revenue: 8_701_445,
        income_tax_payable: 5_500_475,
        dividends_payable: 54_808,
        loans_and_borrowings: 47_783_727,
    };
    let non_current_liabilities = fs::NonCurrentLiabilities {
        loans_and_borrowings: 37_253_438,
        notes_and_accounts_payable: 230_844,
        employee_benefit_liabilities: 713_365,
        deferred_tax_liability: 0,
    };
    let shareholders_equity = fs::ShareholdersEquity {
        share_capital: 32_000_000,
        treasury_shares: -543_858,
        treasury_shares_surplus: 4_256,
        legal_reserve: 3_136_548,
        retained_earnings: 38_442_238,
    };
    let balance_sheet = fs::BalanceSheet {
        current_assets,
        non_current_assets,
        current_liabilities,
        non_current_liabilities,
        shareholders_equity,
    };
    let income_statements = fs::IncomeStatements {
        sales: 226_549_242,
        cost_of_sales: -180_056_411,
        selling_general_and_administrative_expense: -3_655_536,
        other_operating_income: 220_486,
        other_operating_expenses: -1_946_163,
        finance_expense: -6_195_410,
        net_miscellaneous_income: -186_470,
        tax_expense: -952_377,
        profit_or_loss_on_discontinued_operation_net_of_tax: 0,
    };
    let financial_statements = fs::FinancialStatements {
        financial_year: NaiveDate::from_ymd_opt(2023, 3, 20).unwrap(),
        balance_sheet,
        income_statements,
    };
    financial_statements
}
