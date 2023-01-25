pub mod database;
pub mod entity;
pub mod persistent;
pub mod repository; 

pub use database::Database;
pub use database::GetDb;
pub use entity::*;


macro_rules! vars {
    ($($k:expr => $v:expr),* $(,)?) => {{
        ::std::collections::BTreeMap::from([
            $(($k, $v),)+
        ])
    }};
}
pub(crate) use vars;

mod utils {
    use serde::{de::DeserializeOwned, Serialize};

    use anyhow::Result;
    use surrealdb::sql::Value;

    

    pub trait ToValue<T> {
        fn to_value(&self) -> Result<Value>;
    }

    impl<T> ToValue<T> for T
    where
        T: Serialize,
    {
        fn to_value(&self) -> Result<Value> {
            let value = serde_json::to_string(&self)?;
            Ok(Value::parse(value.as_str()))
        }
    }

    pub trait FromValue<T> {
        fn from_value(value: Value) -> Result<T>;
    }

    impl<T> FromValue<T> for T
    where
        T: DeserializeOwned,
    {
        fn from_value(value: Value) -> Result<T> {
            let val = serde_json::to_value(value)?;
            Ok(serde_json::from_value::<T>(val)?)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{FromValue, ToValue};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Some {
            count: i32,
        }

        #[test]
        fn test_to_value() {
            let value = Some { count: 0 }.to_value();
            dbg!(&value);
        }

        #[test]
        fn test_from_value() {
            let entity = Some { count: 0 };
            let value = entity.to_value().unwrap();
            let value = Some::from_value(value).unwrap();
            dbg!(&value);
            assert_eq!(entity, value);
        }
    }

    use surrealdb::sql::json;
    use surrealdb::sql::thing;
    use surrealdb::sql::Thing;

    pub trait Parse<T> {
        fn parse(val: &str) -> T;
    }

    impl Parse<Value> for Value {
        fn parse(val: &str) -> Value {
            json(val).unwrap()
        }
    }

    impl Parse<Thing> for Thing {
        fn parse(val: &str) -> Thing {
            thing(val).unwrap()
        }
    }
}
