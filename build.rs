#[macro_use]
extern crate lazy_static;

use bindgen::callbacks::{ParseCallbacks, IntKind, EnumVariantValue};
use std::collections::BTreeMap;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};


struct Enum {
	name: String,
	kind: IntKind,
	variant: BTreeMap<i64, String>,
}

impl Enum {
	fn new(name: String, kind: IntKind) -> Self {
		Self {
			name: name,
			kind: kind,
			variant: BTreeMap::new(),
		}
	}
}

lazy_static! {
	static ref ENUMS: Arc<Mutex<BTreeMap<String, Enum>>> = Arc::new(Mutex::new(BTreeMap::new()));
}

fn main() -> std::io::Result<()> {
	// Tell cargo to tell rustc to link the inkview
	// shared library.
	println!("cargo:rustc-link-lib=inkview");

	#[derive(Debug)]
	struct InkViewTypeChooser;

	impl ParseCallbacks for InkViewTypeChooser {
		fn int_macro(&self, name: &str, value: i64) -> Option<IntKind> {
			let mutex = Arc::clone(&ENUMS);
			let mut enum_map = mutex.lock().unwrap();
			for (prefix, enum_kind) in &mut *enum_map {
				if name.starts_with(prefix.as_str()) {
					let mut variant_name = String::from(&name[prefix.len()..]);
					if let Some(first) = variant_name.chars().next() {
						if first.is_digit(10) {
							variant_name.insert_str(0, &enum_kind.name.to_uppercase());
						}
					}
					enum_kind.variant.insert(value, variant_name);
					return Some(enum_kind.kind);
				}
			}
			if value >= i32::min_value() as i64 &&
			   value <= i32::max_value() as i64 {
					Some(IntKind::I32)
			} else {
					None
			}
		}

		fn enum_variant_name(&self, enum_name: Option<&str>, original_variant_name: &str, _variant_value: EnumVariantValue) -> Option<String> {
			match enum_name {
				Some("PANEL_FLAGS") => Some(original_variant_name[6..].to_string()),
				_ => None
			}
		}

		fn item_name(&self, original_item_name: &str) -> Option<String> {
			match original_item_name {
				"PANEL_FLAGS" => Some(String::from("PanelType")),
				_ => None
			}
		}
	}

	{
		let mutex = Arc::clone(&ENUMS);
		let mut enum_map = mutex.lock().unwrap();
		enum_map.insert(String::from("EVT_"), Enum::new(String::from("Event"), IntKind::I32));
		enum_map.insert(String::from("IV_KEY_"), Enum::new(String::from("Key"), IntKind::I32));
		enum_map.insert(String::from("REQ_"), Enum::new(String::from("Request"), IntKind::I32));
		enum_map.insert(String::from("ICON_"), Enum::new(String::from("Icon"), IntKind::I32));
		enum_map.insert(String::from("DEF_"), Enum::new(String::from("Button"), IntKind::I32));
	}

	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
		// The input header we would like to generate
		// bindings for.
		.header_contents(
			"inkview.h",
			"#include <inkview.h>"
		)
		.whitelist_var("[A-Z]+DIR[0-9]?")
		.whitelist_var("[A-Z]+DATA[0-9]?")
		.whitelist_var("[A-Z]+PATH[0-9]?")
		.whitelist_var("[A-Z]+PROFILES?[0-9]?")
		.whitelist_var("[A-Z]+FILE")
		.whitelist_var("USER[A-Z]+")
		.whitelist_var("SYSTEM[A-Z]+")
		.whitelist_var("[A-Z][0-9A-Z_]*_APP(_PATH)?")
		.whitelist_var("STATECLEANER")
		.whitelist_var("[A-Z]+SCRIPT")
		.whitelist_var("NETAGENT[A-Z]*")
		.whitelist_var("[A-Z]+APP")
		.whitelist_var("POCKETBOOKSIG")
		.whitelist_var("LASTOPENBOOKS")
		.whitelist_var("CURRENTBOOK_SAVE")
		.whitelist_var("FAVORITES")
		.whitelist_var("CURRENTBOOK")
		.whitelist_var("BOOKSHELFSTATE")
		.whitelist_var("BOOKSHELFSTATE_NV")
		.whitelist_var("DICKEYBOARD")
		.whitelist_var("URLHISTORY")
		.whitelist_var("WEBCACHE[A-Z]*")
		.whitelist_var("WIDGETS[A-Z]+")
		.whitelist_var("SWUPDATESTATUS")
		.whitelist_var("[A-Z]+FOLDER")
		.whitelist_var("SOCIAL[A-Z_]+")
		.whitelist_var("[A-Z][0-9A-Z_]*_DIRECTORY")
		.whitelist_var("[A-Z][0-9A-Z_]*_FILE")
		.whitelist_var("[A-Z][0-9A-Z_]*_CFG")
		.whitelist_var("[A-Z][0-9A-Z_]*_PATH")
		.whitelist_var("BROWSER_FOR_AUTH")
		.whitelist_var("READER_[0-9A-Z_]+")
		.whitelist_var("OBREEY_[0-9A-Z_]+")
		.whitelist_var("PROFILE_[0-9A-Z_]+")
		.whitelist_var("SYSTEMDEPTH")
		.whitelist_var("MAXMSGSIZE")
		.whitelist_type("AvrcpCommands")
		.whitelist_function("IS[A-Z]+EVENT")
		//.whitelist_var("EVT_[0-9A-Z_]+")
		//.whitelist_var("IV_KEY_[0-9A-Z_]+")
		.whitelist_var("KEYMAPPING_KEY_[0-9A-Z_]+")
		.whitelist_var("BLACK")
		.whitelist_var("[DL]GRAY")
		.whitelist_var("WHITE")
		.whitelist_var("ITEM_[0-9A-Z_]+")
		.whitelist_var("KBD_[0-9A-Z_]+")
		//.whitelist_var("ICON_[0-9A-Z_]+")
		//.whitelist_var("DEF_BUTTON[0-9]")
		.whitelist_var("NO_DISMISS")
		.whitelist_var("WITH_SIZE")
		.whitelist_var("PANELICON_[0-9A-Z_]+")
		.whitelist_var("LIST(FLAG)?_[0-9A-Z_]+")
		.whitelist_var("BMK_[0-9A-Z_]+")
		.whitelist_var("CFG_[0-9A-Z_]+")
		.whitelist_var("[A-Z]+TASKS?")
		.whitelist_var("TASK_[0-9A-Z_]+")
		.whitelist_var("RQL_[0-9A-Z_]+")
		//.whitelist_var("REQ_[0-9A-Z_]+")
		.whitelist_var("ALIGN_[A-Z]+")
		.whitelist_var("VALIGN_[A-Z]+")
		.whitelist_var("ROTATE")
		.whitelist_var("HYPHENS")
		.whitelist_var("DOTS")
		.whitelist_var("RTLAUTO")
		.whitelist_var("UNDERLINE")
		.whitelist_var("STRETCH")
		.whitelist_var("TILE")
		.whitelist_var("TO_UPPER")
		.whitelist_var("FR_[A-Z]+")
		.whitelist_var("ARROW_[A-Z]+")
		.whitelist_var("SYMBOL_[A-Z]+")
		.whitelist_var("IMAGE_[A-Z]+")
		.whitelist_var("ROTATE[0-9]+")
		.whitelist_var("[XY]MIRROR")
		.whitelist_var("A2DITHER")
		.whitelist_var("DITHER_[A-Z]+")
		.whitelist_var("QN_[A-Z]+")
		.whitelist_type("PB_(TTS_)?STATE")
		.whitelist_var("MP_[A-Z]+")
		.whitelist_var("FTYPE_[A-Z]+")
		.whitelist_var("OB_[A-Z]+")
		.whitelist_var("NET_[0-9A-Z]+")
		.whitelist_var("CONN_[0-9A-Z]+")
		.whitelist_var("BLUETOOTH_[A-Z]+")
		.whitelist_type("WIFI_SECURITY")
		.whitelist_type("NET_STATE")
		.whitelist_var("VN_[A-Z]+")
		.whitelist_var("A2DP_[0-9A-Z_]+")
		.whitelist_var("CF_[0-9A-Z_]+")
		.whitelist_var("FONT_ACTIVATE_CODE")
		.whitelist_function("TOUCHDRAGDEADZONE")
		.whitelist_type("FONT_TYPE")
		.whitelist_type("SideFlags")
		.whitelist_type("PANEL_FLAGS")
		.whitelist_function("iv_[0-9a-z_]+")
		.whitelist_function("DEFAULTFONT[A-Z]*")
		.whitelist_type("irect")
		.whitelist_type("ibitmap")
		.whitelist_type("control_panel")
		.whitelist_type("TransparentHandle")
		.whitelist_type("ihash(_item)?")
		.whitelist_type("ifont[0-9a-z_]+")
		.whitelist_type("FONT_MENU_FLAGS")
		.whitelist_type("iuser_font")
		.whitelist_type("imenu[0-9a-z_]+")
		.whitelist_type("icanvas")
		.whitelist_type("icontext_menu[0-9a-z_]+")
		.whitelist_type("font_selector_properties")
		.whitelist_type("iapp_caption")
		.whitelist_type("itaskmgr")
		.whitelist_type("ipager")
		.whitelist_type("iselection")
		.whitelist_type("iappstyle")
		.whitelist_type("ievent")
		.whitelist_type("iconfig(edit)?")
		.whitelist_type("oldconfigedit")
		.whitelist_type("tocentry")
		.whitelist_type("itimer")
		.whitelist_type("bookinfo")
		.whitelist_type("iv_[0-9a-z_]+")
		.whitelist_type("(sub)?taskinfo")
		.whitelist_type("network_interface[a-z_]*")
		.whitelist_type("bt_[0-9a-z_]+")
		.whitelist_type("obex_[0-9a-z_]+")
		.whitelist_type("audio_output[a-z_]*")
		.whitelist_type("icolor_map")
		.whitelist_type("APPLICATION_ATTRIBUTE")
		.whitelist_function("(Open|Close)[A-Z][A-Za-z]*")
		.whitelist_function("InkViewMain")
		.whitelist_function("CloseApp")
		.whitelist_function("InitInkview")
		.whitelist_function("iRect")
		.whitelist_function("Screen(Width|Height)")
		.whitelist_function("[SG]et[A-Z][A-Za-z]*")
		.whitelist_function("[SG]et(Global|GSensor)?Orientation")
		.whitelist_var("GSENSOR_[A-Z]+")
		.whitelist_function("[A-Z][a-z]+GSensor(Enabled)?")
		.whitelist_type("estyle")
		.whitelist_function("Clear[A-Z][A-Za-z]*")
		.whitelist_function("ClearScreen")
		.whitelist_function("([SG]et|Merge)Clip(Rect)?")
		.whitelist_function("Draw[A-Z][A-Za-z]*")
		.whitelist_function("Fill[A-Z][A-Za-z]*")
		.whitelist_function("Invert[A-Z][A-Za-z]*")
		.whitelist_function("ColorMap[A-Z][A-Za-z]*")
		.whitelist_function("Dim[A-Z][A-Za-z]*")
		.whitelist_function("QuickFloyd16Dither")
		.whitelist_function("Stretch[A-Z][A-Za-z]*")
		.whitelist_function("[SG]etCanvas")
		.whitelist_function("Repaint")
		.whitelist_function("CheckFramePointer")
		.whitelist_function("(Get|Is)?Pager[A-Z][A-Za-z]*")
		.whitelist_function("Transparent(Rect)?")
		.whitelist_function("(Load|Save)[A-Z][A-Za-z]*")
		.whitelist_function("(zLoad|New|Copy|Move|Tile|Mirror)Bitmap([A-Z][A-Za-z]*)?")
		.whitelist_function("SetTransparentColor")
		.whitelist_function("EnumFonts([A-Z][A-Za-z]*)?")
		.whitelist_function("FreeFontsForSort")
		.whitelist_function("(Open|Close|[SG]et)Font")
		.whitelist_function("TextRectHeight(Ex)?")
		.whitelist_function("(MinimalTextRect|Char|String|GetMultilineString)Width(Ext)?")
		.whitelist_function("RegisterFontList")
		.whitelist_function("SetTextStrength")
		.whitelist_function("(Full|Soft|Partial|Dynamic|Exit|IsInA2|Fine|HQ|Schedule|WaitFor)Update([A-Z][A-Za-z0-9]*)?")
		.whitelist_function("[SG]etEventHandler(Ex)?")
		.whitelist_function("SendEvent(Ex)?")
		.whitelist_function("(Flush|Is)AnyEvents")
		.whitelist_function("GetCurrentEventExData")
		.whitelist_function("ProcessEventLoop(Quick)?")
		.whitelist_function("PrepareForLoop")
		.whitelist_function("ClearOnExit")
		.whitelist_function("(Set(Hard|Weak)|Query|Clear)Timer(Ex|ByName)?")
		.whitelist_function("(Open|Update)Menu(Ex|3x3)?")
		.whitelist_function("(Open|Set|Create|Close)ContextMenu")
		.whitelist_function("GetMenuRect(Ex)?")
		.whitelist_function("Open(Dummy)?List")
		.whitelist_function("[SG]etListHeaderLevel")
		.whitelist_function("EnumKeyboards")
		.blacklist_item("O_[A-Z]+")
		.blacklist_item("E(NOT|IS)DIR")
		.blacklist_item("E[NM]FILE")
		.blacklist_item("ENODATA")
		.ctypes_prefix("c_types")
		.default_enum_style(bindgen::EnumVariation::Rust{non_exhaustive: false})
		.bitfield_enum("PanelType")
		.generate_comments(true)
		.layout_tests(false)
		.parse_callbacks(Box::new(InkViewTypeChooser))
		.prepend_enum_name(false)
		.rustfmt_bindings(true)
		.use_core()
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file.
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	let file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(out_path.join("bindings.rs"))?;
	let mut file_copy = file.try_clone()?;
	bindings.write(Box::new(file))?;
	{
		let mutex = Arc::clone(&ENUMS);
		let enum_map = mutex.lock().unwrap();
		for (_prefix, enum_kind) in &*enum_map {
			match enum_kind.kind {
				IntKind::I32 => {
					writeln!(file_copy, "#[repr(i32)]")?;
				},
				IntKind::U32 => {
					writeln!(file_copy, "#[repr(u32)]")?;
				},
				_ => {},
			}
			writeln!(file_copy, "#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]")?;
			writeln!(file_copy, "pub enum {} {{", enum_kind.name)?;
			for (variant_value, variant_name) in &enum_kind.variant {
				writeln!(file_copy, "    {} = {},", variant_name, variant_value)?;
			}
			writeln!(file_copy, "}}")?;
		}
	}
	Ok(())
}
