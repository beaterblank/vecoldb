mod vecoldb{
    
    use rusqlite::Connection;
    pub use rusqlite::Result;
    use std::collections::HashMap;
    use std::fs::File;
    
    pub enum SqlType{
        TEXT,
        INTEGER,
        REAL,
        BLOB,
        NULL,
    }
    impl SqlType {
        pub fn to_string(&self)->&str{
            match self{
                SqlType::TEXT => "TEXT",
                SqlType::INTEGER => "INTEGER",
                SqlType::REAL => "REAL",
                SqlType::BLOB => "BLOB",
                _ => "NULL",
            }
        }
    }

    struct Collection{
        collection_name: String,
        dim: u16,
        columns:Vec<(String, SqlType)>,
    }


    impl Collection {
        pub fn new ( 
            collection_name:String, dim: u16, 
            columns:Vec<(String, SqlType)>,
        ) -> Self {
            Collection{
                collection_name,
                dim,
                columns,
            }
        }

        pub fn create(&self,conn:&Connection)->Result<()>{
            let mut create_sql = format!("CREATE TABLE IF NOT EXISTS {} (",self.collection_name);
            for (column_name,data_type) in &self.columns{
                let column_definition = format!("{} {}, ",column_name,data_type.to_string());
                create_sql.push_str(&column_definition);
            }
            let length_to_keep = create_sql.len() - String::from(", ").len();
            create_sql.truncate(length_to_keep);
            create_sql.push_str(");");
            conn.execute(&create_sql,[])?;
            //create columns
            for i in 0..self.dim{
                let file_name = format!("{}/{}.txt",self.collection_name,i);
                let file = File::create(file_name);
            }
            Ok(())
        }

    }


    pub struct Database{
        database_name: String,
        collections: HashMap<String,Collection>,
    }


    impl Database {
        pub fn new(database_name:String) -> Self{
            Database{
                database_name,
                collections:HashMap::new(),
            }
        }
        pub fn get_connection(&self)->Connection{
            let db_name = format!("{}.db",&self.database_name);
            Connection::open(&db_name).expect("Could not open db")
        }
        pub fn new_collection(
            &mut self,conn:&Connection,
            collection_name:String, dim: u16, 
            columns:Vec<(String, SqlType)>,
        )->Result<()>{
            let new_collection = Collection::new(
                collection_name.clone(),dim,columns
            );
            let res = new_collection.create(conn);
            self.collections.insert(
                collection_name,
                new_collection
            );
            res
        }
    }
}
