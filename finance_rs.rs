// finance_rs.rs — финансовый учёт с категориями расходов на Rust

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write, BufRead};
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::Local;

#[derive(Serialize, Deserialize, Clone)]
struct Transaction {
    date: String,
    category: String,
    amount: f64,
    comment: String,
}

struct FinanceManager {
    transactions: Vec<Transaction>,
    budgets: HashMap<String, f64>,
    categories: HashSet<String>,
    data_file: String,
}

impl FinanceManager {
    fn new() -> Self {
        let mut fm = FinanceManager {
            transactions: Vec::new(),
            budgets: HashMap::new(),
            categories: HashSet::new(),
            data_file: "finance_data.json".to_string(),
        };
        fm.load_data();
        fm
    }

    fn load_data(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.data_file) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                if let Some(trans) = json["transactions"].as_array() {
                    for v in trans {
                        if let Ok(t) = serde_json::from_value::<Transaction>(v.clone()) {
                            self.transactions.push(t.clone());
                            self.categories.insert(t.category);
                        }
                    }
                }
                if let Some(bud) = json["budgets"].as_object() {
                    for (k, v) in bud {
                        if let Some(amt) = v.as_f64() {
                            self.budgets.insert(k.clone(), amt);
                        }
                    }
                }
            }
        }
    }

    fn save_data(&self) {
        let data = serde_json::json!({
            "transactions": self.transactions,
            "budgets": self.budgets,
            "categories": self.categories,
        });
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            let _ = fs::write(&self.data_file, json);
        }
    }

    fn add_transaction(&mut self, category: &str, amount: f64, comment: &str) {
        self.categories.insert(category.to_string());
        let t = Transaction {
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            category: category.to_string(),
            amount,
            comment: comment.to_string(),
        };
        self.transactions.push(t);
        self.save_data();
        println!("✅ Расход добавлен: {} {:.2} ({})", category, amount, comment);
    }

    fn set_budget(&mut self, category: &str, budget: f64) {
        self.budgets.insert(category.to_string(), budget);
        self.save_data();
        println!("✅ Бюджет для {}: {:.2}", category, budget);
    }

    fn stats(&self) {
        if self.transactions.is_empty() {
            println!("Нет данных.");
            return;
        }
        let total: f64 = self.transactions.iter().map(|t| t.amount).sum();
        let mut by_cat: HashMap<String, f64> = HashMap::new();
        for t in &self.transactions {
            *by_cat.entry(t.category.clone()).or_insert(0.0) += t.amount;
        }
        println!("📊 Общие расходы: {:.2}", total);
        println!("По категориям:");
        let mut sorted: Vec<_> = by_cat.into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for (cat, amt) in sorted {
            println!("  {}: {:.2}", cat, amt);
        }
        println!("Всего транзакций: {}", self.transactions.len());
    }

    fn budget_status(&self) {
        if self.budgets.is_empty() || self.transactions.is_empty() {
            println!("Нет бюджетов или расходов.");
            return;
        }
        let mut spent: HashMap<String, f64> = HashMap::new();
        for t in &self.transactions {
            *spent.entry(t.category.clone()).or_insert(0.0) += t.amount;
        }
        println!("📊 Бюджетный статус:");
        for (cat, budget) in &self.budgets {
            let sp = *spent.get(cat).unwrap_or(&0.0);
            let rem = budget - sp;
            let pct = if *budget > 0.0 { (sp / budget) * 100.0 } else { 0.0 };
            let sign = if sp > *budget { "⚠️" } else { "✅" };
            println!("  {} {}: потрачено {:.2}/{:.2} ({:.1}%), осталось {:.2}",
                sign, cat, sp, budget, pct, rem);
        }
    }

    fn recommendations(&self) {
        if self.transactions.is_empty() || self.budgets.is_empty() {
            println!("Добавьте расходы и бюджеты для получения рекомендаций.");
            return;
        }
        let mut spent: HashMap<String, f64> = HashMap::new();
        for t in &self.transactions {
            *spent.entry(t.category.clone()).or_insert(0.0) += t.amount;
        }
        let over = self.budgets.iter().any(|(cat, budget)| *spent.get(cat).unwrap_or(&0.0) > *budget);
        println!("💡 Рекомендации:");
        if over {
            println!("  ⚠️ Вы превысили бюджет по некоторым категориям.");
        } else {
            println!("  ✅ Все бюджеты в норме, отлично!");
        }
        if self.transactions.len() < 5 {
            println!("  📝 Добавьте больше транзакций для точного анализа.");
        }
    }

    fn export_csv(&self, filename: &str) {
        if let Ok(mut file) = fs::File::create(filename) {
            use std::io::Write;
            writeln!(file, "Date,Category,Amount,Comment").unwrap();
            for t in &self.transactions {
                writeln!(file, "{},{},{:.2},{}", t.date, t.category, t.amount, t.comment).unwrap();
            }
            println!("Экспортировано в {}", filename);
        }
    }

    fn interactive(&mut self) {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        println!("💰 MoneyTracker Pro — Rust Edition");
        println!("Команды: add <категория> <сумма> [комментарий], budget <категория> <сумма>,");
        println!("        stats, status, recs, export, exit");
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            if reader.read_line(&mut line).is_err() { break; }
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.is_empty() { continue; }
            match parts[0] {
                "exit" => break,
                "add" => {
                    if parts.len() >= 3 {
                        if let Ok(amount) = parts[2].parse::<f64>() {
                            let comment = if parts.len() > 3 { parts[3..].join(" ") } else { "" };
                            self.add_transaction(parts[1], amount, &comment);
                        } else {
                            println!("Ошибка ввода суммы.");
                        }
                    } else {
                        println!("Использование: add <категория> <сумма> [комментарий]");
                    }
                }
                "budget" => {
                    if parts.len() >= 3 {
                        if let Ok(budget) = parts[2].parse::<f64>() {
                            self.set_budget(parts[1], budget);
                        } else {
                            println!("Ошибка ввода суммы.");
                        }
                    } else {
                        println!("Использование: budget <категория> <сумма>");
                    }
                }
                "stats" => self.stats(),
                "status" => self.budget_status(),
                "recs" => self.recommendations(),
                "export" => self.export_csv("finance_export.csv"),
                _ => println!("Неизвестная команда."),
            }
        }
    }
}

fn main() {
    let mut fm = FinanceManager::new();
    fm.interactive();
}
