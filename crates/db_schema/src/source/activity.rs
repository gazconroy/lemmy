use crate::schema::activity;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityData {
  #[serde(rename = "type")]
  pub type_: String,
  // this holds all the fields which are not explicitly listed above, so they can be deserialized
  #[serde(flatten)]
  extra: HashMap<String, Value>,
}

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "activity"]
pub struct Activity {
  pub id: i32,
  pub data: Value,
  pub local: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub ap_id: Option<String>,
  pub sensitive: Option<bool>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "activity"]
pub struct ActivityForm {
  pub data: Value,
  pub local: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  pub ap_id: String,
  pub sensitive: bool,
}
