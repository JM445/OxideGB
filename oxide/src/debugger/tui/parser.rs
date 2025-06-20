use crate::debugger::full_debugger::*;

use super::ui_utils::*;
use super::*;
use log::debug;

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
                "mem" | "m" => self.parse_mem(&words[1..]),
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
                if let Ok(a) = parse_hex_or_dec(words[1]) {
                    self.debugger.add_breakpoint(Breakpoint::Address(a));
                    debug!("Added a breakpoint at address {}", a);
                    true
                } else {false}
            },
            "mem" | "m" => {
                if let (Ok(a), Ok(v)) = (parse_hex_or_dec(words[1]), parse_hex_or_dec(words[2])) {
                    self.debugger.add_breakpoint(Breakpoint::MemValue(a, v));
                    debug!{"Added a breakpoint when [{a:#06X}] = {v:#04X}"};
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