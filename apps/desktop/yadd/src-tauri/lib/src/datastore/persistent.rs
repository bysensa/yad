// use serde::{Deserialize, Serialize};
// use surrealdb::sql::Thing;

// use super::Database;

// pub trait PersistentEntity {
//     fn id(&self) -> Option<String>;

//     fn collection(&self) -> String;
// }

// #[derive(Debug, PartialEq)]
// pub struct Persistent {
//     id: String,
//     value: serde_json::Value,
// }

// impl Persistent {
//     pub fn try_from<T>(value: &T) -> anyhow::Result<Persistent>
//     where
//         T: Serialize + PersistentEntity,
//     {
//         let collection = value.collection();
//         let id = match value.id() {
//             Some(id) => id,
//             None => Thing::from((collection, Database::next_id())).to_raw(),
//         };
//         let value = serde_json::to_value(value)?;
//         Ok(Persistent { id, value })
//     }

//     pub fn try_extract<T>(self) -> anyhow::Result<T>
//     where
//         T: for<'a> Deserialize<'a>,
//     {
//         let target = serde_json::from_value::<T>(self.value)?;
//         Ok(target)
//     }

//     pub fn to_string(&self) -> anyhow::Result<String> {
//         let res = serde_json::to_string(&self.value)?;
//         Ok(res)
//     }

//     pub fn with_value(self, value: serde_json::Value) -> Self {
//         Self { value, ..self }
//     }

//     pub fn id(&self) -> &String {
//         &self.id
//     }

//     pub fn from_raw(id: String, value: serde_json::Value) -> Self {
//         Persistent { id, value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use serde::{Deserialize, Serialize};

//     use crate::persistent::PersistentEntity;

//     use super::Persistent;

//     #[derive(Serialize, Deserialize, Debug)]
//     struct Someth {
//         value: i32,
//     }

//     impl PersistentEntity for Someth {
//         fn id(&self) -> Option<String> {
//             None
//         }

//         fn collection(&self) -> String {
//             String::from("someth")
//         }
//     }

//     #[test]
//     fn test_try_from() {
//         let val = Someth { value: 0 };
//         let _persist = Persistent::try_from(&val).unwrap();
//     }

//     #[test]
//     fn test_try_extract() {
//         let val = Someth { value: 0 };
//         let persist = Persistent::try_from(&val).unwrap();
//         let _val: Someth = persist.try_extract().unwrap();
//     }

//     #[test]
//     fn test_to_string() {
//         let val = Someth { value: 0 };
//         let persist = Persistent::try_from(&val).unwrap();
//         let _string = persist.to_string().unwrap();
//     }

//     #[test]
//     fn test_with_value() {
//         let val = Someth { value: 0 };
//         let persist = Persistent::try_from(&val).unwrap();
//         let val = persist.with_value(serde_json::to_value(Someth { value: 1 }).unwrap());
//         let val: Someth = val.try_extract().unwrap();
//         assert_eq!(val.value, 1);
//     }
// }
