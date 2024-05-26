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
pub fn info(message: &str, subsystem: Option<&str>) {
    match subsystem {
        Some(subsystem) => println!("ℹ️ {}{}: {}", "INFO".bold(), format!("({})", subsystem.blue().bold()), message.bright_white().bold()),
        None => println!("ℹ️ {}: {}", "INFO".bold(), message.bright_white().bold())
    }
}