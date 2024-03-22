use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut indent = 0;
    let mut indent_increasing = true;

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {
        let text = format!("{:>width$}********", "", width = indent);
        println!("{}", text);
        thread::sleep(time::Duration::from_millis(1_00));

        if indent_increasing == true {
            indent += 1;
            if indent == 20 {
                indent_increasing = false
            }
        } else {
            indent -= 1;
            if indent == 0 {
                indent_increasing = true
            }
        }
    }
    println!("Got it! Exiting...");

}
