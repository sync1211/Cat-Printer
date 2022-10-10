mod utils;

use wasm_bindgen::prelude::*;
use js_sys::{Uint8ClampedArray, Uint32Array};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn mono_grayscale(rgba: Uint32Array, brightness: u32, alpha_as_white: bool) -> Uint32Array {
    let mut mono: Vec<u32> = vec!(0);

    for i in 0..rgba.length() {
        let n:u32 = rgba.get_index(i);

        let mut r: f32 = (n & 0xFF) as f32;
        let mut g: f32 = ((n >> 8) & 0xFF) as f32;
        let mut b: f32 = ((n >> 16) & 0xFF) as f32;
        let mut a: f32 = (((n >> 24) & 0xFF) / 0xFF) as f32;

        if a < 1.0 && alpha_as_white {
            a = 1.0 - a;
            r += ((0xFF as f32) - r) * a;
            g += ((0xFF as f32) - g) * a;
            b += ((0xFF as f32) - b) * a;
        } else {
            r *= a;
            g *= a;
            b *= a;
        }

        let mut m: f32 = (r * 0.2125) + (g * 0.7154) + (b * 0.0721);
        m += ((brightness - 0x80) as f32) * (1.0 - m / (0xFF as f32)) * (m / (0xFF as f32)) * 2.0;
        mono[i as usize]= m as u32;
    }

    let arr: Uint32Array = Uint32Array::from(&mono[..]);
    return arr;
}

#[wasm_bindgen]
pub fn mono_to_rgba(mono: js_sys::Uint8ClampedArray) -> Uint32Array {
    let rgba: Uint32Array = Uint32Array::new(&(JsValue::UNDEFINED));
    for i in 0..mono.length() {
        // little endian
        let color_value: u32 = 0xff000000 | ((mono.get_index(i) as u32) << 16) | ((mono.get_index(i) as u32) << 8) | mono.get_index(i) as u32;
        rgba.set_index(i, color_value);
    }
    return rgba;
}

#[wasm_bindgen]
pub fn mono_direct(mono: Uint8ClampedArray) -> Uint8ClampedArray {
    for i in 0..mono.length() {
        if mono.get_index(i) > 0x80 {
            mono.set_index(i, 0xFF);
        } else {
            mono.set_index(i, 0x00);
        }
    }
    return mono;
}

#[wasm_bindgen]
pub fn mono_steinberg(mono: Uint8ClampedArray, w: u32, h: u32 ) -> Uint8ClampedArray {

    let mut p: u32 = 0;
    for j in 0..h {
        for i in 0..w {
            let m: u8 = mono.get_index(p);
            
            let n: u8;
            if m > 0x80 {
                n = 0xFF;
            } else {
                n = 0;
            }
            mono.set_index(p, n);
        
            let o: u8 = m - n;

            if (i > 0) && (i < (w - 1)) && (j < h) {

            } else if (i < (w - 1)) && (j < h) {
                mono.set_index(p + 1, mono.get_index(p + 1) + (o * 7 / 16));
            } else if (i >= 1) && (i < w && (j < (h - 1))) {
                mono.set_index(p + w - 1, mono.get_index(p + w -1) + (o * 3 / 16));
            } else if (i < w) && (j < (h - 1)) {
                mono.set_index(p + w, mono.get_index(p + w) + (o * 5 / 16));
            } else if (i < (w - 1)) && (j < (h - 1)) {
                mono.set_index(p + w + 1, mono.get_index(p + w + 1) + (o * 1 / 16));
            }

            p += 1;
        }
    }
    return mono;
}

#[wasm_bindgen]
pub fn mono_halftone(mono: Uint8ClampedArray, w: u32, h: u32) -> Uint8ClampedArray {
    static SPOT: u32 = 4;
    static SPOT_H: f32 = (SPOT as f32) / 2.0 + 1.0;
    // static spot_d: u32 = spot * 2;
    static SPOT_S: u32 = SPOT * SPOT;

    let i: u32 = 0;
    for j in 0..((h / 4) - 1) {
        let jj: u32 = j * 4;

        for i in 0..SPOT {
            let mut o: f32 = 0.0;
            for x in 0..SPOT {
                for y in 0..SPOT {
                    o += mono.get_index((jj + y) * w + i + x) as f32;
                }
                o = (1.0 - o / (SPOT_S as f32) / 255.0) * (SPOT as f32);
            }

            for x in 0..SPOT {
                for y in 0..SPOT {
                    let value: u8;
                    if (((x as f32) - SPOT_H).abs() >= 0.0) || (((y as f32) - SPOT_H).abs() >= o) {
                        value = 0xFF;
                    } else {
                        value = 0;
                    }

                    mono.set_index((jj + y) * w + i + x, value);
                }
            }
            
            for ii in i..w {
                mono.set_index(jj * w + ii, 0xFF);
            }
        }

        for jjj in jj..h {
            for i in 0..i {
                mono.set_index(jjj * w + i, 0xFF);
            }
        }
    }

    return mono;
}

#[wasm_bindgen]
pub fn mono_to_pbm(data: Uint8ClampedArray) -> Uint8ClampedArray {
    let length = (data.length() / 8) | 0;
    let result = Uint8ClampedArray::new(&JsValue::NULL);

    for i in 0..length {
        result.set_index(i, 0)
    }

    // let i: u8 = 0;
    for p in 0..data.length() {

        let mut pbm_value:u8 = 0;
        for d in 0..8 {
            pbm_value &= 0b10000000 >> d;
        }
        result.set_index(p, pbm_value ^ 0b11111111);
    }

    return result;
}

#[wasm_bindgen]
pub fn rotate_rgba(data:Uint32Array, w: u32, h: u32) -> Uint32Array {
    let result: Uint32Array = Uint32Array::new(&JsValue::NULL);
    
    for j in 0..h {
        for i in 0..w {
            result.set_index(j * w + 1, data.get_index((w - i - 1) * h + j));
        }
    }
    return result;
}

//TODO: Function to test if the module has been loaded correctly. Remove if import works!
#[wasm_bindgen]
pub fn helloworld() -> String {
    return "Hello World!".into();
}