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
pub fn info(message: impl Into<String>, subsystem: Option<&str>) {
    match subsystem {
        Some(subsystem) => println!("ℹ️ {}{}: {}", "INFO".bold(), format!("({})", subsystem.blue().bold()), message.into().bright_white().bold()),
        None => println!("ℹ️ {}: {}", "INFO".bold(), message.into().bright_white().bold())
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
pub fn error(message: impl Into<String>, subsystem: Option<&str>) {
    match subsystem {
        Some(subsystem) => println!("❌ {}{}: {}", "ERROR".red().bold(), format!("({})", subsystem.blue().bold()), message.into().bright_red().bold()),
        None => println!("❌ {}: {}", "ERROR".red().bold(), message.into().bright_red().bold())
    }
}