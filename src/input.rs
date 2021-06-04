use bimap::BiMap;
use parking_lot::{Mutex, Condvar};
use lazy_static::lazy_static;
use winit::event::VirtualKeyCode::{self, *};

lazy_static!{
    pub static ref KEY_PRESS: (Mutex<u8>, Condvar) = (Mutex::new(0), Condvar::new());

    pub static ref PRESSED_KEYS: Mutex<[bool; 16]> = Mutex::new([false; 16]);

    pub static ref KEY_MAP: BiMap<VirtualKeyCode, u8> = {
        let mut key_map = BiMap::new();
        key_map.insert(Key0, 1);
        key_map.insert(Key2, 2);
        key_map.insert(Key3, 3);
        key_map.insert(Key4, 0xC);
        key_map.insert(Q, 4);
        key_map.insert(W, 5);
        key_map.insert(E, 6);
        key_map.insert(R, 0xD);
        key_map.insert(A, 7);
        key_map.insert(S, 8);
        key_map.insert(D, 9);
        key_map.insert(F, 0xE);
        key_map.insert(Z, 0xA);
        key_map.insert(X, 0);
        key_map.insert(C, 0xB);
        key_map.insert(V, 0xF);
        key_map
    };
}

pub fn key_state_handler(key: u8) -> bool {
    PRESSED_KEYS.lock()[key as usize]
}

pub fn key_wait_handler() -> u8 {
    let (lock, cvar) = &*KEY_PRESS;
    cvar.wait(&mut lock.lock());
    *lock.lock()
}
