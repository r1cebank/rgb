mod command;
mod debuggable;

use std::thread::{Builder, JoinHandle};

#[cfg(feature = "debug")]
pub fn start_debug_thread() -> JoinHandle<()> {
    use rustyline::error::ReadlineError;
    use rustyline::Editor;
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.log").is_err() {
        println!("No previous history.");
    }
    Builder::new()
        .name("debugger".to_string())
        .spawn(move || {
            debug!("thread spawned");
            loop {
                let readline = rl.readline("[rgb-debug]# ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                    }
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
                rl.save_history("history.log").unwrap();
            }
        })
        .unwrap()
}
