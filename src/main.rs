mod vecoldb;

use vecoldb::Database;
use vecoldb::SqlType;
use vecoldb::Result;

fn main()->Result<()> {
    let mut db = Database::new(String::from("hello"));
    let conn = db.get_connection();
    let res = db.new_collection(&conn,String::from("new_collection"),1024,vec![(String::from("age"),SqlType::INTEGER)]);
    res
}
