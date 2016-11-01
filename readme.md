# Log Update

Log by overwriting the previous output in the terminal.

Useful for rendering progress bars, animations, etc.

## Usage

This example will count down from 3 and then display `Lift off! 🚀`.

```rust
extern crate log_update;

use log_update::LogUpdate;

use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Construct the log updater
    let mut log_update = LogUpdate::new(stdout()).unwrap();

    for i in [3, 2, 1].iter() {
        // Update log to show message
        log_update.render(&format!("Lift off in {}...", i)).unwrap();

        // Sleep for one second
        sleep(Duration::from_secs(1));
    }

    // Print final message
    log_update.render("Lift off! 🚀").unwrap();
}
```

## API

See [documentation](https://docs.rs/log-update)

## Inspiration

Heavily inspired by [@sindresorhus](http://sindresorhus.com/)’ excellent [log-update](https://npmjs.org/log-update).
