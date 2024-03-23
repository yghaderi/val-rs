use chrono::{Datelike, NaiveDate};
use std::arch::aarch64::float32x2_t;
use std::cmp::min;

#[derive(Clone, Debug)]
pub enum CostAllocationMethods {
    Fixed,
    Variable,
}
#[derive(Clone, Debug)]
pub struct CostAllocation {
    pub method: CostAllocationMethods,
    pub ratio: f64,
}

#[derive(Clone, Debug)]
pub enum DepreciationMethods {
    StraightLine,
    DecliningBalance,
    DoubleDecliningBalance,
}

#[derive(Clone, Debug)]
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

struct ExtraChange {
    date: NaiveDate,
    factor: f64,
}

pub struct AssetsUnderConstruction {
    pub id: String,
    pub name: String,
    pub capital_expense: i64,
    pub salvage_value: i64,
    pub depreciation_method: DepreciationMethods,
    pub useful_life: i64,
    pub cost_allocation: Option<Vec<CostAllocation>>,
    pub extra_change: Option<Vec<ExtraChange>>,
}

struct PreNormProduction {
    date: NaiveDate,
    qty: f64,
}
pub struct CapacityEffect {
    product_id: String,
    capacity: f64,
    production: f64,
    pre_norm_production: Vec<PreNormProduction>,

}

struct Progress {
    date: NaiveDate,
    pct: f64, // cum-sum
}

struct DevelopPlan {
    id: String,
    name: String,
    assets_under_construction: Vec<AssetsUnderConstruction>,
    capacity_effect: Option<Vec<CapacityEffect>>,
    progress: Vec<Progress>,
}

trait Test{
    fn fy(date:NaiveDate)-> NaiveDate{
        date
    }
}
impl DevelopPlan {


    fn fixed_asset(self) -> Option<Vec<FixedAsset>> {
        if self.progress[0].pct == 1. {
            let mut result: Vec<FixedAsset> = vec![];
            for i in self.assets_under_construction {
                result.push(FixedAsset {
                    id: i.id,
                    name: i.name,
                    book_value: i.capital_expense,
                    useful_life: i.useful_life,
                    salvage_value: i.salvage_value,
                    cum_depreciation: 0,
                    depreciation: 0,
                    depreciation_method: i.depreciation_method,
                    cost_allocation: i.cost_allocation,
                })
            }
            Some(result)
        } else {
            None
        }
    }
    fn assets_under_construction(self){

    }
}
#[derive(Clone, Debug)]
pub struct FinancialYear {
    pub date: NaiveDate,
    pub length: u8,
}

impl FinancialYear {
    pub fn dates(&self) -> Vec<NaiveDate> {
        let mut dates: Vec<NaiveDate> = vec![];
        for i in 0..self.length {
            dates.push(
                NaiveDate::from_ymd_opt(
                    self.date.yaer() + i as i32,
                    self.date.month(),
                    self.date.day(),
                )
                    .unwrap(),
            )
        }
        dates
    }
}

pub struct RateChange{
    pub date: NaiveDate,
    pub f: f64
}

pub struct  BaseRateChange{
    pub id: String,
    pub name: String,
    pub rates: Vec<RateChange>
}

pub struct Input {
    pub fixed_assets: Option<Vec<FixedAsset>>,
}
pub enum CostCenterCategory {
    Product,
    Service,
    Operational,
}
pub struct CostCenter {
    pub id: String,
    pub name: String,
    pub category: CostCenterCategory,
    pub input: Option<Input>,
}