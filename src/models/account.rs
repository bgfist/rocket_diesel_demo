use diesel::Queryable;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Account {
  id: i32,
  address: String,
  charge_user_id: Option<i32>,
  created_at: SystemTime,
  intention: Option<u8>,
  is_cooperation: Option<u8>,
  is_follow: Option<bool>,
  is_know_all: bool,
  is_sign: bool,
  last_charge_user_id: Option<i32>,
  name: String,
  recently_activity_date: SystemTime,
  school_id: i32,
  sign_type: u8,
  signed_at: SystemTime,
  start_activity_date: SystemTime,
  updated_at: SystemTime,
  area_id: i32,
  recently_activity_id: i32,
  start_activity_id: i32,
  city: String,
  city_id: String,
  people_count: i32,
  province: String,
  province_id: i32,
  level: u8,
  nature: u8,
}
