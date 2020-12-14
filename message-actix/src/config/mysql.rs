#[derive(Default)]
pub struct MysqlConfig{
   pub host:String,
   pub port:Option<i32>,
   pub name:String,
   pub password:String,
}

impl MysqlConfig{
    pub fn new()->Self{
        Self{
            host:"127.0.0.1".to_string(),
            name:"root".to_string(),
            port:Some(6379),
            password:"".to_string()
            
        }
    }
}