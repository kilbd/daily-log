use std::{
    fs::{rename, File},
    io::{BufRead, BufReader, Write},
    os::unix::process::CommandExt,
    path::Path,
};

use chrono::Local;
use color_eyre::eyre::Result;

use crate::config::Config;

const TEMPLATE: &str = r#"
### Tasks

- [ ] 

### Notes


"#;

pub fn open_log(config: Config, reset: bool) -> Result<()> {
    let now = Local::now();
    let header = format!("## {}", now.format("%Y-%m-%d"));
    let read_path = Path::new(&config.base_dir).join("today.md");
    let write_path = Path::new(&config.base_dir).join("today.temp.md");
    let existing_log = File::options().read(true).open(&read_path);
    let mut today_log = File::options().write(true).create(true).open(&write_path)?;
    // If there's already a log for today, update the date if requested...
    if let Ok(existing) = existing_log {
        let mut lines = BufReader::new(&existing).lines();
        if let Some(first_line) = lines.next() {
            let first_line = first_line?;
            if first_line != header && reset {
                writeln!(today_log, "{}", header)?;
                for line in lines {
                    writeln!(today_log, "{}", line?)?;
                }
                rename(&write_path, &read_path)?;
            }
        }
    // ...otherwise, create new log file from template.
    } else {
        writeln!(today_log, "{}", header)?;
        write!(today_log, "{}", TEMPLATE)?;
        rename(&write_path, &read_path)?;
    }
    // Open file for editing
    std::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "{} {}",
            config.editor.unwrap(),
            read_path.display()
        ))
        .exec();
    Ok(())
}
