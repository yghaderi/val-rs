use valrs::demo::models;
use valrs::models::fs::Base;

#[test]
fn financial_statements() {
    let fs = models::demo_financial_statements();
    // ---- balance-sheet
    assert_eq!(87_714_608, fs.balance_sheet.current_assets.total());
    assert_eq!(121_133_957, fs.balance_sheet.non_current_assets.total());
    assert_eq!(97_611_734, fs.balance_sheet.current_liabilities.total());
    assert_eq!(38_197_647, fs.balance_sheet.non_current_liabilities.total());
    assert_eq!(73_039_184, fs.balance_sheet.shareholders_equity.total());
    assert_eq!(
        fs.balance_sheet.total_assets(),
        fs.balance_sheet.total_liabilities_and_shareholders_equity()
    );
    // ---- income-statements
    let is_bottom_line = fs.income_statements.bottom_line();
    assert_eq!(46_492_831, is_bottom_line.gross_profit);
    assert_eq!(41_111_618, is_bottom_line.profit_from_operations);
    assert_eq!(34_729_738, is_bottom_line.profit_before_tax);
    assert_eq!(33_777_361, is_bottom_line.profit_from_continuing_operations);
    assert_eq!(33_777_361, is_bottom_line.profit);
}
