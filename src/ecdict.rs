use crate::{LookupError, LookupResult};
use rusqlite::{Connection, OpenFlags, OptionalExtension};

pub struct Ecdict {
    db: Connection,
}

impl Ecdict {
    pub fn new() -> Self {
        Self {
            db: Connection::open_with_flags(
                "/opt/jc-dict/database/stardict.db",
                OpenFlags::SQLITE_OPEN_READ_ONLY,
            )
            .unwrap(),
        }
    }

    pub fn get(&self, to_search: &str) -> Result<Option<LookupResult>, LookupError> {
        let mut statement = self
            .db
            .prepare("SELECT phonetic, translation FROM stardict WHERE word = ?")
            .unwrap();

        let result = statement
            .query_row([to_search], |row| {
                let phonetic: Option<String> = match row.get::<_, String>(0) {
                    Ok(value) => {
                        if value.is_empty() {
                            None
                        } else {
                            Some(format!("[{}]", value))
                        }
                    }
                    Err(_) => None,
                };
                Ok(LookupResult {
                    uk_phonetic: None,
                    us_phonetic: phonetic,
                    definition: row.get(1)?,
                    suggestions: None,
                })
            })
            .optional();

        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(LookupError {
                message: e.to_string(),
            }),
        }
    }
}
