

use yad_storage::Database;


#[derive(Debug)]
pub struct ScopeState {
    db: Database,
}

impl ScopeState {
    pub fn new(db: Database) -> Self {
        ScopeState {db}
    }
}

impl yad_storage::GetDb for ScopeState {
    fn get_db(&self) ->  &Database {
        &self.db
    }
}