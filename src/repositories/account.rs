use diesel::prelude::*;
use crate::schema::accounts::dsl::*;
use crate::models::account::Account;


pub fn list(conn: &MysqlConnection) -> Vec<Account> {
  accounts.filter(people_count.gt(30)).load(conn).unwrap()
}