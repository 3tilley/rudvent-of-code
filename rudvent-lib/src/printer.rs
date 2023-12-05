use color_eyre::owo_colors::OwoColorize;

impl Printer {
    pub fn print_or_info(&self, output: &str) {
        println!("{}", output);
    }

    pub fn success(&self, output: &str) {
        println!("\n{}\n", format!("✅ {}", output).green());
    }
}

#[derive(Debug)]
pub struct Printer {}
