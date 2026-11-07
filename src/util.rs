//! sqlite model — embedded SQL database.

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub tables: Vec<Table>,
}

impl Default for Model {
    fn default() -> Self { Self { tables: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create_table(&mut self, name: &str, columns: Vec<String>) -> bool {
        if self.tables.iter().any(|t| t.name == name) { return false; }
        self.tables.push(Table { name: name.into(), columns, rows: vec![] });
        true
    }

    pub fn insert(&mut self, table: &str, row: Vec<String>) -> bool {
        if let Some(t) = self.tables.iter_mut().find(|t| t.name == table) { t.rows.push(row); true } else { false }
    }

    pub fn query(&self, table: &str) -> Option<&Table> {
        self.tables.iter().find(|t| t.name == table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_insert_query() {
        let mut s = Model::new();
        assert!(s.create_table("users", vec!["id".into(), "name".into()]));
        assert!(s.insert("users", vec!["1".into(), "alice".into()]));
        assert_eq!(s.query("users").unwrap().rows.len(), 1);
    }
}
