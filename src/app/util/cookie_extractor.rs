pub fn cookie_extractor(cookies: &str, cookie_key: &str) -> Option<String> {
    match cookies.split(";").find(|x| x.contains(cookie_key)) {
        Some(cookie) => {
            let assignment = &cookie[format!("{}=", cookie_key).len()..];
            Some(assignment.to_string())
        }
        None => None,
    }
}
