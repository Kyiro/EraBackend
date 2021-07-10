use regex::Regex;

pub fn get_season(useragent: &str) -> Option<&str> {
    let regex = match Regex::new(r"\+\+Fortnite\+Release-(\d+)\.(\d+).*-CL") {
        Ok(data) => data,
        Err(_) => return None,
    };
    let capture = match regex.captures(useragent) {
        Some(data) => match data.get(1) {
            Some(data) => data,
            None => return None,
        },
        None => return None,
    };

    Some(capture.as_str())
}
