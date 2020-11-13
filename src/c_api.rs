#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::convert::{TryFrom, TryInto};

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

impl Event {
	#[inline]
	pub fn is_key(&self) -> bool {
		match self {
			Self::KEYPRESS |
			Self::KEYRELEASE |
			Self::KEYREPEAT => true,
			_ => false,
		}
	}

	#[inline]
	pub fn is_pointer(&self) -> bool {
		match self {
			Self::POINTERUP |
			Self::POINTERDOWN |
			Self::POINTERMOVE |
			Self::POINTERLONG |
			Self::POINTERHOLD |
			Self::MTSYNC |
			Self::POINTERDRAG |
			Self::POINTERCANCEL => true,
			_ => false,
		}
	}

	#[inline]
	pub fn is_panel(&self) -> bool {
		match self {
			Self::TAB |
			Self::PANEL |
			Self::PANEL_ICON |
			Self::PANEL_TEXT |
			Self::PANEL_PROGRESS |
			Self::PANEL_MPLAYER |
			Self::PANEL_USBDRIVE |
			Self::PANEL_NETWORK |
			Self::PANEL_CLOCK |
			Self::PANEL_BLUETOOTH |
			Self::PANEL_TASKLIST |
			Self::PANEL_OBREEY_SYNC |
			Self::PANEL_SETREADINGMODE |
			Self::PANEL_SETREADINGMODE_INVERT => true,
			_ => false,
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
