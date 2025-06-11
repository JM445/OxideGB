use crate::debugger::full_debugger::*;

use log::{debug};
use super::*;

use std::str::FromStr;
use num_traits::PrimInt;



impl<'a> Ui<'a> {
    pub (super) fn parse_line(&mut self, line: &str) {
        debug!("Received command: {}", line);
        let words : Vec<&str> = line.split_whitespace().collect();

        if words.len() > 0 {
            self.last_cmd = Some(line.to_string());
            match words[0] {
                "quit" => self.exit = true,
                "tick" | "t" | "step" | "s" => {
                    if self.parse_breakpoint(&words) {
                           self.tick();
                    }
                },
                "debug" => {
                    self.debugger.debug_stop = !self.debugger.debug_stop;
                    info!("Debug Break Mode: {}", if self.debugger.debug_stop {"Enabled"} else {"Disabled"});
                },
                "break" | "b" => {self.parse_breakpoint(&words[1..]);},
                "continue" | "c" => self.tick(),
                _ => {
                    self.last_cmd = None;
                    self.cmd_area.insert_str(format!("Error: Unknown command: {}\n> ", line));
                },
            }
        } else if let Some(ref last) = self.last_cmd.clone() {
            self.parse_line(last);
        }
    }

    fn parse_breakpoint(&mut self, words: &[&str]) -> bool {
        if words.len() < 1 || words.len() > 3 {
            self.cmd_area.insert_str("Error: Invalid breakpoint argument count !\nUsage: break type value");
            return false;
        }

        match words[0] {
            "tick" | "t" => {
                if let Some(len) = parse_numeric(words, 1) {
                    self.debugger.add_breakpoint(Breakpoint::Ticks(len));
                    debug!("Added a breakpoint in {} ticks", len);
                    true
                } else {false}
            }
            "step" | "s" => {
                if let Some(len) = parse_numeric(words, 1) {
                    self.debugger.add_breakpoint(Breakpoint::Instructions(len));
                    debug!("Added a breakpoint in {} steps", len);
                    true
                } else {false}
            },
            "addr" | "a" => {
                if let Some(a) = parse_numeric(words, 1) {
                    self.debugger.add_breakpoint(Breakpoint::Address(a));
                    debug!("Added a breakpoint at address {}", a);
                    true
                } else {false}
            },
            _ => {
                error!("Error: Unknown break type: {}", words[0]);
                false
            }
        }
    }

}

fn parse_numeric<T>(words: &[&str], pos: usize) -> Option<T>
where T: PrimInt + FromStr {
    if words.len() > pos {
        if let Ok(n) = words[1].parse() {
            Some(n)
        } else {
            error!("Error: Invalid tick number: {}", words[1]);
            return None;
        }
    } else {
        Some(T::one())
    }
}
