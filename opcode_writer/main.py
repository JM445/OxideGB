#!/usr/bin/env python3

import pprint
import os
import json
import requests
from dataclasses import dataclass
from typing import List, Optional, Dict

@dataclass
class Opcode:
    mnemonic: str
    length: int
    operand1: Optional[str]
    operand2: Optional[str]

def parse_opcodes(url: str):
    response = requests.get(url)
    response.raise_for_status()
    data = response.json()

    def extract_opcodes(section: dict) -> Dict[int, Opcode]:
        return {
            int(hex_str, 16): Opcode(
                mnemonic=entry.get("mnemonic", ""),
                length=entry.get("length", 1),
                operand1=entry.get("operand1"),
                operand2=entry.get("operand2")
            )
            for hex_str, entry in section.items()
        }
    unprefixed = extract_opcodes(data["unprefixed"])
    prefixed = extract_opcodes(data["cbprefixed"])

    return unprefixed, prefixed

# Example usage
unprefixed_list, prefixed_list = parse_opcodes(
    "https://raw.githubusercontent.com/lmmendes/game-boy-opcodes/master/opcodes.json"
)

# Optional: Print a sample
# print(unprefixed_list[0])
# print(prefixed_list[0])

#
def get_operand(operand):
    match operand:
        case "d16":
            return "{:#06X}"
        case "d8":
            return "{:#04X}"
        case "(a16)":
            return "({:#06X})"
        case "(a8)":
            return "(0xFF{:02X})"
        case "SP+r8":
            return "(SP + {:#04X})"
        case _:
            return operand

def get_argument(operand):
    match operand:
        case "d16":
            return ", ((bytes.get(2).copied().unwrap_or(0) as u16) << 8 | bytes.get(1).copied().unwrap_or(0) as u16)"
        case "d8":
            return ", bytes.get(1).copied().unwrap_or(0)"
        case "(a16)":
            return ", ((bytes.get(2).copied().unwrap_or(0) as u16) << 8 | bytes.get(1).copied().unwrap_or(0) as u16)"
        case "(a8)":
            return ", bytes.get(1).copied().unwrap_or(0)"
        case "SP+r8":
            return ", bytes.get(1).copied().unwrap_or(0)"
        case _:
            return ""


if os.path.exists("normal.txt"):
    os.remove("normal.txt")
with open("normal.txt", "a+") as f:
    for val, opcode in unprefixed_list.items():
        op1 = get_operand(opcode.operand1)
        op2 = get_operand(opcode.operand2)

        arg = get_argument(opcode.operand1)
        if arg == "":
            arg = get_argument(opcode.operand2)

        if op1 == None:
            f.write(f"0x{val:X} => format!(\"{opcode.mnemonic}\"),")
        elif op2 != None:
            f.write(f"0x{val:X} => format!(\"{opcode.mnemonic} {op1}, {op2}\"{arg}),")
        else:
            f.write(f"0x{val:X} => format!(\"{opcode.mnemonic} {op1}\"{arg}),")
        f.write("\n")
        arg = None;

if os.path.exists("prefixed.txt"):
    os.remove("prefixed.txt")
with open("prefixed.txt", "a+") as f:
    for val, opcode in prefixed_list.items():
        op1 = get_operand(opcode.operand1)
        op2 = get_operand(opcode.operand2)

        arg = get_argument(opcode.operand1)
        if arg == "":
            arg = get_argument(opcode.operand2)

        if op1 == None:
            f.write(f"0x{val:X} => format!(\"{opcode.mnemonic}\"),")
        elif op2 != None:
            f.write(f"0x{val:X} => format!(\"{opcode.mnemonic} {op1}, {op2}\"{arg}),")
        else:
            f.write(f"0x{val:X} => format!(\"{opcode.mnemonic} {op1}\"{arg}),")
        f.write("\n")
        arg = None;

if os.path.exists("lengths.txt"):
    os.remove("lengths.txt")
with open("lengths.txt", "a+") as f:
    for val, opcode in unprefixed_list.items():
        if val != 203:
            f.write(f"0x{val:X} => {opcode.length},\n")
        else:
            f.write(f"0xCB => 2,\n")

pprint.pprint(unprefixed_list)
