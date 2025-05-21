use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

const FILE_BUFFER: LazyLock<Arc<Mutex<HashMap<String, String>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

pub trait FileBuffer {
    async fn read_file(path: String) -> Result<String, String>;
}

impl FileBuffer for HashMap<String, String> {
    async fn read_file(path: String) -> Result<String, String> {
        let mut buffer = FILE_BUFFER.lock().await.clone();
        if buffer.contains_key(&path) {
            Ok(buffer.get(&path).unwrap().to_string())
        } else {
            // Read contents from file
            let mut file = match File::open(&path) {
                Ok(x) => x,
                Err(e) => {
                    return Err(e.to_string());
                }
            };
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_x) => {
                    buffer.insert(path, contents.clone());
                    Ok(contents)
                }
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
