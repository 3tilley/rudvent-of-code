use tracing::{debug, info};
use color_eyre::eyre::eyre;
use std::path::Path;
use crate::cli::App;

static DAY_TEMPLATE: &str = include_str!("../templates/day_template.rs");
static DAY_MOD_TEMPLATE: &str = include_str!("../templates/mod_template.rs");

#[derive(Debug)]
pub struct NewInstructions<'a> {
    pub(crate) day: u8,
    pub(crate) overwrite: bool,
    pub(crate) app: &'a App,
}

impl NewInstructions<'_> {
    pub fn execute(&self) -> color_eyre::Result<()> {
        info!("Creating new day {}", self.day);
        debug!("Instructions: {:?}", self);
        if self.app.days_directory.exists() {
            info!("Days directory exists");
        } else {
            info!("Days directory does not exist");
            fs_err::create_dir_all(&self.app.days_directory)?;
        }
        let day_file = self
            .app
            .days_directory
            .join(self.app.day_format.replace("{day}", &*self.day.to_string()));
        if day_file.exists() {
            info!("Day file exists");
            if self.overwrite {
                info!("Overwriting day file");
                self.create_and_replace(&day_file, DAY_TEMPLATE)?;
            } else {
                info!("Not overwriting day file");
            }
        } else {
            info!("Day file does not exist, creating");
            self.create_and_replace(&day_file, DAY_TEMPLATE)?;
        }

        info!("Updating mod.rs to include new day");
        let mod_file = self.app.days_directory.join("mod.rs");
        if !mod_file.exists() {
            info!("mod.rs doesn't exist, creating");
            let replaced_template = DAY_MOD_TEMPLATE.lines().filter_map(|line| {
                if line.contains("crate::templates") {
                    None
                } else if line.contains("use crate") {
                    Some(line.replace("use crate", "use rudvent_lib"))
                } else {
                    Some(line.to_string())
                }
            }).collect::<Vec<_>>().join("\n");
            fs_err::write(&mod_file, replaced_template).expect("Unable to create mod.rs");
        }
        let mut mod_file_contents = fs_err::read_to_string(&mod_file)?;
        let day_comment = format!("// Day {:0>2}", self.day);
        debug!("Looking for day comment: {}", day_comment);
        let mod_declaration = format!(
            "mod {};",
            self.app
                .day_format
                .replace("{day}", &*self.day.to_string())
                .replace(".rs", "")
        );
        debug!("Looking for mod declaration: {}", mod_declaration);
        let mut changes = 0;
        let mut within_mods = false;
        let mut found_mod_declaration = false;
        let new_lines = mod_file_contents
            .lines()
            .map(|line| {
                if line.contains("// Begin mod declarations") {
                    within_mods = true;
                    line.to_string()
                } else if within_mods & line.contains(&mod_declaration) {
                    found_mod_declaration = true;
                    line.to_string()
                } else if line.contains("// End mod declarations") {
                    within_mods = false;
                    if !found_mod_declaration {
                        changes += 1;
                        format!("{}\n{}", mod_declaration, line)
                    } else {
                        line.to_string()
                    }
                } else if line.contains(&day_comment) & line.contains("None") {
                    // We need a day file relative to the mod file, otherwise too many directories will be included
                    // Giving up on this "better way", just going to hardcode it, I can't imagine the mod file won't be next our day files
                    // let rel_day = pathdiff::diff_paths(&day_file, &mod_file.parent().unwrap()).unwrap();
                    // let new_data = format!("Some({}::make_sol)", pathbuf_to_import_string(&rel_day, None));
                    let new_data = format!(
                        "Some({}::make_sol)",
                        &self
                            .app
                            .day_format
                            .replace("{day}", &*self.day.to_string())
                            .replace(".rs", "")
                    );
                    changes += 1;
                    line.replace("None", &*new_data)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>();
        if changes == 0 {
            return Err(eyre!(
                "No changes made to mod.rs, perhaps day {} has already been included",
                self.day
            ));
        } else if changes > 2 {
            return Err(eyre!("Too many matching lines found in mod.rs, this probably means there has been an error"));
        }
        fs_err::write(mod_file, new_lines.join("\n"))?;
        self.app.printer.success(&format!(
            "Created template for day {} in {}",
            self.day,
            self.app.project_root.clone().map(|d| pathdiff::diff_paths(&day_file, d).unwrap()).unwrap_or(day_file).to_string_lossy(),
        ));
        Ok(())
    }

    fn create_and_replace(&self, target_path: &Path, template_contents: &str) -> color_eyre::Result<()> {
        let updated = template_contents.replace("use crate::", "use rudvent_lib::");
        fs_err::write(&target_path, updated)?;
        Ok(())
    }
}
