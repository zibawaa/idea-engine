//! SQLite persistence for chats, recipes, scores, feedback

use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRow {
    pub id: String,
    pub title: String,
    pub template_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRow {
    pub id: String,
    pub chat_id: String,
    pub role: String,
    pub content: String,
    pub idea_bundles_json: Option<String>,
    pub feedback: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeRow {
    pub id: String,
    pub name: String,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub rubric_json: String,
    pub few_shot_examples_json: Option<String>,
    pub created_at: String,
}

pub struct Storage {
    conn: Mutex<Connection>,
}

impl Storage {
    pub fn new(db_path: impl AsRef<Path>) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        Self::init_schema(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn init_schema(conn: &Connection) -> SqlResult<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS chats (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                template_id TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                chat_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                idea_bundles_json TEXT,
                feedback TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (chat_id) REFERENCES chats(id)
            );
            CREATE TABLE IF NOT EXISTS recipes (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                system_prompt TEXT NOT NULL,
                user_prompt_template TEXT NOT NULL,
                rubric_json TEXT NOT NULL,
                few_shot_examples_json TEXT,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS eval_results (
                id TEXT PRIMARY KEY,
                recipe_id TEXT NOT NULL,
                problem_id TEXT NOT NULL,
                bundle_id TEXT NOT NULL,
                score_card_json TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_messages_chat ON messages(chat_id);
            CREATE INDEX IF NOT EXISTS idx_eval_recipe ON eval_results(recipe_id);
            "#,
        )?;
        Ok(())
    }

    pub fn create_chat(&self, id: &str, title: &str, template_id: Option<&str>) -> SqlResult<()> {
        let now = chrono_utc_now();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO chats (id, title, template_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
            params![id, title, template_id, now],
        )?;
        Ok(())
    }

    pub fn list_chats(&self) -> SqlResult<Vec<ChatRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, template_id, created_at, updated_at FROM chats ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ChatRow {
                id: row.get(0)?,
                title: row.get(1)?,
                template_id: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        rows.collect()
    }

    pub fn get_chat_messages(&self, chat_id: &str) -> SqlResult<Vec<MessageRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, chat_id, role, content, idea_bundles_json, feedback, created_at FROM messages WHERE chat_id = ?1 ORDER BY created_at ASC",
        )?;
        let rows = stmt.query_map(params![chat_id], |row| {
            Ok(MessageRow {
                id: row.get(0)?,
                chat_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                idea_bundles_json: row.get(4)?,
                feedback: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        rows.collect()
    }

    pub fn insert_message(
        &self,
        id: &str,
        chat_id: &str,
        role: &str,
        content: &str,
        idea_bundles_json: Option<&str>,
    ) -> SqlResult<()> {
        let now = chrono_utc_now();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO messages (id, chat_id, role, content, idea_bundles_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, chat_id, role, content, idea_bundles_json, now],
        )?;
        conn.execute("UPDATE chats SET updated_at = ?1 WHERE id = ?2", params![now, chat_id])?;
        Ok(())
    }

    pub fn set_message_feedback(&self, message_id: &str, feedback: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE messages SET feedback = ?1 WHERE id = ?2", params![feedback, message_id])?;
        Ok(())
    }

    pub fn save_recipe(&self, recipe: &RecipeRow) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO recipes (id, name, system_prompt, user_prompt_template, rubric_json, few_shot_examples_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                recipe.id,
                recipe.name,
                recipe.system_prompt,
                recipe.user_prompt_template,
                recipe.rubric_json,
                recipe.few_shot_examples_json,
                recipe.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_recipes(&self) -> SqlResult<Vec<RecipeRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, system_prompt, user_prompt_template, rubric_json, few_shot_examples_json, created_at FROM recipes ORDER BY name",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(RecipeRow {
                id: row.get(0)?,
                name: row.get(1)?,
                system_prompt: row.get(2)?,
                user_prompt_template: row.get(3)?,
                rubric_json: row.get(4)?,
                few_shot_examples_json: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        rows.collect()
    }
}

fn chrono_utc_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    secs.to_string()
}
