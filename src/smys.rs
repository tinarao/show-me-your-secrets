use std::{fs, path::Path};

pub struct Parser {}

impl Parser {
    /// Checks, if .env file exists.
    /// Returns ```true``` if so, else ```false```.
    pub fn ok() -> bool {
        let path = Path::new("./.env");
        fs::metadata(path).is_ok()
    }

    fn read_env() -> Vec<String> {
        let mut result = Vec::new();
        let mut result_str = String::new();

        match fs::read("./.env") {
            Ok(v) => {
                for byte in v.iter() {
                    let ch = char::from(*byte);
                    result_str.push(ch)
                }
            }
            Err(_err) => (),
        };

        let splitted = result_str.split("\r\n");
        for word in splitted {
            result.push(String::from(word));
        }

        result
    }

    /// Get's environment variable value by key.
    /// Returns ```String```. If variable does not exists, returns empty string.
    /// # Examples
    /// ```
    /// let bar = parser::get("FOO");
    /// assert_eq!(bar, "BAR"); // true
    ///
    /// let unknown = parser::get("NOT_DECLARED_VAR") // ""
    /// assert_eq!(unknown. "") // true
    /// ```
    pub fn get(key: &str) -> String {
        let mut result = String::new();
        if !Parser::ok() {
            return result;
        }

        let kv_pairs = Parser::read_env();

        for kv in kv_pairs {
            if kv.len() > key.len() && &kv[0..key.len()] == key {
                result.push_str(&kv[key.len() + 1..]);
                break;
            }
        }

        result
    }
}
