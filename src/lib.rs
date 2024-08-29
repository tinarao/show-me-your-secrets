mod parser;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn env_file_exists() {
        if fs::metadata("./.env").is_ok() {
            let is_exists = parser::ok();
            assert_eq!(is_exists, true);
        } else {
            let is_exists = parser::ok();
            assert_eq!(is_exists, false);
        }
    }

    #[test]
    fn get_known_vars() {
        if !parser::ok() {
            println!(".env file does not exist!")
        };

        assert_eq!(parser::get("FOO"), "BAR");
        assert_eq!(parser::get("BAR"), "BAZ");
        assert_eq!(parser::get("BAZ"), "BAT");
    }

    #[test]
    fn get_unknown_vars() {
        if !parser::ok() {
            println!(".env file does not exist!")
        };

        assert_eq!(parser::get("UNKNOWN_ONE"), String::new());
        assert_eq!(parser::get("WE_DONT_HAVE_THIS_ONE"), String::new());
        assert_eq!(parser::get("NOTHING"), String::new());
    }
}
