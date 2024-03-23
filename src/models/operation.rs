use std::cmp::min;

pub enum CostAllocationMethods {
    Fixed,
    Variable,
}
pub struct CostAllocation {
    pub method: CostAllocationMethods,
    pub ratio: f64,
}

pub enum DepreciationMethods {
    StraightLine,
    DecliningBalance,
    DoubleDecliningBalance,
}
pub struct FixedAsset {
    pub id: String,
    pub name: String,
    pub book_value: i64,
    pub useful_life: i64,
    pub salvage_value: i64,
    pub cum_depreciation: i64,
    pub depreciation: i64,
    pub depreciation_method: DepreciationMethods,
    pub cost_allocation: Option<Vec<CostAllocation>>,
}

impl FixedAsset {
    pub fn calc_depreciation(&self) -> i64 {
        match self.depreciation_method {
            DepreciationMethods::StraightLine => {
                let depr = (self.book_value + self.cum_depreciation - self.salvage_value)
                    / self.useful_life;
                let r_useful_life = self.useful_life - (self.cum_depreciation / depr);
                return depr * min(1, r_useful_life);
            }
            DepreciationMethods::DecliningBalance => 1 + 2,
            DepreciationMethods::DoubleDecliningBalance => 1 + 2,
        }
    }
    pub fn calc_book_value(&self) -> i64 {
        self.book_value - self.calc_depreciation()
    }
    pub fn calc_cum_depreciation(&self) -> i64 {
        self.cum_depreciation + self.calc_depreciation()
    }

    pub fn gen(&self) -> FixedAsset {
        FixedAsset {
            id: String::from(&self.id),
            name: String::from(&self.name),
            book_value: self.calc_book_value(),
            useful_life: self.useful_life,
            salvage_value: self.salvage_value,
            cum_depreciation: self.calc_cum_depreciation(),
            depreciation: self.calc_depreciation(),
            depreciation_method: self.depreciation_method.clone(),
            cost_allocation: self.cost_allocation.clone(),
        }
    }

}