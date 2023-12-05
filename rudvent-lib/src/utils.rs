use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use std::path::{Component, Path, PathBuf};

pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn release_time_for_problem(year: u16, day: u8) -> Result<DateTime<Utc>, DateTime<Utc>> {
    let release_time = FixedOffset::west_opt(5 * 3600)
        .unwrap()
        .with_ymd_and_hms(year as i32, 12, day as u32, 0, 0, 0)
        .unwrap();
    if release_time < Utc::now() {
        Ok(release_time.with_timezone(&Utc))
    } else {
        Err(release_time.with_timezone(&Utc))
    }
}

pub fn pathbuf_to_import_string(path: &Path, final_component: Option<&str>) -> String {
    let mut output = Vec::new();
    let mut found_src = false;
    path.components().for_each(|c| match c {
        Component::Normal(s) => {
            if found_src {
                output.push(s.to_string_lossy().to_string());
            } else {
                if s == "src" {
                    found_src = true;
                }
            }
        }
        _ => {}
    });
    match final_component {
        Some(s) => output.push(s.to_string()),
        None => {}
    }
    let output = output.join("::");
    output.replace(".rs", "")
}

// Write some tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("hello world"), "Hello World");
        assert_eq!(title_case("hello_world"), "Hello_world");
    }

    #[test]
    fn test_pathbuf_to_import_string() {
        assert_eq!(
            pathbuf_to_import_string(&PathBuf::from("./runner/work/rudvent/src/days/day_1.rs")),
            "days::day_1"
        );
        assert_eq!(
            pathbuf_to_import_string(&PathBuf::from("./runner/work/rudvent/src/days/day_1")),
            "days::day_1"
        );
        assert_eq!(
            pathbuf_to_import_string(&PathBuf::from("./runner/work/rudvent/src/days/day_1.rs")),
            "days::day_1"
        );
    }
}
