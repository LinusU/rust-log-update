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
