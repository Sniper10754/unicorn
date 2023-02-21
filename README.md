# Unicorn
blazing fast error reporting library

# Use
## Add to cargo.toml
```toml
[dependencies]
unicorn = { git = "https://github.com/Sniper10754/unicorn.git" }
```

# Examples
```rs
use std::fs;
use unicorn::error::WrapErr;

fn main() {
    let content = fs::read_to_string("foo")
        .wrap_err(Some("could not read file foo"));

    println!("{}", content);
}
```
