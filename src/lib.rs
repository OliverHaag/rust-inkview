#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate num;
#[macro_use]
extern crate num_derive;

use std::convert::{TryFrom, TryInto};
use std::sync::{Arc, Mutex};

mod c_types {
	pub type c_char = u8;
	pub type c_uchar = u8;
	pub type c_schar = i8;
	pub type c_short = i16;
	pub type c_ushort = u16;
	pub type c_int = i32;
	pub type c_uint = u32;
	pub type c_long = i32;
	pub type c_ulong = u32;
	pub type c_longlong = i64;
	pub type c_ulonglong = u64;
	pub type c_void = std::ffi::c_void;
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Color(i32);

impl Color {
	pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
		Self((blue as i32) << 16 + (green as i32) << 8 + red as i32)
	}

	pub const fn gs(intensity: u8) -> Self {
		Self(intensity as i32 * 0x010101)
	}

	pub const WHITE: Self = Self(WHITE);
	pub const LGRAY: Self = Self(LGRAY);
	pub const DGRAY: Self = Self(DGRAY);
	pub const BLACK: Self = Self(BLACK);
}

pub trait EventHandler {
	fn handle_event(&mut self, event: Event, par1: i32, par2: i32) -> i32;
}

static mut iv_event_handler: Option<Arc<Mutex<dyn EventHandler>>> = None;
extern fn iv_event_handler_wrapper(event: i32, par1: i32, par2: i32) -> i32 {
	unsafe {
		match iv_event_handler {
			Some(ref mut event_handler) => {
				if let Some(event) = num::FromPrimitive::from_i32(event) {
					event_handler.lock().expect("Event handler is locked").handle_event(event, par1, par2)
				}
				else {
					-1
				}
			},
			None => -2,
		}
	}
}

impl TryFrom<i32> for PanelType {
	type Error = &'static str;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		let value: u32 = value.try_into().unwrap();
		if value > (PanelType::ENABLED | PanelType::EVENT_NO_HANDLING | PanelType::NO_FB_OFFSET).0 {
			Err("Invalid panel type value")
		}
		else {
			Ok(PanelType(value))
		}
	}
}

pub fn main(event_handler: &mut Arc<Mutex<dyn EventHandler>>) {
	unsafe {
		iv_event_handler = Some(Arc::clone(event_handler));
		InkViewMain(Some(iv_event_handler_wrapper));
	}
}

pub fn exit() {
	unsafe {
		CloseApp();
	}
}

pub fn panel_type() -> PanelType {
	unsafe {
		{ GetPanelType().try_into().unwrap() }
	}
}

pub fn set_panel_type(panel_type: PanelType) {
	unsafe {
		SetPanelType(panel_type.0.try_into().unwrap());
	}
}

pub fn clear_screen() {
	unsafe {
		ClearScreen();
	}
}

pub fn screen_width() -> i32 {
	unsafe {
		ScreenWidth()
	}
}

pub fn screen_height() -> i32 {
	unsafe {
		ScreenHeight()
	}
}

pub fn full_update() {
	unsafe {
		FullUpdate();
	}
}

pub fn partial_update(x: i32, y: i32, w: i32, h: i32) {
	unsafe {
		PartialUpdate(x, y, w, h);
	}
}

pub fn draw_pixel(x: i32, y: i32, color: Color) {
	unsafe {
		DrawPixel(x, y, color.0);
	}
}

pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
	unsafe {
		DrawLine(x1, y1, x2, y2, color.0);
	}
}

pub fn draw_circle(x: i32, y: i32, radius: i32, color: Color) {
	unsafe {
		DrawCircle(x, y, radius, color.0)
	}
}

pub fn fill_area(x: i32, y: i32, w: i32, h: i32, color: Color) {
	unsafe {
		FillArea(x, y, w, h, color.0);
	}
}
