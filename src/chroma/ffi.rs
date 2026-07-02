#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::c_int;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Xyz,
    Yxy,
    Srgb,
    LinearSrgb,
    DisplayP3,
    LinearDisplayP3,
    Rec2020,
    Rec2020Scene,
    LinearRec2020,
    Hsl,
    Hsv,
    Hsi,
    Hwb,
    Cmyk,
    Lab,
    Lch,
    Oklab,
    Oklch,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Lch {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Cmyk {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Xyz {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Yxy {
    pub luma: f32,
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hwb {
    pub h: f32,
    pub w: f32,
    pub b: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hsi {
    pub h: f32,
    pub s: f32,
    pub i: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ColorData {
    pub xyz: Xyz,
    pub yxy: Yxy,
    pub srgb: Rgb,
    pub linear_srgb: Rgb,
    pub display_p3: Rgb,
    pub linear_display_p3: Rgb,
    pub rec2020: Rgb,
    pub rec2020scene: Rgb,
    pub linear_rec2020: Rgb,
    pub hsl: Hsl,
    pub hsv: Hsv,
    pub hsi: Hsi,
    pub hwb: Hwb,
    pub cmyk: Cmyk,
    pub lab: Lab,
    pub lch: Lch,
    pub oklab: Lab,
    pub oklch: Lch,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub space: Space,
    pub data: ColorData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AlphaColor {
    pub color: Color,
    pub alpha: f32,
}

unsafe extern "C" {
    // Conversion
    pub fn chroma_convert(src: Color, dst: Space) -> Color;

    // Gamut
    pub fn chroma_is_in_gamut(src: Color, gamut: Space) -> bool;
    pub fn chroma_gamut_map(src: Color, target: Space) -> Color;

    // Generic init/unpack
    pub fn chroma_init(space: Space, vals: *const f32) -> Color;
    pub fn chroma_unpack(clr: Color, vals: *mut f32) -> c_int;
    pub fn chroma_init_alpha(space: Space, vals: *const f32, alpha: f32) -> AlphaColor;
    pub fn chroma_unpack_alpha(aclr: AlphaColor, vals: *mut f32, alpha: *mut f32) -> c_int;

    // Hex helpers
    pub fn chroma_init_hex(hex: u32) -> Color;
    pub fn chroma_unpack_hex(clr: Color) -> u32;
    pub fn chroma_init_hexa(rgba: u32) -> AlphaColor;
    pub fn chroma_unpack_hexa(aclr: AlphaColor) -> u32;

    // 8-bit sRGB helpers
    pub fn chroma_init_srgb8(r: u8, g: u8, b: u8) -> Color;
    pub fn chroma_unpack_srgb8(clr: Color, r: *mut u8, g: *mut u8, b: *mut u8);
    pub fn chroma_init_srgba8(r: u8, g: u8, b: u8, a: u8) -> AlphaColor;
    pub fn chroma_unpack_srgba8(aclr: AlphaColor, r: *mut u8, g: *mut u8, b: *mut u8, a: *mut u8);

    // Formatting
    pub fn chroma_format(color: Color, buf: *mut u8, buf_size: c_int) -> c_int;

    // Meta utilities
    pub fn chroma_space_from_name(name: *const u8) -> c_int;
    pub fn chroma_space_count() -> c_int;
    pub fn chroma_space_name(index: c_int) -> *const u8;
    pub fn chroma_field_count(space: Space) -> c_int;
}
