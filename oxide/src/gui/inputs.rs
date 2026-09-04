use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use sdl3::keyboard::Keycode;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GBKey {
    DpadL,
    DpadR,
    DpadU,
    DpadD,
    BtnA,
    BtnB,
    Start,
    Select,
    NONE,
}

impl GBKey {
    pub fn from_key(key: Keycode) -> Self {
        let res = crate::settings::GLOB_SETTINGS.get().unwrap().gb_keys.iter().find_map(
            |(gbk, k)| (*k == key).then_some(*gbk));
        if !res.is_none() {
            res.unwrap()
        } else {
            GBKey::NONE
        }
    }

    pub fn to_key(&self) -> Option<Keycode> {
        crate::settings::GLOB_SETTINGS.get().unwrap().gb_keys.iter().find_map(
            |(gbk, k)| (gbk == self).then_some(*k))
    }

    pub fn get_bit(&self) -> u8 {
        match self {
            GBKey::DpadL => 5,
            GBKey::DpadR => 4,
            GBKey::DpadU => 6,
            GBKey::DpadD => 7,
            GBKey::BtnA => 0,
            GBKey::BtnB => 1,
            GBKey::Start => 3,
            GBKey::Select => 2,
            GBKey::NONE => 0,
        }
    }
}

pub fn parse_key_event(keycode: Keycode, down: bool, joystate: &Arc<AtomicU8>) {
    let gb_key = GBKey::from_key(keycode);

    if gb_key != GBKey::NONE {
        let mask = 1u8 << gb_key.get_bit();
        let current = joystate.load(Ordering::Relaxed);
        if down {
            joystate.store(current | mask, Ordering::Relaxed);
        } else {
            joystate.store(current & !mask, Ordering::Relaxed);
        }
    }
}

