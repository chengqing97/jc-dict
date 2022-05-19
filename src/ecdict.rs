use super::{LookupError, LookupResult};
use rusqlite::{Connection, OptionalExtension};

pub struct Ecdict {
    db: Connection,
}

impl Ecdict {
    pub fn new() -> Self {
        Self {
            db: Connection::open("../database/stardict.db").unwrap(),
        }
    }
    pub fn lookup(self, to_search: &str) -> Result<Option<LookupResult>, LookupError> {
        let mut statement = self
            .db
            .prepare("SELECT phonetic, translation FROM stardict WHERE word = ?")
            .unwrap();

        let result = statement
            .query_row([to_search], |result| {
                Ok(LookupResult {
                    uk_phonetic: None,
                    us_phonetic: result.get(0)?,
                    definition: result.get(1)?,
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
