use std::thread;
use std::process::Command;


fn main() {

    let mut counter = 0u32;

    // Spawn 5 child processes (they get their own PIDs)
    loop {
        counter += 1;
        println!("counter before: {}", counter);
        thread::spawn(move || {
            Command::new("sleep")
                .arg("1m")
                .output()
                .unwrap_or_else(|e| {
                    panic!("Failed to execute process: {}", e)
                })
        });

        if counter > 4 {
            break; // This is apparently enough to stop more processes from spawning
            // But not enough to end the infinite loop even after children have exited?
        }
    }
}
