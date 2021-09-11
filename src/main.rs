#![feature(slice_group_by)]

mod utils;

use rustyline::config::Configurer;
use rustyline::error::*;
use rustyline::{ColorMode, Editor};
use crate::utils::Engine;

fn main() {
    let mut rl = Editor::<()>::new();
    rl.set_color_mode(ColorMode::Enabled);
    let engine = Engine::new();
    loop {
        let line = rl.readline(">> ");
        match line {
            Ok(phrase) => {
                let cleaned = utils::clean_string(phrase);
                let response = engine.reply(&cleaned);
                println!("{}", response);
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
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}
