use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, SystemExt};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn get_solutions() -> Vec<AdventSolutionWithMetadata> {
    let mut vec = Vec::new();

    let solution = AdventSolution {
        year: 2023,
        functioning: true,
        description: None,
        day: 1,
        part: 1,
        solution: Some("Hello World".to_string()),
        language: SolutionLanguage::Rust,
        execution_claim: Some(ExecutionClaim {
            wall_time_s: 0.5,
            processors_used: 1,
        }),
        machine_info: Default::default(),
        code_source: None,
        server_side_run: None,
    };
    let solution_2 = AdventSolution {
        year: 2023,
        functioning: true,
        description: None,
        day: 1,
        part: 2,
        solution: Some("Answer to part 2".to_string()),
        language: SolutionLanguage::Rust,
        execution_claim: Some(ExecutionClaim {
            wall_time_s: 2.0,
            processors_used: 2,
        }),
        machine_info: Default::default(),
        code_source: None,
        server_side_run: None,
    };

    vec.push(AdventSolutionWithMetadata {
        solution,
        id: 1,
        created: Utc::now(),
        user: User {
            user_id: 2,
            user_name: "Bob".to_string(),
        }
    });
    vec.push(AdventSolutionWithMetadata {
        solution: solution_2,
        id: 3,
        created: Utc::now(),
        user: User {
            user_id: 3,
            user_name: "Charlie".to_string(),
        },
    });
    vec
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub user_id: u64,
    pub user_name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display)]
pub enum SolutionLanguage {
    Python,
    Rust,
    Other(String),
}

impl TryFrom<&str> for SolutionLanguage {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "python" => Ok(SolutionLanguage::Python),
            "rust" => Ok(SolutionLanguage::Rust),
            "" => Err(()),
            // TODO: This is really ugly, needs testing, and needs fixing
            x if x.starts_with("Other(") => Ok(SolutionLanguage::Other(x.replace("Other(", "").replace(")", ""))),
            _ => Ok(SolutionLanguage::Other(value.to_string())),
        }
    }
}


#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, EnumString, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum OS {
    Linux,
    Macos,
    Ios,
    Freebsd,
    Dragonfly,
    Netbsd,
    Openbsd,
    Solaris,
    Android,
    Windows,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MachineInfo {
    pub os: OS,
    pub arch: String,
    pub cpu_name: String,
    pub clock_freq_mhz: Option<u16>,
}

impl Default for MachineInfo {
    fn default() -> Self {
        let os = OS::from_str(std::env::consts::OS).unwrap();
        let arch = std::env::consts::ARCH.to_string();
        let mut sys2 = sysinfo::System::new_all();
        let clock_freq = sys2.cpus().iter().map(|c| c.frequency()).max().unwrap();
        // It should be possible to do this without refreshing the whole world, but I can't see how
        // to do it without it just returning frequency = 0
        MachineInfo {
            os,
            arch,
            cpu_name: sys2.global_cpu_info().brand().to_string(),
            clock_freq_mhz: if clock_freq == 0 { None} else {Some(clock_freq as u16)},
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdventSolution {
    pub year: u16,
    pub functioning: bool,
    pub day: u16,
    pub part: u8,
    pub description: Option<String>,
    pub solution: Option<String>,
    pub language: SolutionLanguage,
    pub execution_claim: Option<ExecutionClaim>,
    pub machine_info: MachineInfo,
    pub code_source: Option<CodeSource>,
    pub server_side_run: Option<ServerSideCheck>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionClaim {
    pub wall_time_s: f64,
    pub processors_used: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdventSolutionWithMetadata {
    pub id: u64,
    pub created: DateTime<Utc>,
    pub user: User,
    pub solution: AdventSolution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerSideCheck {
    pub run_id: u64,
    pub success: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSource {
    pub repository: String,
    pub filename: Option<String>,
    pub commit_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_from_string() {
        let os = "wINdows";
        let actual = OS::from_str(os);
        assert_eq!(actual, Ok(OS::Windows))
    }

    #[test]
    fn test_machine_info() {
        let info = MachineInfo::default();
        println!("{:?}", info );
    }
}