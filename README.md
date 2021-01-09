# Progress
A simple progress bar written in Rust.
# Demo
![ProgressBar](https://user-images.githubusercontent.com/58354812/104094954-73d95000-52d7-11eb-80b6-9a8dfdda7001.png)
# Installation
```toml
[dependencies]
progress = { git = "https://github.com/yuta0709/progress/", branch = "main"}
```
# Example
```rust
use progress::{ProgressBar};
use std::{thread,time};
fn main() {
    let mut progress = ProgressBar::new();
    progress.set_text("Doing something...");
    for _ in 0..100{
        thread::sleep(time::Duration::from_millis(50));
        progress.increment_and_show();
    }
    print!("\n")
}
```
