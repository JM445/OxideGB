use super::*;

use std::fmt;
use std::fmt::format;

impl fmt::Display for DebugEvent {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            DebugEvent::MicroOpEnd(op) => format!("MicroOpEnd({op})"),
            DebugEvent::IrPrefetch(ir, ..) => format!("Prefetch({ir:#02X})"),
            DebugEvent::InstructionEnd(ir) => format!("InstructionEnd({ir:#02X})"),
            DebugEvent::Register16Change(reg, value) => format!("RegChange({value:#04X} => {reg})"),
            DebugEvent::Register8Change(reg, value) => format!("RegChange({value:#02X} => {reg})"),
            DebugEvent::PpuModeChanged(mode) => format!("PpuModeChange({mode})"),
            DebugEvent::SpriteScanned(sprite) => format!("SpriteScanned({sprite})"),
        };

        write!(f, "{}", s)
    }
}
