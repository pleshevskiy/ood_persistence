use crate::app::list::List;

pub mod storage;

pub type DbListId = i32;

#[derive(Debug, FromSql)]
#[postgres(name = "lists")]
struct DbList {
    pub id: DbListId,
    pub name: String,
}

impl From<DbList> for List {
    fn from(db: DbList) -> Self {
        Self {
            id: db.id,
            name: db.name,
        }
    }
}
