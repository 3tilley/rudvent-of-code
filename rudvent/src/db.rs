use chrono::{DateTime, Utc};

use rudvent_lib::{AdventSolution, AdventSolutionWithMetadata, CodeSource, ExecutionClaim, MachineInfo, SolutionLanguage, User};

pub struct PgAdventSolution {
    pub id: i64,
    pub user_id: i64,
    pub created: DateTime<Utc>,
    pub year: i16,
    pub day: i16,
    pub part: i16,
    pub functioning: bool,
    pub server_side_run_id: Option<i64>,
    pub description: Option<String>,
    pub solution: Option<String>,
    pub language: String,
    pub claim_wall_time_s: Option<f64>,
    pub claim_processors_used: Option<i16>,
    pub machine_os: String,
    pub machine_arch: String,
    pub machine_cpu_name: String,
    pub machine_clock_freq_mhz: Option<i16>,
    pub code_source_repository: Option<String>,
    pub code_source_filename: Option<String>,
    pub code_source_commit_id: Option<String>,
}

impl PgAdventSolution {
    pub fn get_user_details(&self) -> User {
        match self.user_id {
            2 => User { user_id: 2, user_name: "Bob".to_string() },
            3 => User { user_id: 3, user_name: "Charlie".to_string() },
            _ => unimplemented!("No other users supported")
        }

    }

    pub fn to_advent_solution(&self) -> AdventSolutionWithMetadata {
        let code_source = match &self.code_source_repository {
            Some(c) => Some(CodeSource {
                repository: c.to_string(),
                filename: self.code_source_filename.clone(),
                commit_id: self.code_source_commit_id.clone(),
            }),
            None => None,
        };
        let execution_claim = match &self.claim_wall_time_s {
            None => None,
            Some(wall_time) => {
                ExecutionClaim {
                    wall_time_s: *wall_time,
                    processors_used: self.claim_processors_used.expect("If wall time is supplied processors must be") as u8
                }.into()
            }
        };
        AdventSolutionWithMetadata {
            id: self.id as u64,
            created: self.created,
            user: self.get_user_details(),
            solution: AdventSolution {
                year: self.year as u16,
                functioning: self.functioning,
                day: self.day as u16,
                part: self.part as u8,
                description: self.description.clone(),
                solution: self.solution.clone(),
                language: SolutionLanguage::try_from(&*self.language).unwrap(),
                execution_claim,
                machine_info: MachineInfo {
                    os: self.machine_os.parse().unwrap(),
                    arch: self.machine_arch.clone(),
                    cpu_name: self.machine_cpu_name.clone(),
                    clock_freq_mhz: self.machine_clock_freq_mhz.map(|f| f as u16),
                },
                code_source,
                server_side_run: None,
            },
        }
    }
}

impl From<AdventSolutionWithMetadata> for PgAdventSolution {
    fn from(value: AdventSolutionWithMetadata) -> Self {
        PgAdventSolution {
            id: value.id as i64,
            user_id: value.user.user_id as i64,
            created: value.created,
            year: value.solution.year as i16,
            day: value.solution.day as i16,
            part: value.solution.part as i16,
            functioning: value.solution.functioning,
            server_side_run_id: None,
            description: value.solution.description,
            solution: value.solution.solution,
            language: value.solution.language.to_string(),
            claim_wall_time_s: value.solution.execution_claim.as_ref().map(|f| f.wall_time_s),
            claim_processors_used: value.solution.execution_claim.as_ref().map(|f| f.processors_used as i16),
            machine_os: value.solution.machine_info.os.to_string(),
            machine_arch: value.solution.machine_info.arch.to_string(),
            machine_cpu_name: value.solution.machine_info.cpu_name.to_string(),
            machine_clock_freq_mhz: value.solution.machine_info.clock_freq_mhz.map(|f| f as i16),
            code_source_repository: value.solution.code_source.as_ref().map(|s| s.repository.clone()),
            code_source_filename: value.solution.code_source.as_ref().map(|s| s.filename.clone()).flatten(),
            code_source_commit_id: value.solution.code_source.as_ref().map(|s| s.commit_id.clone()).flatten(),
        }
    }
}

// sqlx tests
#[cfg(test)]
mod tests {
    use rudvent_lib::get_solutions;
    use super::*;

    #[sqlx::test]
    async fn test_get_solutions(pool: sqlx::SqlitePool) {
        let mut conn = pool.acquire().await?;
        let solutions = get_solutions();
        sqlx::query_as()
        assert_eq!(solutions.len(), 2);
        assert_eq!(solutions[0].id, 1);
        assert_eq!(solutions[1].id, 2);
    }
}