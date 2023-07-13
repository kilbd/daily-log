use std::{
    fs::{rename, File},
    io::{BufRead, BufReader, BufWriter, Write},
    os::unix::process::CommandExt,
    path::Path,
};

use chrono::{Days, Local};
use color_eyre::eyre::{eyre, Result};

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

pub fn close_log(config: Config) -> Result<()> {
    let now = Local::now();
    let tomorrow = now
        .checked_add_days(Days::new(1))
        .ok_or(eyre!("Chrono believes tomorrow does not exist."))?;
    let month_log_path = Path::new(&config.base_dir).join(format!("{}.md", now.format("%Y-%m")));
    let today_log_path = Path::new(&config.base_dir).join("today.md");
    let mut incomplete_tasks: Vec<String> = vec![];
    // Reading today's log in a block so it's closed afterwards
    {
        let today_log = File::options().read(true).open(&today_log_path)?;
        let today = BufReader::new(&today_log);
        let month_log = File::options()
            .append(true)
            .create(true)
            .open(month_log_path)?;
        let mut month = BufWriter::new(&month_log);
        if month_log.metadata()?.len() == 0 {
            writeln!(month, "# {}", now.format("%B %Y"))?;
        }
        writeln!(month, "\n")?;
        for line in today.lines() {
            let line = line?;
            if line.contains("- [ ]") {
                incomplete_tasks.push(line);
            } else {
                writeln!(month, "{}", line)?;
            }
        }
        dbg!(&incomplete_tasks);
        writeln!(month, "\n")?;
    }
    // Want to pre-create log for tomorrow if any undone tasks
    if !incomplete_tasks.is_empty() {
        let tomorrow_log = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&today_log_path)?;
        let mut today = BufWriter::new(tomorrow_log);
        writeln!(today, "## {}", tomorrow.format("%Y-%m-%d"))?;
        writeln!(today, "\n")?;
        writeln!(today, "## Tasks\n")?;
        for task in incomplete_tasks {
            writeln!(today, "{}", task)?;
        }
        writeln!(today, "\n")?;
        writeln!(today, "## Notes\n")?;
    }
    Ok(())
}
