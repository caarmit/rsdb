use std::{borrow::BorrowMut, collections::HashMap};

use crate::errors::DbErr;

pub enum FieldValue {
    I32(i32),
}

struct TableEntry {
    pub fields: Vec<FieldValue>,
}

pub struct Table {
    pub columns: Vec<ColumnDescription>,
    pub rows: Vec<TableEntry>,
}

pub struct Database {
    tables: HashMap<String, Table>,
}

pub struct ColumnDescription {
    pub column_name: String,
    pub column_type: String,
}

impl Database {
    pub fn new() -> Database {
        Database { tables: HashMap::new() }
    }

    pub(crate) fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }

    pub(crate) fn create_table(&mut self, name: &str, columns: Vec<ColumnDescription>) -> Result<&Table, DbErr> {
        match self.tables.contains_key(name) {
            true => Err(DbErr::TableAlreadyExists),
            false => {
                let table = Table::from_column_definition(columns);
                self.tables.insert(name.to_string(), table);
                Ok(self.tables.get(name).unwrap())
            }
        }
    }

    pub(crate) fn delete_table(&mut self, name: &str) -> Result<Table, DbErr> {
        match self.tables.remove(name) {
            Some(table) => Ok(table),
            None => Err(DbErr::TableNotExists),
        }
    }
}

impl TableEntry {
    pub fn set_field(&mut self, field_id: usize, value: FieldValue) {
        self.fields[field_id] = value
    }
}

impl Table {
    pub fn from_column_definition(columns: Vec<ColumnDescription>) -> Table {
        return Table { columns, rows: Vec::new() };
    }

    fn insert_row(&mut self, values: Vec<FieldValue>) {
        self.rows.push(TableEntry { fields: values });
    }

    fn delete_row(&mut self, row_id: usize) {
        self.rows.remove(row_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::database::Database;

    #[test]
    fn create_first_table() {
        let mut database = Database::new();
        assert!(database.create_table("new_table", Vec::new()).is_ok())
    }

    #[test]
    fn error_overwrite_table() {
        let mut database = Database::new();

        let _ = database.create_table("new_table", Vec::new());
        assert!(database.create_table("new_table", Vec::new()).is_err())
    }

    #[test]
    fn delete_table() {
        let mut database = Database::new();
        let _ = database.create_table("table1", Vec::new());
        assert!(database.delete_table("table1").is_ok())
    }

    #[test]
    fn delete_missing_table() {
        let mut database = Database::new();
        assert!(database.delete_table("missing_table").is_err())
    }
}
