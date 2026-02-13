use crate::registry::models::*;
use crate::registry::Registry;
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::sync::Mutex;
use include_dir::{include_dir, Dir};
use std::path::Path;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub struct SqliteRegistry {
    conn: Mutex<Connection>,
}

impl SqliteRegistry {
    /// Create or open the database at the given path, and run migrations.
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let registry = SqliteRegistry {
            conn: Mutex::new(conn),
        };
        registry.run_migrations()?;
        Ok(registry)
    }

    /// Run embedded SQL migrations.
    fn run_migrations(&self) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        // Check if tables exist by trying to select from a known table
        let table_exists: bool = tx
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='frameworks'")?
            .exists([])?;

        if !table_exists {
            // Run the initial migration
            let migration_sql = MIGRATIONS_DIR
                .get_file("01_initial.sql")
                .context("Migration file not found")?
                .contents_utf8()
                .context("Migration file is not valid UTF-8")?;
            tx.execute_batch(migration_sql)?;
        }
        tx.commit()?;
        Ok(())
    }
}

impl Registry for SqliteRegistry {
    fn framework_supports_language(&self, framework_id: &str, language: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT 1 FROM frameworks WHERE id = ?1 AND language = ?2",
        )?;
        let exists = stmt.exists(params![framework_id, language])?;
        Ok(exists)
    }

    fn get_scaffold_command(&self, framework_id: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT base_scaffold_command FROM frameworks WHERE id = ?1")?;
        let cmd: Option<String> = stmt.query_row(params![framework_id], |row| row.get(0)).optional()?;
        Ok(cmd)
    }

    fn features_for_framework(&self, framework_id: &str) -> Result<Vec<Feature>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT f.id, f.description FROM features f
             JOIN framework_features ff ON ff.feature_id = f.id
             WHERE ff.framework_id = ?1",
        )?;
        let features = stmt
            .query_map(params![framework_id], |row| {
                Ok(Feature {
                    id: row.get(0)?,
                    description: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(features)
    }

    fn is_feature_compatible(&self, framework_id: &str, feature_id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT 1 FROM framework_features WHERE framework_id = ?1 AND feature_id = ?2",
        )?;
        let exists = stmt.exists(params![framework_id, feature_id])?;
        Ok(exists)
    }

    fn get_dependencies(
        &self,
        framework_id: Option<&str>,
        features: &[String],
    ) -> Result<Vec<Dependency>> {
        if features.is_empty() {
            return Ok(vec![]);
        }

        let conn = self.conn.lock().unwrap();
        // Build a parameterized query with IN clause
        let placeholders: Vec<String> = (1..=features.len()).map(|i| format!("?{}", i)).collect();
        let in_clause = placeholders.join(",");

        let mut sql = format!(
            "SELECT package_name, version_constraint, is_dev FROM dependencies
             WHERE feature_id IN ({})",
            in_clause
        );
        // If framework_id is Some, also match NULL (global) or that specific framework
        if let Some(fid) = framework_id {
            sql.push_str(" AND (framework_id IS NULL OR framework_id = ?)");
        } else {
            sql.push_str(" AND framework_id IS NULL");
        }

        let mut params: Vec<&dyn rusqlite::ToSql> = features.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
        if let Some(fid) = framework_id {
            params.push(&fid);
        }

        let mut stmt = conn.prepare(&sql)?;
        let deps = stmt
            .query_map(params.as_slice(), |row| {
                Ok(Dependency {
                    package_name: row.get(0)?,
                    version_constraint: row.get(1)?,
                    is_dev: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(deps)
    }

    fn get_config_mutations(
        &self,
        framework_id: &str,
        features: &[String],
    ) -> Result<Vec<ConfigMutation>> {
        if features.is_empty() {
            return Ok(vec![]);
        }

        let conn = self.conn.lock().unwrap();
        let placeholders: Vec<String> = (1..=features.len()).map(|i| format!("?{}", i)).collect();
        let in_clause = placeholders.join(",");

        let sql = format!(
            "SELECT file_path, mutation_type, content FROM config_mutations
             WHERE framework_id = ?1 AND feature_id IN ({})",
            in_clause
        );

        let mut params: Vec<&dyn rusqlite::ToSql> = vec![&framework_id];
        params.extend(features.iter().map(|s| s as &dyn rusqlite::ToSql));

        let mut stmt = conn.prepare(&sql)?;
        let mutations = stmt
            .query_map(params.as_slice(), |row| {
                Ok(ConfigMutation {
                    file_path: row.get(0)?,
                    mutation_type: row.get(1)?,
                    content: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(mutations)
    }
}
