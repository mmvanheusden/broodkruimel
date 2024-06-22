use colored;
use colored::Colorize;

/**
Logs info

**Example usage:**
```rust
info("The BlueTooth device is ready to pare", Some("bluetooth"))
```

Results in: "`ℹ️ INFO(bluetooth): The BlueTooth device is ready to pare`"
 **/
pub fn info(message: impl Into<String>, tag: Option<impl Into<String>>) {
    let time = chrono::Utc::now().naive_utc().format("%F %T").to_string();
    match tag {
        Some(tag) => println!("{} {} {}: {}", time, "INFO".green().bold(), format!("[{}]", tag.into().yellow().bold()), message.into().bright_white().bold()),
        None => println!("{} {}: {}", time, "INFO".green().bold(), message.into().bright_white().bold())
    }
}

/**
Logs errors

**Example usage:**
```rust
error("The BlueTooth device has exploded!", Some("bluetooth"))
```

Results in: "`❌ ERROR(bluetooth): The BlueTooth device has exploded!`"
 **/
pub fn error(message: impl Into<String>, tag: Option<impl Into<String>>) {
    let time = chrono::Utc::now().naive_utc().format("%F %T").to_string();
    match tag {
        Some(tag) => println!("{} {} {}: {}", time, "ERROR".red().bold(), format!("[{}]", tag.into().yellow().bold()), message.into().bright_red().bold()),
        None => println!("{} {}: {}", time, "ERROR".red().bold(), message.into().bright_red().bold())
    }
}