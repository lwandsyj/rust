use crate::config::mysql;

pub fn create_user()->String{
    mysql::MysqlConfig::new().name
}