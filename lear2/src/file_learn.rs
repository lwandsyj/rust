use std::fs::File;
use std::io::{self,Write};

pub  fn learn_file(){

}

pub  fn write_info_file(content:&str,file_name:&str)->io::Result<()>{
    let mut f=File::create(file_name)?;
    f.write_all(content.as_bytes())
}