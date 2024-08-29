mod smys;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use smys::Parser;

    #[test]
    fn env_file_exists() {
        if fs::metadata("./.env").is_ok() {
            let is_exists = Parser::ok();
            assert_eq!(is_exists, true);
        } else {
            let is_exists = Parser::ok();
            assert_eq!(is_exists, false);
        }
    }

    #[test]
    fn get_known_vars() {
        if !Parser::ok() {
            println!(".env file does not exist!")
        };

        assert_eq!(Parser::get("FOO"), "BAR");
        assert_eq!(Parser::get("BAR"), "BAZ");
        assert_eq!(Parser::get("BAZ"), "BAT");
    }

    #[test]
    fn get_unknown_vars() {
        if !Parser::ok() {
            println!(".env file does not exist!")
        };

        assert_eq!(Parser::get("UNKNOWN_ONE"), String::new());
        assert_eq!(Parser::get("WE_DONT_HAVE_THIS_ONE"), String::new());
        assert_eq!(Parser::get("NOTHING"), String::new());
    }
}
