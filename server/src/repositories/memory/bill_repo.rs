use async_trait::async_trait;
use chrono::{Datelike, NaiveDate, Weekday};
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{Bill, BillType, CreateBill, UpdateBill};
use crate::repositories::traits::{BillFilter, BillRepository, BillStatistics, CategoryStatistics, DailyStatistics, TrendStatistics};

pub struct MemoryBillRepository {
    bills: RwLock<HashMap<Uuid, Bill>>,
}

impl MemoryBillRepository {
    pub fn new() -> Self {
        Self {
            bills: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryBillRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BillRepository for MemoryBillRepository {
    async fn create(&self, bill: CreateBill) -> AppResult<Bill> {
        let new_bill = Bill::new(
            bill.ledger_id,
            bill.category_id,
            bill.user_id,
            bill.bill_type,
            bill.amount,
            bill.note,
            bill.bill_date,
        );
        let mut bills = self.bills.write().unwrap();
        bills.insert(new_bill.id, new_bill.clone());
        Ok(new_bill)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Bill>> {
        let bills = self.bills.read().unwrap();
        Ok(bills.get(&id).cloned())
    }

    async fn find_by_filter(&self, filter: BillFilter) -> AppResult<(Vec<Bill>, u64)> {
        let bills = self.bills.read().unwrap();
        let mut filtered: Vec<Bill> = bills
            .values()
            .filter(|b| {
                if b.ledger_id != filter.ledger_id {
                    return false;
                }
                if let Some(start) = filter.start_date {
                    if b.bill_date < start {
                        return false;
                    }
                }
                if let Some(end) = filter.end_date {
                    if b.bill_date > end {
                        return false;
                    }
                }
                if let Some(ref t) = filter.bill_type {
                    if &b.bill_type != t {
                        return false;
                    }
                }
                if let Some(cat_id) = filter.category_id {
                    if b.category_id != cat_id {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        filtered.sort_by(|a, b| b.bill_date.cmp(&a.bill_date));

        let total = filtered.len() as u64;
        let start = ((filter.page - 1) * filter.page_size) as usize;
        let end = std::cmp::min(start + filter.page_size as usize, filtered.len());

        if start >= filtered.len() {
            return Ok((vec![], total));
        }

        Ok((filtered[start..end].to_vec(), total))
    }

    async fn update(&self, id: Uuid, update: UpdateBill) -> AppResult<Bill> {
        let mut bills = self.bills.write().unwrap();
        let bill = bills.get_mut(&id).ok_or_else(|| AppError::NotFound("Bill not found".to_string()))?;

        if let Some(cat_id) = update.category_id {
            bill.category_id = cat_id;
        }
        if let Some(t) = update.bill_type {
            bill.bill_type = t;
        }
        if let Some(amount) = update.amount {
            bill.amount = amount;
        }
        if update.note.is_some() {
            bill.note = update.note;
        }
        if let Some(date) = update.bill_date {
            bill.bill_date = date;
        }
        bill.updated_at = chrono::Utc::now();

        Ok(bill.clone())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut bills = self.bills.write().unwrap();
        bills.remove(&id);
        Ok(())
    }

    async fn get_statistics(&self, ledger_id: Uuid, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>) -> AppResult<BillStatistics> {
        let bills = self.bills.read().unwrap();
        let filtered: Vec<&Bill> = bills
            .values()
            .filter(|b| {
                if b.ledger_id != ledger_id {
                    return false;
                }
                if let Some(start) = start_date {
                    if b.bill_date < start {
                        return false;
                    }
                }
                if let Some(end) = end_date {
                    if b.bill_date > end {
                        return false;
                    }
                }
                true
            })
            .collect();

        let total_income: f64 = filtered.iter().filter(|b| b.bill_type == BillType::Income).map(|b| b.amount).sum();
        let total_expense: f64 = filtered.iter().filter(|b| b.bill_type == BillType::Expense).map(|b| b.amount).sum();

        Ok(BillStatistics {
            total_income,
            total_expense,
            balance: total_income - total_expense,
        })
    }

    async fn get_category_statistics(&self, ledger_id: Uuid, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>, bill_type: Option<BillType>) -> AppResult<Vec<CategoryStatistics>> {
        let bills = self.bills.read().unwrap();
        let target_type = bill_type.unwrap_or(BillType::Expense);

        let filtered: Vec<&Bill> = bills
            .values()
            .filter(|b| {
                if b.ledger_id != ledger_id || b.bill_type != target_type {
                    return false;
                }
                if let Some(start) = start_date {
                    if b.bill_date < start {
                        return false;
                    }
                }
                if let Some(end) = end_date {
                    if b.bill_date > end {
                        return false;
                    }
                }
                true
            })
            .collect();

        let mut category_data: HashMap<Uuid, (f64, u32)> = HashMap::new();
        for bill in &filtered {
            let entry = category_data.entry(bill.category_id).or_insert((0.0, 0));
            entry.0 += bill.amount;
            entry.1 += 1;
        }

        let total: f64 = category_data.values().map(|(a, _)| a).sum();

        let mut result: Vec<CategoryStatistics> = category_data
            .into_iter()
            .map(|(cat_id, (amount, count))| CategoryStatistics {
                category_id: cat_id,
                category_name: "Category".to_string(),
                category_icon: None,
                bill_type: target_type.clone(),
                amount,
                count,
                percentage: if total > 0.0 { (amount / total) * 100.0 } else { 0.0 },
            })
            .collect();

        result.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap_or(std::cmp::Ordering::Equal));
        Ok(result)
    }

    async fn get_daily_statistics(&self, ledger_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> AppResult<Vec<DailyStatistics>> {
        let bills = self.bills.read().unwrap();

        let mut daily_data: HashMap<NaiveDate, (f64, f64)> = HashMap::new();

        // Initialize all dates in range
        let mut current = start_date;
        while current <= end_date {
            daily_data.insert(current, (0.0, 0.0));
            current = current.succ_opt().unwrap_or(current);
        }

        // Aggregate bills
        for bill in bills.values() {
            if bill.ledger_id != ledger_id {
                continue;
            }
            if bill.bill_date < start_date || bill.bill_date > end_date {
                continue;
            }

            let entry = daily_data.entry(bill.bill_date).or_insert((0.0, 0.0));
            match bill.bill_type {
                BillType::Income => entry.0 += bill.amount,
                BillType::Expense => entry.1 += bill.amount,
            }
        }

        let mut result: Vec<DailyStatistics> = daily_data
            .into_iter()
            .map(|(date, (income, expense))| DailyStatistics { date, income, expense })
            .collect();

        result.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(result)
    }

    async fn get_trend_statistics(&self, ledger_id: Uuid, start_date: NaiveDate, end_date: NaiveDate, group_by: &str) -> AppResult<Vec<TrendStatistics>> {
        let bills = self.bills.read().unwrap();

        let filtered: Vec<&Bill> = bills
            .values()
            .filter(|b| b.ledger_id == ledger_id && b.bill_date >= start_date && b.bill_date <= end_date)
            .collect();

        let mut trend_data: HashMap<String, (f64, f64)> = HashMap::new();

        for bill in filtered {
            let period = match group_by {
                "day" => bill.bill_date.format("%Y-%m-%d").to_string(),
                "week" => {
                    let iso_week = bill.bill_date.iso_week();
                    format!("{}-W{:02}", iso_week.year(), iso_week.week())
                }
                "month" => bill.bill_date.format("%Y-%m").to_string(),
                "year" => bill.bill_date.format("%Y").to_string(),
                _ => bill.bill_date.format("%Y-%m").to_string(),
            };

            let entry = trend_data.entry(period).or_insert((0.0, 0.0));
            match bill.bill_type {
                BillType::Income => entry.0 += bill.amount,
                BillType::Expense => entry.1 += bill.amount,
            }
        }

        let mut result: Vec<TrendStatistics> = trend_data
            .into_iter()
            .map(|(period, (income, expense))| TrendStatistics {
                period,
                income,
                expense,
                balance: income - expense,
            })
            .collect();

        result.sort_by(|a, b| a.period.cmp(&b.period));
        Ok(result)
    }
}
