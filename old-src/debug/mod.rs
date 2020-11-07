#[cfg(feature = "tui")]
use cursive::event::Key;
#[cfg(feature = "tui")]
use cursive::menu::MenuTree;
#[cfg(feature = "tui")]
use cursive::traits::*;
#[cfg(feature = "tui")]
use cursive::views::Dialog;
#[cfg(feature = "repl")]
use flume::{Receiver, Sender};
#[cfg(feature = "tui")]
use flume::{Receiver, Sender};
#[cfg(feature = "repl")]
use std::rc::Rc;
#[cfg(feature = "repl")]
use std::thread;
#[cfg(feature = "tui")]
use std::thread;

#[cfg(feature = "tui")]
use crate::cpu::registers::Registers;
#[cfg(feature = "repl")]
use crate::cpu::registers::Registers;

#[derive(Clone, Debug)]
pub enum DebugResult {
    Close,
    Error,
    NotACommand,
    Log,
    Pause,
    Text(String),
    Registers(Registers),
}

#[derive(Clone, Debug)]
pub enum DebugCommand {
    Pause,
    ShowRegister,
}

#[cfg(feature = "tui")]
pub fn setup_tui(tui_sender: Sender<DebugCommand>, ui_receiver: Receiver<DebugResult>) {
    let _ = thread::spawn(move || {
        let mut siv = cursive::default();

        // The menubar is a list of (label, menu tree) pairs.
        siv.menubar()
            .add_subtree(
                "CPU",
                MenuTree::new()
                    // Trees are made of leaves, with are directly actionable...
                    .leaf("Show Registers", move |s| {
                        &tui_sender.send(DebugCommand::ShowRegister).unwrap();
                        match &ui_receiver.recv() {
                            Ok(result) => match result {
                                DebugResult::Registers(registers) => {}
                                _ => {}
                            },
                            Err(_) => {}
                        }
                    }),
            )
            .add_subtree(
                "Help",
                MenuTree::new()
                    .subtree(
                        "Help",
                        MenuTree::new()
                            .leaf("General", |s| s.add_layer(Dialog::info("Help message!")))
                            .leaf("Online", |s| {
                                let text = "Google it yourself!\n\
                                        Kids, these days...";
                                s.add_layer(Dialog::info(text))
                            }),
                    )
                    .leaf("About", |s| s.add_layer(Dialog::info("rgb v0.0.0"))),
            )
            .add_delimiter()
            .add_leaf("Quit", |s| s.quit());

        siv.set_autohide_menu(false);

        siv.add_layer(Dialog::text("rgb debug console"));

        siv.run();
    });
}

#[cfg(feature = "repl")]
pub fn setup_repl(repl_sender: Sender<String>, ui_receiver: Receiver<DebugResult>) {
    use rustyline::error::ReadlineError;
    use rustyline::Editor;
    // Setup repl
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.log").is_err() {
        println!("No previous history.");
    }

    let repl_thread = thread::spawn(move || {
        loop {
            let readline = rl.readline("[rgb-debug]# ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    repl_sender.send(line);
                    match ui_receiver.recv() {
                        Ok(result) => {
                            println!("{:?}", result);
                        }
                        Err(_) => {}
                    }
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
        }
        rl.save_history("history.log").unwrap();
    });
}
