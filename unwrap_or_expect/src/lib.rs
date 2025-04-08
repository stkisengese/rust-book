pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server {
        Ok(url) => url.to_string(),
        Err(error) => match security_level {
            Security::Unknown => panic!("{error}"),
            Security::Message => panic!("ERROR: program stops"),
            Security::Warning => format!("WARNING: check the server"),
            Security::NotFound => format!("Not found: {error}"),
            Security::UnexpectedUrl => panic!("{error:?}"),
        },
    }
}
