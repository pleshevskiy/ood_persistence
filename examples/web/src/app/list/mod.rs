#[cfg(test)]
pub mod _mocks;

pub mod controller;
pub mod service;
pub mod storage_type;

pub type ListId = i32;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct List {
    pub id: ListId,
    pub name: String,
}
