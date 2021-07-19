use regex::Regex;

pub fn get_season(useragent: &str) -> Option<&str> {
    let regex = Regex::new(r"\+\+Fortnite\+Release-(\d+)\.(\d+).*-CL").ok()?;
    let capture = regex.captures(useragent)?.get(1)?;

    Some(capture.as_str())
}
