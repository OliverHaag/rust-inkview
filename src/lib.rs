#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate num;
#[macro_use]
extern crate num_derive;

use std::convert::TryInto;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

pub mod c_api;

/// Color for use with inkview drawing routines.
/// Internally stored in i32 as 0x00RRGGBB.
pub struct Color(i32);

impl Color {
	/// Generate color from RGB channels
	#[inline]
	pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
		Self((red as i32) << 16 + (green as i32) << 8 + blue as i32)
	}

	/// Generate color from a single greyscale channel
	#[inline]
	pub const fn gs(intensity: u8) -> Self {
		Self(intensity as i32 * 0x010101)
	}

	pub const WHITE: Self = Self(c_api::WHITE);
	pub const LGRAY: Self = Self(c_api::LGRAY);
	pub const DGRAY: Self = Self(c_api::DGRAY);
	pub const BLACK: Self = Self(c_api::BLACK);
}

////////////////////////////////////////////////////////////////////////////////
// Application control

pub type Event = c_api::Event;              /// Application events
pub type Key = c_api::Key;                  /// Key events
//pub type Request = c_api::Request;

/// Event handler trait for InkView application events
pub trait EventHandler {
	/// Called for new events, par1 and par2 meanings depend on event.
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

/// Registers event_handler as application event handler and runs InkViewMain.
pub fn main(event_handler: &mut Arc<Mutex<dyn EventHandler>>) {
	unsafe {
		iv_event_handler = Some(Arc::clone(event_handler));
		c_api::InkViewMain(Some(iv_event_handler_wrapper));
	}
}

/// Put Event::EXIT into applications event queue and closes the application.
pub fn exit() {
	unsafe {
		c_api::CloseApp();
	}
}

/// Put Event::SHOW into applications event queue.
pub fn repaint() {
	unsafe {
		c_api::CloseApp();
	}
}

////////////////////////////////////////////////////////////////////////////////
// Graphic functions

pub type Dither = c_api::Dither;

pub fn screen_width() -> i32 {
	unsafe {
		c_api::ScreenWidth()
	}
}

pub fn screen_height() -> i32 {
	unsafe {
		c_api::ScreenHeight()
	}
}

pub fn clear_screen() {
	unsafe {
		c_api::ClearScreen();
	}
}

pub fn set_clip(x: i32, y: i32, w: i32, h: i32) {
	unsafe {
		c_api::SetClip(x, y, w, h);
	}
}

pub fn draw_pixel(x: i32, y: i32, color: Color) {
	unsafe {
		c_api::DrawPixel(x, y, color.0);
	}
}

pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
	unsafe {
		c_api::DrawLine(x1, y1, x2, y2, color.0);
	}
}

pub fn draw_dot_line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color, step: i32) {
	unsafe {
		c_api::DrawLineEx(x1, y1, x2, y2, color.0, step);
	}
}

pub fn draw_dash_line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color, fill: u32, space: u32) {
	unsafe {
		c_api::DrawDashLine(x1, y1, x2, y2, color.0, fill, space);
	}
}

pub fn draw_rect(x: i32, y: i32, w: i32, h: i32, color: Color) {
	unsafe {
		c_api::DrawRect(x, y, w, h, color.0);
	}
}

pub fn draw_rect_round(x: i32, y: i32, w: i32, h: i32, color: Color, radius: i32) {
	unsafe {
		c_api::DrawRectRound(x, y, w, h, color.0, radius);
	}
}

pub fn fill_area(x: i32, y: i32, w: i32, h: i32, color: Color) {
	unsafe {
		c_api::FillArea(x, y, w, h, color.0);
	}
}

pub fn invert_area(x: i32, y: i32, w: i32, h: i32) {
	unsafe {
		c_api::InvertArea(x, y, w, h);
	}
}

pub fn invert_area_bw(x: i32, y: i32, w: i32, h: i32) {
	unsafe {
		c_api::InvertAreaBW(x, y, w, h);
	}
}

pub fn dim_area(x: i32, y: i32, w: i32, h: i32, color: Color) {
	unsafe {
		c_api::DimArea(x, y, w, h, color.0);
	}
}

pub fn draw_selection(x: i32, y: i32, w: i32, h: i32, color: Color) {
	unsafe {
		c_api::DrawSelection(x, y, w, h, color.0);
	}
}

pub fn draw_circle(x: i32, y: i32, radius: i32, color: Color) {
	unsafe {
		c_api::DrawCircle(x, y, radius, color.0);
	}
}

pub fn draw_pick_out(x: i32, y: i32, w: i32, h: i32, key: &str) {
	let c_key = CString::new(key).expect("CString::new failed").into_raw();
	unsafe {
		c_api::DrawPickOut(x, y, w, h, c_key);
	}
}

pub fn dither_area(x: i32, y: i32, w: i32, h: i32, levels: i32, method: Dither) {
	unsafe {
		c_api::DitherArea(x, y, w, h, levels, method as i32);
	}
}

pub fn dither_area_quick_2level(x: i32, y: i32, w: i32, h: i32) {
	unsafe {
		c_api::DitherAreaQuick2Level(x, y, w, h);
	}
}

pub fn dither_area_pattern_2level(x: i32, y: i32, w: i32, h: i32) {
	unsafe {
		c_api::DitherAreaPattern2Level(x, y, w, h);
	}
}

pub fn draw_diagonal_hatch(x: i32, y: i32, w: i32, h: i32, step: i32, color: Color) {
	unsafe {
		c_api::DrawDiagonalHatch(x, y, w, h, step, color.0);
	}
}

pub fn transparent(x: i32, y: i32, w: i32, h: i32, percent: i32) {
	unsafe {
		c_api::Transparent(x, y, w, h, percent);
	}
}

////////////////////////////////////////////////////////////////////////////////
// Screen update

pub fn full_update() {
	unsafe {
		c_api::FullUpdate();
	}
}

pub fn soft_update() {
	unsafe {
		c_api::FullUpdate();
	}
}

pub fn partial_update(x: i32, y: i32, w: i32, h: i32) {
	unsafe {
		c_api::PartialUpdate(x, y, w, h);
	}
}

////////////////////////////////////////////////////////////////////////////////
// UI functions

//pub type Icon = c_api::Icon;                /// Dialog icons
//pub type Button = c_api::Button;            /// Dialog buttons
pub type PanelType = c_api::PanelType;      /// InkView header panel control flags

pub fn panel_type() -> PanelType {
	unsafe {
		{ c_api::GetPanelType().try_into().unwrap() }
	}
}

pub fn set_panel_type(panel_type: PanelType) {
	unsafe {
		c_api::SetPanelType(panel_type.0.try_into().unwrap());
	}
}
