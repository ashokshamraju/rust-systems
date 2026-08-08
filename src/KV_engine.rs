use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, Write};


pub struct KVEngine {
    file_path: String,
    index: HashMap<String, String>,
}

impl KVEngine {

    pub fn new(file_path:String)->Self {
        Self { 
            file_path, 
            index: HashMap::new(), 
        }        
    }

    pub fn set(&mut self, key:String, value:String) -> Result<(),String> {
        let mut file = OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .write(true)
                                    .open(&self.file_path)
                                    .map_err(|e|e.to_string())?;
        
        let row_string = format!("{},{}\n",key,value);

        file.write_all(row_string.as_bytes()).map_err(|e|e.to_string())?;

        self.index.insert(key,value);

        Ok(())
    }

    pub fn get(&self, key:String)->Option<String> {
        if let Some(value) = self.index.get(&key) {
            Some(value.clone())
        }
        else {
            None
        }
    }

    pub fn load_from_disk(&mut self)-> Result<(), String> {
        
        let file = File::open(&self.file_path);
        match file {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Ok(())
            },

            Err(other_error) =>{
                Err(other_error.to_string())
            },
            Ok(opened_file) => {
                let reader = BufReader::new(opened_file);

                for lines in reader.lines() {
                    let line = lines.map_err(|e|e.to_string())?;

                    if let Some((k,v)) =  line.split_once(','){
                        self.index.insert(k.to_string(), v.to_string());
                    }
                }
                Ok(())
            }
        }
    }
    
}


fn main() {
        let db_filename = "storage.db".to_string();
    
    // Clean up old database logs from previous test iterations
    let _ = std::fs::remove_file(&db_filename);

    println!("--- Phase 1: Bootstrapping Empty Engine ---");
    let mut db = KVEngine::new(db_filename.clone());
    db.load_from_disk().unwrap();

    println!("\n--- Phase 2: Committing Database Transactions ---");
    db.set("user_101".to_string(), "Alice".to_string()).unwrap();
    db.set("user_102".to_string(), "Bob".to_string()).unwrap();
    // Overwriting user_101 to check if updating state works
    db.set("user_101".to_string(), "Charlie".to_string()).unwrap();

    println!("\n--- Phase 3: Validating Fast Cache Reads ---");
    match db.get("user_101".to_string()) {
        Some(val) => println!("Success: Found key 'user_101' -> {}", val), // Must be Charlie!
        None => println!("Error: Key 'user_101' missing from database."),
    }

    println!("\n--- Phase 4: Simulating System Crash & Re-Boot Recovery ---");
    // Instantiate an entirely new engine pointing to the exact same file on disk
    let mut crashed_db = KVEngine::new(db_filename);
    
    // Recover state by reading the raw physical log file transactions back into RAM
    crashed_db.load_from_disk().unwrap();

    println!("Recovery verification check:");
    match crashed_db.get("user_101".to_string()) {
        Some(val) => println!("Success: Recovered key 'user_101' from disk log -> {}", val), // Should be Charlie!
        None => println!("Error: State recovery failed. File log corrupted or unread."),
    }
    match crashed_db.get("user_102".to_string()) {
        Some(val) => println!("Success: Recovered key 'user_102' from disk log -> {}", val), // Should be Bob!
        None => println!("Error: State recovery failed. File log corrupted or unread."),
    }
}