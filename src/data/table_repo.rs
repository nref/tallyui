use gloo::storage::LocalStorage;
use gloo_console::log;
use gloo_storage::Storage;
use std::collections::HashMap;

use crate::model::table::Table;

pub struct TableRepo {}

impl TableRepo {
    pub fn get_table(&self) -> Table {
        let table: Table = LocalStorage::get("table").unwrap_or_else(|_| {
            let mut rows = HashMap::new();
            rows.insert("EAs".to_string(), 0);
            rows.insert("Speech".to_string(), 0);
            rows.insert("Psych".to_string(), 0);
            rows.insert("Copiers".to_string(), 0);
            rows.insert("Misc".to_string(), 0);

            let default_table = Table { rows };

            self.save_table(&default_table);
            LocalStorage::get("table").unwrap_or(Table {
                rows: HashMap::new(),
            })
        });

        table
    }

    pub fn save_table(&self, table: &Table) {
        LocalStorage::set("table", table).ok();
    }

    pub fn update_row(&self, row: &String, count: i32) -> Table {
        let mut table = self.get_table();

        if let Some(entry) = table.rows.get_mut(row) {
            *entry += count;
        }

        self.save_table(&table);
        table
    }

    pub fn clear_data(&self) {
        log!("Clear All");
        LocalStorage::delete("table");
        self.get_table();
    }

    pub fn decrement_row(&self, row: &String) {
        self.update_row(row, -1);
    }

    pub fn increment_row(&self, row: &String) {
        self.update_row(row, 1);
    }
}
