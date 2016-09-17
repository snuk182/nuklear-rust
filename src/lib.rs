#![feature(alloc, heap_api)]

#[macro_use]
extern crate log;
extern crate nuklear_sys;
extern crate alloc;

use std::default::Default;
use std::os::raw::*;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

use nuklear_sys::*;

pub use nuklear_sys::nk_font_atlas_format as NkFontAtlasFormat;
pub use nuklear_sys::nk_flags as NkFlags; //TODO
pub use nuklear_sys::nk_collapse_states as NkCollapseState;
pub use nuklear_sys::nk_show_states as NkShowState;
pub use nuklear_sys::nk_layout_format as NkLayoutFormat;
pub use nuklear_sys::nk_tree_type as NkTreeType;
pub use nuklear_sys::nk_symbol_type as NkSymbolType;
pub use nuklear_sys::nk_button_behavior as NkButtonBehavior;
pub use nuklear_sys::nk_color_format as NkColorFormat;
pub use nuklear_sys::nk_chart_type as NkChartType;
pub use nuklear_sys::nk_popup_type as NkPopupType;
pub use nuklear_sys::nk_keys as NkKey;
pub use nuklear_sys::nk_buttons as NkButton;
pub use nuklear_sys::nk_rune as NkRune;
pub use nuklear_sys::nk_style_colors as NkStyleColor;
pub use nuklear_sys::nk_style_cursor as NkStyleCursor;
pub use nuklear_sys::nk_widget_layout_states as NkWidgetLayoutState;
pub use nuklear_sys::nk_draw_list_stroke as NkDrawListStroke;
pub use nuklear_sys::nk_anti_aliasing as NkAntiAliasing;
pub use nuklear_sys::nk_allocation_type as NkAllocationType;
pub use nuklear_sys::nk_draw_vertex_layout_attribute as NkDrawVertexLayoutAttribute;
pub use nuklear_sys::nk_draw_vertex_layout_format as NkDrawVertexLayoutFormat;

pub use nuklear_sys::nk_panel_flags as NkPanelFlags;
pub use nuklear_sys::nk_text_alignment as NkTextAlignment;

pub use nuklear_sys::nk_vec2 as NkVec2;
pub use nuklear_sys::nk_vec2i as NkVec2i;
pub use nuklear_sys::nk_scroll as NkScroll;
pub use nuklear_sys::nk_color as NkColor;
pub use nuklear_sys::nk_rect as NkRect;
pub use nuklear_sys::nk_recti as NkRecti;

pub use nuklear_sys::nk_glyph as NkGlyph;

pub use nuklear_sys::nk_plugin_filter as NkPluginFilter;

pub const NK_FILTER_DEFAULT: NkPluginFilter = Some(nk_filter_default);
pub const NK_FILTER_ASCII: NkPluginFilter = Some(nk_filter_ascii);
pub const NK_FILTER_FLOAT: NkPluginFilter = Some(nk_filter_float);
pub const NK_FILTER_DECIMAL: NkPluginFilter = Some(nk_filter_decimal);
pub const NK_FILTER_HEX: NkPluginFilter = Some(nk_filter_hex);
pub const NK_FILTER_OCT: NkPluginFilter = Some(nk_filter_oct);
pub const NK_FILTER_BINARY: NkPluginFilter = Some(nk_filter_binary);

/*impl NkPluginFilter {
	fn to_native(&mut self) -> nk_plugin_filter {
		Some(|editor, rune|  )
	}
}*/

/*fn nk_filter_my(arg1: *const nk_text_edit, unicode: nk_rune) -> ::std::os::raw::c_int {
	
}

pub const NK_FILTER_MY: NkPluginFilter = Some(nk_filter_my);*/

pub type NkPlotValueGetter = ::std::option::Option<unsafe extern "C" fn(user: *mut ::std::os::raw::c_void, index: ::std::os::raw::c_int) -> f32>;

#[derive(Clone)]
pub struct NkString<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> NkString<'a> {
    pub unsafe fn from_bytes_unchecked(bytes: &'a [u8]) -> NkString<'a> {
        NkString { bytes: Cow::Borrowed(bytes) }
    }
    pub fn as_ptr(&self) -> *const c_char { self.bytes.as_ptr() as *const c_char }
    
    /*
    pub fn nk_str_init(arg1: *mut nk_str, arg2: *const nk_allocator,
                       size: nk_size);
    pub fn nk_str_init_fixed(arg1: *mut nk_str,
                             memory: *mut ::std::os::raw::c_void,
                             size: nk_size);
    pub fn nk_str_clear(arg1: *mut nk_str);
    pub fn nk_str_free(arg1: *mut nk_str);
    pub fn nk_str_append_text_char(arg1: *mut nk_str,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nk_str_append_str_char(arg1: *mut nk_str,
                                  arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn nk_str_append_text_utf8(arg1: *mut nk_str,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nk_str_append_str_utf8(arg1: *mut nk_str,
                                  arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn nk_str_append_text_runes(arg1: *mut nk_str, arg2: *const nk_rune,
                                    arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nk_str_append_str_runes(arg1: *mut nk_str, arg2: *const nk_rune)
     -> ::std::os::raw::c_int;
    pub fn nk_str_insert_at_char(arg1: *mut nk_str,
                                 pos: ::std::os::raw::c_int,
                                 arg2: *const ::std::os::raw::c_char,
                                 arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nk_str_insert_at_rune(arg1: *mut nk_str,
                                 pos: ::std::os::raw::c_int,
                                 arg2: *const ::std::os::raw::c_char,
                                 arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nk_str_insert_text_char(arg1: *mut nk_str,
                                   pos: ::std::os::raw::c_int,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nk_str_insert_str_char(arg1: *mut nk_str,
                                  pos: ::std::os::raw::c_int,
                                  arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn nk_str_insert_text_utf8(arg1: *mut nk_str,
                                   pos: ::std::os::raw::c_int,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nk_str_insert_str_utf8(arg1: *mut nk_str,
                                  pos: ::std::os::raw::c_int,
                                  arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn nk_str_insert_text_runes(arg1: *mut nk_str,
                                    pos: ::std::os::raw::c_int,
                                    arg2: *const nk_rune,
                                    arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn nk_str_insert_str_runes(arg1: *mut nk_str,
                                   pos: ::std::os::raw::c_int,
                                   arg2: *const nk_rune)
     -> ::std::os::raw::c_int;
    pub fn nk_str_remove_chars(arg1: *mut nk_str, len: ::std::os::raw::c_int);
    pub fn nk_str_remove_runes(str: *mut nk_str, len: ::std::os::raw::c_int);
    pub fn nk_str_delete_chars(arg1: *mut nk_str, pos: ::std::os::raw::c_int,
                               len: ::std::os::raw::c_int);
    pub fn nk_str_delete_runes(arg1: *mut nk_str, pos: ::std::os::raw::c_int,
                               len: ::std::os::raw::c_int);
    pub fn nk_str_at_char(arg1: *mut nk_str, pos: ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
    pub fn nk_str_at_rune(arg1: *mut nk_str, pos: ::std::os::raw::c_int,
                          unicode: *mut nk_rune,
                          len: *mut ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
    pub fn nk_str_rune_at(arg1: *const nk_str, pos: ::std::os::raw::c_int)
     -> nk_rune;
    pub fn nk_str_at_char_const(arg1: *const nk_str,
                                pos: ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
    pub fn nk_str_at_const(arg1: *const nk_str, pos: ::std::os::raw::c_int,
                           unicode: *mut nk_rune,
                           len: *mut ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
    pub fn nk_str_get(arg1: *mut nk_str) -> *mut ::std::os::raw::c_char;
    pub fn nk_str_get_const(arg1: *const nk_str)
     -> *const ::std::os::raw::c_char;
    pub fn nk_str_len(arg1: *mut nk_str) -> ::std::os::raw::c_int;
    pub fn nk_str_len_char(arg1: *mut nk_str) -> ::std::os::raw::c_int;
    */
}

impl<'a> From<&'a str> for NkString<'a> {
    fn from(value: &'a str) -> NkString<'a> {
        let mut bytes: Vec<u8> = value.bytes().collect();
        bytes.push(0);
        NkString { bytes: Cow::Owned(bytes) }
    }
}

impl From<String> for NkString<'static> {
    fn from(mut value: String) -> NkString<'static> {
        value.push('\0');
        NkString { bytes: Cow::Owned(value.into_bytes()) }
    }
}

#[macro_export]
macro_rules! nk_string {
    ($e:tt) => ({
        let value = concat!($e, "\0");
        unsafe { ::nuklear_rust::NkString::from_bytes_unchecked(value.as_bytes()) }
    });
    ($e:tt, $($arg:tt)*) => ({
        ::nuklear_rust::NkString::from(format!($e, $($arg)*))
    })
}

//======================================================================================

#[derive(Clone)]
pub struct NkStringArray<'a> {
    arr: Vec<NkString<'a>>,
    ptrs: Vec<*const c_char>
}

impl<'a> NkStringArray<'a> {
    pub fn as_ptr(&self) -> *const *const c_char { self.ptrs.as_slice() as *const _ as *const *const c_char }
    pub fn as_mut(&mut self) -> *mut *const c_char { self.ptrs.as_mut_slice() as *mut _ as *mut *const c_char }
    pub fn len(&self) -> usize { self.ptrs.len() }
}

impl<'a> From<&'a [&'a str]> for NkStringArray<'a> {
    fn from(value: &[&'a str]) -> NkStringArray<'a> {
        let mut r = NkStringArray {
        	arr: Vec::with_capacity(value.len()),
        	ptrs: Vec::with_capacity(value.len()),
        };
        
        for s in value {
        	r.arr.push(NkString::from(*s));
        	r.ptrs.push(r.arr[r.arr.len()-1].as_ptr());
        }
        
        r
    }
}

//======================================================================================

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum NkHandleKind {
	Empty, Ptr, Id, Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct NkHandle {
	kind: NkHandleKind,
	internal: nk_handle
}

impl Default for NkHandle {
	fn default() -> Self {
		NkHandle {
			kind: NkHandleKind::Empty,
			internal: nk_handle::default(),
		}
	}
}

impl NkHandle {
	pub fn kind(&self) -> NkHandleKind {
		self.kind
	}
	
	pub fn id(&mut self) -> Option<i32> {
		match self.kind {
			NkHandleKind::Id | NkHandleKind::Unknown => {
				Some(unsafe { *(self.internal.id()) } )
			},
			_ => {
				None
			},
		}
	}
	
	pub fn ptr(&mut self) -> Option<*mut c_void> {
		match self.kind {
			NkHandleKind::Ptr | NkHandleKind::Unknown => {
				Some(unsafe { *(self.internal.ptr()) } )
			},
			_ => {
				None
			},
		}
	}
	
	/*pub fn set_id(&mut self, value: i32) {
		self.kind = NkHandleKind::Id;
		unsafe {
			*(self.internal.id()) = value;
		}
	}
	
	pub fn set_ptr(&mut self, value: *mut c_void) {
		self.kind = NkHandleKind::Ptr;
		unsafe {
			*(self.internal.ptr()) = value;
		}
	}*/
	
	pub fn from_id(value: i32) -> NkHandle {
		NkHandle {
			kind: NkHandleKind::Id,
			internal: unsafe { nk_handle_id(value) },
		}
	}
	
	pub fn from_ptr(value: *mut c_void) -> NkHandle {
		NkHandle {
			kind: NkHandleKind::Ptr,
			internal: unsafe { nk_handle_ptr(value) },
		}
	}
}

//==================================================================================

pub struct NkInput {
	internal: *const nk_input,
}

impl NkInput {
	fn new(i: *const nk_input) -> NkInput {
		NkInput {
			internal: i
		}
	}
	
	pub fn mouse(&self) -> NkMouse {
		NkMouse {
			internal: unsafe { (*self.internal).mouse },
		}
	}
	
	pub fn has_mouse_click(&self, b: NkButton) -> bool {
		unsafe {
			nk_input_has_mouse_click(self.internal, b) != 0
		}
	}
	
	pub fn has_mouse_click_in_rect(&self, b: NkButton, rect: NkRect) -> bool {
		unsafe {
			nk_input_has_mouse_click_in_rect(self.internal, b, rect) != 0
		}
	}
	
	pub fn has_mouse_click_down_in_rect(&self, b: NkButton, rect: NkRect, down: bool) -> bool {
		unsafe {
			nk_input_has_mouse_click_down_in_rect(self.internal, b, rect, if down { 1 } else { 0 }) != 0
		}
	}
	
	pub fn is_mouse_click_in_rect(&self, b: NkButton, rect: NkRect) -> bool {
		unsafe {
			nk_input_is_mouse_click_in_rect(self.internal, b, rect) != 0
		}
	}
    
	pub fn is_mouse_click_down_in_rect(&self, b: NkButton, rect: NkRect, down: bool) -> bool {
		unsafe {
			nk_input_is_mouse_click_down_in_rect(self.internal, b, rect, down as ::std::os::raw::c_int) != 0
		}
	}
	
	pub fn any_mouse_click_in_rect(&self, rect: NkRect) -> bool {
		unsafe {
			nk_input_any_mouse_click_in_rect(self.internal, rect) != 0
		}
	}
	
	pub fn is_mouse_prev_hovering_rect(&self, rect: NkRect) -> bool {
		unsafe {
			nk_input_is_mouse_prev_hovering_rect(self.internal, rect) != 0
		}
	}
	
	pub fn is_mouse_hovering_rect(&self, rect: NkRect) -> bool {
		unsafe {
			nk_input_is_mouse_hovering_rect(self.internal, rect) != 0
		}
	}
	
	pub fn is_mouse_clicked(&self, b: NkButton, rect: NkRect) -> bool {
		unsafe {
			nk_input_mouse_clicked(self.internal, b, rect) != 0
		}
	}
	
	pub fn is_mouse_down(&self, b: NkButton) -> bool {
		unsafe {
			nk_input_is_mouse_down(self.internal, b) != 0
		}
	}
	
	pub fn is_mouse_pressed(&self, b: NkButton) -> bool {
		unsafe {
			nk_input_is_mouse_pressed(self.internal, b) != 0
		}
	}
	
	pub fn nk_input_is_mouse_released(&self, b: NkButton) -> bool {
		unsafe {
			nk_input_is_mouse_released(self.internal, b) != 0
		}
	}
	
	pub fn is_key_pressed(&self, k: NkKey) -> bool {
		unsafe {
			nk_input_is_key_pressed(self.internal, k) != 0
		}
	}
	
	pub fn is_key_released(&self, k: NkKey) -> bool {
		unsafe {
			nk_input_is_key_released(self.internal, k) != 0
		}
	}
	
	pub fn is_key_down(&self, k: NkKey) -> bool {
		unsafe {
			nk_input_is_key_down(self.internal, k) != 0
		}
	}	
}

//=====================================================================

#[derive(Clone)]
pub struct NkDrawCommand {
	internal: *const nk_draw_command,
}

impl Debug for NkDrawCommand {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        unsafe { (*self.internal).fmt(f) }
    }
}

impl NkDrawCommand {
	pub fn clip_rect(&self) -> NkRect {
		unsafe {
			(*self.internal).clip_rect
		}
	}
	
	pub fn elem_count(&self) -> u32 {
		unsafe {
			(*self.internal).elem_count
		}
	}
	
	pub fn texture(&self) -> NkHandle {
		NkHandle {
			kind: NkHandleKind::Unknown,
			internal: unsafe {
				(*self.internal).texture
			}
		}
	}
}

//=====================================================================

pub struct NkMouse {
	internal: nk_mouse,
}

impl NkMouse {
	pub fn pos(&self) -> NkVec2 {
		self.internal.pos
	}
}

//=====================================================================

pub struct NkStyle {
	internal: *mut nk_style,
}

impl NkStyle {
	pub fn window(&mut self) -> NkStyleWindow {
		NkStyleWindow {
			internal: &mut unsafe { (*self.internal).window }
		}
	}
}

//=====================================================================

pub struct NkStyleWindow {
	internal: *mut nk_style_window,
}

impl NkStyleWindow {
	pub fn border_color(&self) -> NkColor {
		unsafe { (*self.internal).border_color }
	}
	
	pub fn fixed_background(&self) -> NkStyleItem {
		NkStyleItem {
			internal: unsafe { (*self.internal).fixed_background },
		}
	}
	
	pub fn spacing(&self) -> NkVec2 {
		unsafe { (*self.internal).spacing }
	}
	
	pub fn padding(&self) -> NkVec2 {
		unsafe { (*self.internal).padding }
	}
	
	pub fn set_border_color(&mut self, color: NkColor) {
		unsafe { (*self.internal).border_color = color; }
	}
	
	pub fn set_fixed_background(&mut self, item: NkStyleItem) {
		unsafe { (*self.internal).fixed_background = item.internal; }
	}
	
	pub fn set_spacing(&mut self, spacing: NkVec2) {
		unsafe { (*self.internal).spacing = spacing; }
	}
	
	pub fn set_padding(&mut self, padding: NkVec2) {
		unsafe { (*self.internal).padding = padding; }
	}
}

//=====================================================================

pub struct NkDrawList {
	internal: *mut nk_draw_list,
}

impl NkDrawList {
	fn new(i: *mut nk_draw_list) -> NkDrawList {
		NkDrawList {
			internal: i
		}
	}
	
	pub fn init(&mut self) {
		unsafe {
			nk_draw_list_init(self.internal);
		}
	}
	
	pub fn setup(&mut self, config: &NkConvertConfig, cmds: &mut NkBuffer, vertices: &mut NkBuffer, elements: &mut NkBuffer) {
		unsafe {
			nk_draw_list_setup(self.internal, &config.internal as *const nk_convert_config, &mut cmds.internal as *mut nk_buffer, &mut vertices.internal as *mut nk_buffer, &mut elements.internal as *mut nk_buffer)
		}
	}
	
	pub fn clear(&mut self) {
		unsafe {
			nk_draw_list_clear(self.internal);
		}
	}
	
	pub fn begin(&self, buf: &NkBuffer) -> NkDrawCommand {
		unsafe {
			NkDrawCommand {
				internal: nk__draw_list_begin(self.internal as *const nk_draw_list, &buf.internal as *const nk_buffer)
			}
		}
	}
	
	pub fn next(&self, buf: &NkBuffer, prev: &NkDrawCommand) -> NkDrawCommand {
		unsafe {
			NkDrawCommand {
				internal: nk__draw_list_next(prev.internal, &buf.internal as *const nk_buffer, self.internal as *const nk_draw_list)
			}
		}
	}
	
	pub fn path_clear(&mut self) {
		unsafe {
			nk_draw_list_path_clear(self.internal);
		}
	}
	
	pub fn path_line_to(&mut self, pos: NkVec2) {
		unsafe {
			nk_draw_list_path_line_to(self.internal, pos);
		}
	}
	
	pub fn path_arc_to_fast(&mut self, center: NkVec2, radius: f32, a_min: i32, a_max: i32) {
		unsafe {
			nk_draw_list_path_arc_to_fast(self.internal, center, radius, a_min, a_max);
		}
	}
	
	pub fn path_arc_to(&mut self, center: NkVec2, radius: f32, a_min: f32, a_max: f32, segments: u32) {
		unsafe {
			nk_draw_list_path_arc_to(self.internal, center, radius, a_min, a_max, segments);    
		}
	}
	
	pub fn path_rect_to(&mut self, a: NkVec2, b: NkVec2, rounding: f32) {
		unsafe {
			nk_draw_list_path_rect_to(self.internal, a, b, rounding);
		}
	}
	
	pub fn path_curve_to(&mut self, p2: NkVec2, p3: NkVec2, p4: NkVec2, num_segments: u32) {
		unsafe {
			nk_draw_list_path_curve_to(self.internal, p2, p3, p4, num_segments)
		}
	}
	
	pub fn path_fill(&mut self, col: NkColor) {
		unsafe {
			nk_draw_list_path_fill(self.internal, col);
		}
	}
	
	pub fn path_stroke(&mut self, arg2: NkColor, closed: NkDrawListStroke, thickness: f32) {
		unsafe {
			nk_draw_list_path_stroke(self.internal, arg2, closed, thickness);
		}
	}
	
	pub fn stroke_line(&mut self, a: NkVec2, b: NkVec2, arg2: NkColor, thickness: f32) {
		unsafe {
			nk_draw_list_stroke_line(self.internal, a, b, arg2, thickness);
		}
	}
	
	pub fn stroke_rect(&mut self, rect: NkRect, arg2: NkColor, rounding: f32, thickness: f32) {
		unsafe {
			nk_draw_list_stroke_rect(self.internal, rect, arg2, rounding, thickness);
		}
	}
	
	pub fn stroke_triangle(&mut self, a: NkVec2, b: NkVec2, c: NkVec2, arg2: NkColor, thickness: f32) {
		unsafe {
			nk_draw_list_stroke_triangle(self.internal, a, b, c, arg2, thickness);
		}
	}
	
	pub fn stroke_circle(&mut self, center: NkVec2, radius: f32, arg2: NkColor, segs: u32, thickness: f32) {
		unsafe {
			nk_draw_list_stroke_circle(self.internal, center, radius, arg2, segs, thickness);
		}
	}
	
	pub fn stroke_curve(&mut self, p0: NkVec2, cp0: NkVec2, cp1: NkVec2, p1: NkVec2, arg2: NkColor, segments: u32, thickness: f32) {
		unsafe {
			nk_draw_list_stroke_curve(self.internal, p0, cp0, cp1, p1, arg2, segments, thickness);
		}
	}
	
	pub fn stroke_poly_line(&mut self, points: &[NkVec2], arg2: NkColor, arg3: NkDrawListStroke, thickness: f32, aa: NkAntiAliasing) {
		unsafe {
			nk_draw_list_stroke_poly_line(self.internal, &points[0] as *const nk_vec2, points.len() as u32, arg2, arg3, thickness, aa);
		}
	}
	
	pub fn fill_rect(&mut self, rect: NkRect, arg2: NkColor, rounding: f32) {
		unsafe {
			nk_draw_list_fill_rect(self.internal, rect, arg2, rounding);
		}
	}
	
	pub fn fill_rect_multi_color(&mut self, rect: NkRect, left: NkColor, top: NkColor, right: NkColor, bottom: NkColor) {
		unsafe {
			nk_draw_list_fill_rect_multi_color(self.internal, rect, left, top, right, bottom);
		}
	}
	
	pub fn fill_triangle(&mut self, a: NkVec2, b: NkVec2, c: NkVec2, arg2: NkColor) {
		unsafe {
			nk_draw_list_fill_triangle(self.internal, a, b, c, arg2);
		}
	}
	
	pub fn fill_circle(&mut self, center: NkVec2, radius: f32, col: NkColor, segs: u32) {
		unsafe {
			nk_draw_list_fill_circle(self.internal, center, radius, col, segs);
		}
	}
	
	pub fn fill_poly_convex(&mut self, points: &[NkVec2], arg2: NkColor, arg3: NkAntiAliasing) {
		unsafe {
			nk_draw_list_fill_poly_convex(self.internal, &points[0] as *const nk_vec2, points.len() as u32, arg2, arg3);
		}
	}
	
	pub fn add_image(&mut self, texture: NkImage, rect: NkRect, arg2: NkColor) {
		unsafe {
			nk_draw_list_add_image(self.internal, texture.internal, rect, arg2);
		}
	}
	
	pub fn add_text(&mut self, arg2: &NkUserFont, arg3: NkRect, text: NkString, font_height: f32, arg4: NkColor) {
		unsafe {
			nk_draw_list_add_text(self.internal, arg2.internal, arg3, text.as_ptr(), text.bytes.len() as i32, font_height, arg4);
		}
	}
	
	/*pub fn push_userdata(&mut self, userdata: nk_handle) {
		unsafe {
			nk_draw_list_push_userdata(&mut self as *mut nk_draw_list, userdata.internal);
		}
	}*/
}

//========

pub struct NkColorMap {
	internal: [nk_color; 28],
}

impl Default for NkColorMap {
	fn default() -> Self {
		NkColorMap {
			internal: [nk_color::default(); 28],
		}
	}
}

impl NkColorMap {
	pub fn set(&mut self, target: NkStyleColor, color: NkColor) {
		self.internal[target as usize] = color;
	}
}

//==================================================================================

pub struct NkCursorMap {
	internal: [nk_cursor; 7],
}

impl Default for NkCursorMap {
	fn default() -> Self {
		NkCursorMap {
			internal: [nk_cursor::default(); 7],
		}
	}
}

impl NkCursorMap {
	pub fn set(&mut self, target: NkStyleCursor, res: NkCursor) {
		self.internal[target as usize] = res.internal;
	}
}

//==================================================================================

pub struct NkCursor {
	internal: nk_cursor
}

impl Default for NkCursor {
	fn default() -> Self {
		NkCursor {
			internal: nk_cursor::default(),
		}
	}
}

//==================================================================================

#[derive(Clone, Debug)]
pub struct NkAllocator {
	internal: nk_allocator,
}

impl NkAllocator {
	pub fn new_heap() -> NkAllocator {
		let mut a = NkAllocator::default();
		
		a.internal.alloc = Some(alloc_rust);
		a.internal.free = Some(free_rust);
		a.internal.userdata = nk_handle::default();
		unsafe { *(a.internal.userdata.ptr()) = ::std::ptr::null_mut(); }
		
		a
	}
	
	pub fn new_vec() -> NkAllocator {
		let mut a = NkAllocator::default();
		
		a.internal.alloc = Some(alloc_rust_hacky);
		a.internal.free = Some(free_rust_hacky);
		a.internal.userdata = nk_handle::default();
		unsafe { *(a.internal.userdata.ptr()) = ::std::ptr::null_mut(); }
		
		a
	}
}

impl Default for NkAllocator {
	fn default() -> Self {
		NkAllocator {
			internal: nk_allocator::default(),
		}
	}
}

//============================================================================================

pub struct NkConvertConfig {
	internal: nk_convert_config,
}

impl Default for NkConvertConfig {
	fn default() -> Self {
		NkConvertConfig {
			internal: nk_convert_config {
				vertex_alignment: ALIGNMENT,
				..Default::default()
			}
		}
	}
}

impl NkConvertConfig {
	pub fn set_global_alpha(&mut self, val: f32) {
		self.internal.global_alpha = val;
	}
    pub fn set_line_aa(&mut self, val: NkAntiAliasing) {
    	self.internal.line_AA = val;
    }
    pub fn set_shape_aa(&mut self, val: NkAntiAliasing) {
    	self.internal.shape_AA = val;
    }
    pub fn set_circle_segment_count(&mut self, val: u32) {
    	self.internal.circle_segment_count = val;
    }
    pub fn set_arc_segment_count(&mut self, val: u32) {
    	self.internal.arc_segment_count = val;
    }
    pub fn set_curve_segment_count(&mut self, val: u32) {
    	self.internal.curve_segment_count = val;
    }
    pub fn set_null(&mut self, val: NkDrawNullTexture) {
    	self.internal.null = val.internal;
    }
    pub fn set_vertex_layout(&mut self, val: &NkDrawVertexLayoutElements) {
    	self.internal.vertex_layout = &val.arr.as_slice()[0];
    }
    pub fn set_vertex_size(&mut self, val: usize) {
    	self.internal.vertex_size = val;
    }
    /*pub fn set_vertex_alignment(&mut self, val: usize) {
    	self.internal.vertex_alignment = val;
    }*/
}

//============================================================================================

pub struct NkDrawVertexLayoutElements {
	arr: Vec<nk_draw_vertex_layout_element>
}

impl NkDrawVertexLayoutElements {
	pub fn new(var: &[(NkDrawVertexLayoutAttribute, NkDrawVertexLayoutFormat, u32)]) -> NkDrawVertexLayoutElements {
		let mut m = NkDrawVertexLayoutElements {
			arr: Vec::with_capacity(var.len()),
		};
		
		for v in var {
			let &(a, f, o) = v;
			m.arr.push(nk_draw_vertex_layout_element {
				attribute: a,
				format: f,
				offset: o as usize,
			});
		}
		
		m
	}
}

//=============================================================================================

pub struct NkStyleItem {
	internal: nk_style_item,
}

/*impl Default for NkStyleItem {
	fn default() -> Self {
		NkStyleItem {
			internal: nk_style_item::default(),
		}
	}
}*/

impl NkStyleItem {
	pub fn image(img: NkImage) -> NkStyleItem {
		unsafe {
			NkStyleItem {
				internal: nk_style_item_image(img.internal)
			}
		}
	}
	
	pub fn color(col: NkColor) -> NkStyleItem {
		unsafe {
			NkStyleItem {
				internal: nk_style_item_color(col)
			}
		}
	}
	
	pub fn hide() -> NkStyleItem {
		unsafe {
			NkStyleItem {
				internal: nk_style_item_hide()
			}
		}
	}
}

//=============================================================================================

pub struct NkTextEdit {
	internal: *mut nk_text_edit,
}

impl NkTextEdit {
	fn new(t: *mut nk_text_edit) -> NkTextEdit {
		NkTextEdit {
			internal: t,
		}
	}
	
	pub fn init(&mut self, arg2: &mut NkAllocator, size: usize) {
		unsafe {
			nk_textedit_init(self.internal, &mut arg2.internal as *mut nk_allocator, size);
		}
	}
	
	pub fn free(self) {
		unsafe {
			nk_textedit_free(self.internal);    
		}
	}  
	
	pub fn text(&mut self, arg2: &str) {
		unsafe {
			nk_textedit_text(self.internal, arg2.as_ptr() as *const i8, arg2.as_bytes().len() as ::std::os::raw::c_int);
		}
	}
	
	pub fn delete(&mut self, where_: u32, len: u32) {
		unsafe {
			nk_textedit_delete(self.internal, where_ as ::std::os::raw::c_int, len as ::std::os::raw::c_int);
		}
	}
	
	pub fn delete_selection(&mut self) {
		unsafe {
			nk_textedit_delete_selection(self.internal);
		}
	}
	
	pub fn select_all(&mut self) {
		unsafe {
			nk_textedit_select_all(self.internal);
		}
	}
	
	pub fn cut(&mut self) -> bool {
		unsafe {
			nk_textedit_cut(self.internal) != 0
		}
	}
	
	pub fn paste(&mut self, arg2: &str) -> bool {
		unsafe {
			nk_textedit_paste(self.internal, arg2.as_ptr() as *const i8, arg2.as_bytes().len() as ::std::os::raw::c_int) != 0
		}
	}
	
	pub fn undo(&mut self) {
		unsafe {
			nk_textedit_undo(self.internal);
		}
	}
	
	pub fn redo(&mut self) {
		unsafe {
			nk_textedit_redo(self.internal);
		}
	}
	
	/*
	
    pub fn nk_textedit_init_fixed(arg1: *mut nk_text_edit,
                                  memory: *mut ::std::os::raw::c_void,
                                  size: nk_size);
	*/
}

//=============================================================================================

#[derive(Clone, Debug)]
pub struct NkFontConfig {
	internal: nk_font_config,
}

impl NkFontConfig {
	pub fn new(pixel_height: f32) -> NkFontConfig {
		unsafe { 
			NkFontConfig {
				internal: nk_font_config(pixel_height),
			}
		}
	}
	
	pub fn set_oversample_v(&mut self, v: u8) {
		self.internal.oversample_v = v;
	}
	
	pub fn set_oversample_h(&mut self, h: u8) {
		self.internal.oversample_h = h;
	}
}

//=============================================================================================

#[derive(Clone, Debug)]
pub enum NkError {
	FontAtlasAlreadyFinalized
}

#[derive(Clone, Debug)]
pub struct NkFontAtlas {
	internal: nk_font_atlas,
	state: NkFontAtlasState,
	fonts: Vec<NkFont>,
}

impl Default for NkFontAtlas {
	fn default() -> Self {
		NkFontAtlas {
			internal: nk_font_atlas::default(),
			state: NkFontAtlasState::New,
			fonts: Vec::new(),
		}
	}
}

impl NkFontAtlas {
	pub fn new(alloc: &mut NkAllocator) -> NkFontAtlas {
		let mut a = NkFontAtlas::default();
		
		a.init(alloc);
		
		a
	}
	
	pub fn add_font_with_config(&mut self, cfg: &NkFontConfig) -> Result<NkFont, NkError> {
		match self.state {
			NkFontAtlasState::New => unsafe { 
				nk_font_atlas_begin(&mut self.internal as *mut nk_font_atlas);
				self.state = NkFontAtlasState::Started;
			},
			NkFontAtlasState::Finalized => return Err(NkError::FontAtlasAlreadyFinalized),
			_ => {},
		}
		
		self.fonts.push(NkFont::new(unsafe {
	    	nk_font_atlas_add(&mut self.internal as *mut nk_font_atlas, &cfg.internal as *const nk_font_config)
	    }));
	    
	    Ok(self.fonts[self.fonts.len()-1].clone())
	}
	
	pub fn add_font_with_bytes(&mut self, font_bytes: &[u8], font_size: f32) -> Result<NkFont, NkError> {
		let mut cfg = NkFontConfig::new(font_size);
		
		cfg.internal.ttf_size = font_bytes.len();
	    cfg.internal.ttf_blob = font_bytes as *const _ as *mut c_void;
	    cfg.internal.size = font_size;
	    cfg.internal.ttf_data_owned_by_atlas = 1;
	    
	    self.add_font_with_config(&cfg)
	}
	
	pub fn bake(&mut self, format: NkFontAtlasFormat) -> (Vec<u8>, usize, usize) {
		let mut width: i32 = 0;
		let mut height: i32 = 0;
		
		let image = unsafe {
			nk_font_atlas_bake(&mut self.internal as *mut nk_font_atlas, &mut width as *mut c_int, &mut height as *mut c_int, format)
		};
		
		if width < 1 || height < 1 {
			return (vec![], width as usize, height as usize);
		}
		
		let size = (match format {
			NkFontAtlasFormat::NK_FONT_ATLAS_ALPHA8 => 1,
			NkFontAtlasFormat::NK_FONT_ATLAS_RGBA32 => 4,
		} * width * height) as usize;
		
//		self.state = NkFontAtlasState::Finalized;
		
		(
			unsafe { 
				Vec::from_raw_parts(image as *mut u8, size, size) 
			}, 
			width as usize, 
			height as usize
		)
	}
	
	pub fn end(&mut self, hnd: NkHandle, null_texture: Option<&mut NkDrawNullTexture>) {
		let nullt = match null_texture {
			Some(n) => &mut n.internal as *mut nk_draw_null_texture,
			None => ::std::ptr::null_mut(),
		};
		unsafe {
			nk_font_atlas_end(&mut self.internal as *mut nk_font_atlas, hnd.internal, nullt);
		}
		self.state = NkFontAtlasState::Finalized;
	}
	
	pub fn clear(&mut self) {
		unsafe {
			nk_font_atlas_clear(&mut self.internal as *mut nk_font_atlas);
		}
	}
	
	fn init(&mut self, arg2: &mut NkAllocator) {
		unsafe {
			nk_font_atlas_init(&mut self.internal as *mut nk_font_atlas, &mut arg2.internal as *mut nk_allocator);
		}
	}
	
	#[allow(dead_code)]
	fn init_custom(&mut self, persistent: &mut NkAllocator, transient: &mut NkAllocator) {
		unsafe {
			nk_font_atlas_init_custom(&mut self.internal as *mut nk_font_atlas, &mut persistent.internal as *mut nk_allocator, &mut transient.internal as *mut nk_allocator);
		}
	}
	
	pub fn begin(&mut self) {
		unsafe {
			nk_font_atlas_begin(&mut self.internal as *mut nk_font_atlas);
		}
	}
	
	/*
	pub fn nk_font_atlas_add_from_memory(atlas: *mut nk_font_atlas,
                                         memory: *mut ::std::os::raw::c_void,
                                         size: nk_size, height: f32,
                                         config: *const nk_font_config)
     -> *mut nk_font;
    pub fn nk_font_atlas_add_compressed(arg1: *mut nk_font_atlas,
                                        memory: *mut ::std::os::raw::c_void,
                                        size: nk_size, height: f32,
                                        arg2: *const nk_font_config)
     -> *mut nk_font;
    pub fn nk_font_atlas_add_compressed_base85(arg1: *mut nk_font_atlas,
                                               data:
                                                   *const ::std::os::raw::c_char,
                                               height: f32,
                                               config: *const nk_font_config)
     -> *mut nk_font;
	*/
}

#[derive(Clone, Debug)]
enum NkFontAtlasState {
	New, Started, Finalized
}

//=============================================================================================

#[derive(Clone, Debug)]
pub struct NkDrawNullTexture {
	internal: nk_draw_null_texture,
}

impl Default for NkDrawNullTexture {
	fn default() -> Self {
		NkDrawNullTexture {
			internal: nk_draw_null_texture::default(),
		}
	}
}

//=============================================================================================

const DEFAULT_BUFFER_SIZE: usize = 8096;

pub struct NkBuffer {
	internal: nk_buffer,
}

impl Default for NkBuffer {
	fn default() -> Self {
		NkBuffer {
			internal: nk_buffer::default(),
		}
	}
}

impl NkBuffer {
	pub fn new(alloc: &mut NkAllocator) -> NkBuffer {
		NkBuffer::with_size(alloc, DEFAULT_BUFFER_SIZE)
	}
	
	pub fn with_size(alloc: &mut NkAllocator, buffer_size: usize) -> NkBuffer {
		let mut a = NkBuffer::default();
		
		unsafe {
			nk_buffer_init(&mut a.internal as *mut nk_buffer, &mut alloc.internal as *const nk_allocator, buffer_size);
		}
		
		a
	}
	
	pub fn with_fixed(memory: &mut [u8]) -> NkBuffer {
		let mut a = NkBuffer::default();
		
		unsafe {
			nk_buffer_init_fixed(&mut a.internal as *mut nk_buffer, memory as *mut _ as *mut ::std::os::raw::c_void, memory.len());
		}
		
		a
	}
	
	pub fn total(&mut self) -> usize {
		unsafe {
			nk_buffer_total(&mut self.internal as *mut nk_buffer)
		}
	}
	
	pub fn info(&mut self) -> (usize, usize, usize, usize) /*size, allocated, needed, calls*/ {
		let mut s = nk_memory_status::default();
		unsafe {
			nk_buffer_info(&mut s, &mut self.internal as *mut nk_buffer);
		}
		(s.size, s.allocated, s.needed, s.calls)
	}
	
	/*
	pub fn nk_buffer_push(arg1: *mut nk_buffer,
                          type_: nk_buffer_allocation_type,
                          memory: *const ::std::os::raw::c_void,
                          size: nk_size, align: nk_size);
    pub fn nk_buffer_mark(arg1: *mut nk_buffer,
                          type_: nk_buffer_allocation_type);
    pub fn nk_buffer_reset(arg1: *mut nk_buffer,
                           type_: nk_buffer_allocation_type);
    pub fn nk_buffer_clear(arg1: *mut nk_buffer);
    pub fn nk_buffer_free(arg1: *mut nk_buffer);
    pub fn nk_buffer_memory(arg1: *mut nk_buffer)
     -> *mut ::std::os::raw::c_void;
    pub fn nk_buffer_memory_const(arg1: *const nk_buffer)
     -> *const ::std::os::raw::c_void;
    pub fn nk_buffer_total(arg1: *mut nk_buffer) -> nk_size;
	
	pub fn nk_buffer_init(arg1: *mut nk_buffer, arg2: *const nk_allocator,
                          size: nk_size);
    pub fn nk_buffer_init_fixed(arg1: *mut nk_buffer,
                                memory: *mut ::std::os::raw::c_void,
                                size: nk_size);
    */
}

//=============================================================================================

pub struct NkContext {
	internal: nk_context,
}

impl Default for NkContext {
	fn default() -> Self {
		NkContext {
			internal: nk_context::default(),
		}
	}
}

impl NkContext {
	pub fn new(alloc: &mut NkAllocator, font: &NkUserFont) ->NkContext {
		let mut a = NkContext::default();
		
		unsafe {
			nk_init(&mut a.internal as *mut nk_context, &mut alloc.internal as *mut nk_allocator, font.internal as *const nk_user_font);
		}
		
		a
	}
	
	pub fn input(&mut self) -> NkInput {
		NkInput::new(&mut self.internal.input)
	}
	
	pub fn style(&mut self) -> NkStyle {
		NkStyle {
			internal: &mut self.internal.style
		}
	}
	
	pub fn draw_list(&mut self) -> NkDrawList {
		NkDrawList::new(&mut self.internal.draw_list)
	}
	
	pub fn clear(&mut self) {
		unsafe {
			nk_clear(&mut self.internal as *mut nk_context);
		}
	}
	
	pub fn free(&mut self) {
		unsafe {
			nk_free(&mut self.internal as *mut nk_context);
		}
	}
	
	pub fn begin(&mut self, panel: &mut NkPanel, title: NkString, bounds: NkRect, flags: NkFlags) -> i32 {
		unsafe {
			nk_begin(&mut self.internal as *mut nk_context, &mut panel.internal, title.as_ptr(), bounds, flags)
		}
	}
	
	pub fn begin_titled(&mut self, panel: &mut NkPanel, name: NkString, title: NkString, bounds: NkRect, flags: NkFlags) -> i32 {
		unsafe {
			nk_begin_titled(&mut self.internal as *mut nk_context, &mut panel.internal, name.as_ptr(), title.as_ptr(), bounds, flags)
		}
	}
	
	pub fn end(&mut self) {
		unsafe {
			nk_end(&mut self.internal as *mut nk_context);
		}
	}
	
	pub fn window_find(&mut self, name: NkString) -> Option<NkWindow> {
		let w = unsafe {
			nk_window_find(&mut self.internal as *mut nk_context, name.as_ptr())
		};
		
		if w.is_null() {
			None
		} else {
			Some(NkWindow::new(w))
		}
	}
	
	pub fn window_get_bounds(&self) -> NkRect {
		unsafe {
			nk_window_get_bounds(&self.internal as *const nk_context)
		}
	}
	
	pub fn window_get_size(&self) -> NkVec2 {
		unsafe {
			nk_window_get_size(&self.internal as *const nk_context)
		}
	}
	
	pub fn window_get_position(&self) -> NkVec2 {
		unsafe {
			nk_window_get_position(&self.internal as *const nk_context)
		}
	}
	
	pub fn window_get_width(&self) -> f32 {
		unsafe {
			nk_window_get_width(&self.internal as *const nk_context)
		}
	}
	
	pub fn window_get_height(&self) -> f32 {
		unsafe {
			nk_window_get_height(&self.internal as *const nk_context)
		}
	}
	
	pub fn window_get_panel(&mut self) -> Option<NkPanelRef> {
		let p = unsafe {
			nk_window_get_panel(&mut self.internal as *mut nk_context)
		};
		
		if p.is_null() {
			None
		} else {
			Some(NkPanelRef::new(p))
		}
	}
	
	pub fn window_get_content_region(&mut self) -> NkRect {
		unsafe {
			nk_window_get_content_region(&mut self.internal as *mut nk_context)
		}
	}
	
	pub fn window_get_content_region_min(&mut self) -> NkVec2 {
		unsafe {
			nk_window_get_content_region_min(&mut self.internal as *mut nk_context)
		}
	}
	
	pub fn window_get_content_region_max(&mut self) -> NkVec2 {
		unsafe {
			nk_window_get_content_region_max(&mut self.internal as *mut nk_context)
		}
	}
	
	pub fn window_get_content_region_size(&mut self) -> NkVec2 {
		unsafe {
			nk_window_get_content_region_size(&mut self.internal as *mut nk_context)
		}
	}
	
	pub fn window_get_canvas(&mut self) -> Option<NkCommandBuffer> {
		let b = unsafe {
			nk_window_get_canvas(&mut self.internal as *mut nk_context)
		};
		
		if b.is_null() {
			None
		} else {
			Some(NkCommandBuffer::new(b))
		}
	}
	
	pub fn window_has_focus(&self) -> bool {
		unsafe {
			nk_window_has_focus(&self.internal as *const nk_context) > 0
		}
	}
	
	pub fn window_is_collapsed(&mut self, name: NkString) -> bool {
		unsafe {
			nk_window_is_collapsed(&mut self.internal as *mut nk_context, name.as_ptr()) > 0
		}
	}
	
	pub fn window_is_closed(&mut self, name: NkString) -> bool {
		unsafe {
			nk_window_is_closed(&mut self.internal as *mut nk_context, name.as_ptr()) > 0
		}
	}
	
	pub fn window_is_hidden(&mut self, name: NkString) -> bool {
		unsafe {
			nk_window_is_hidden(&mut self.internal as *mut nk_context, name.as_ptr()) > 0
		}
	}
	
	pub fn window_is_active(&mut self, name: NkString) -> bool {
		unsafe {
			nk_window_is_active(&mut self.internal as *mut nk_context, name.as_ptr()) > 0
		}
	}
	
	pub fn window_is_hovered(&mut self) -> bool {
		unsafe {
			nk_window_is_hovered(&mut self.internal as *mut nk_context) > 0
		}
	}
	
	pub fn window_is_any_hovered(&mut self) -> bool {
		unsafe {
			nk_window_is_any_hovered(&mut self.internal as *mut nk_context) > 0
		}
	}
	
	pub fn item_is_any_active(&mut self) -> bool {
		unsafe {
			nk_item_is_any_active(&mut self.internal as *mut nk_context) > 0
		}
	}
	
	pub fn window_set_bounds(&mut self, bounds: NkRect) {
		unsafe {
			nk_window_set_bounds(&mut self.internal as *mut nk_context, bounds);
		}
	}
	
	pub fn window_set_position(&mut self, pos: NkVec2) {
		unsafe {
			nk_window_set_position(&mut self.internal as *mut nk_context, pos);
		}
	}
	
	pub fn window_set_size(&mut self, size: NkVec2) {
		unsafe {
			nk_window_set_size(&mut self.internal as *mut nk_context, size);
		}
	}
	
	pub fn window_set_focus(&mut self, name: NkString) {
		unsafe {
			nk_window_set_focus(&mut self.internal as *mut nk_context, name.as_ptr());
		}
	}
	
	pub fn window_close(&mut self, name: NkString) {
		unsafe {
			nk_window_close(&mut self.internal as *mut nk_context, name.as_ptr());
		}
	}
	
	pub fn window_collapse(&mut self, name: NkString, state: NkCollapseState) {
		unsafe {
			nk_window_collapse(&mut self.internal as *mut nk_context, name.as_ptr(), state);
		}
	}
	
	pub fn window_collapse_if(&mut self, name: NkString, state: NkCollapseState, cond: bool) {
		unsafe {
			nk_window_collapse_if(&mut self.internal as *mut nk_context, name.as_ptr(), state, if cond { 1 } else { 0 });
		}
	}
	
	pub fn window_show(&mut self, name: NkString, state: NkShowState) {
		unsafe {
			nk_window_show(&mut self.internal as *mut nk_context, name.as_ptr(), state);
		}
	}
	
	pub fn window_show_if(&mut self, name: NkString, state: NkShowState, cond: bool) {
		unsafe {
			nk_window_show_if(&mut self.internal as *mut nk_context, name.as_ptr(), state, if cond { 1 } else { 0 });
		}
	}
	
	pub fn layout_row_dynamic(&mut self, height: f32, cols: i32) {
		unsafe {
			nk_layout_row_dynamic(&mut self.internal as *mut nk_context, height, cols);
		}
	}	
	
	pub fn layout_row_static(&mut self, height: f32, item_width: i32, cols: i32) {
		unsafe {
			nk_layout_row_static(&mut self.internal as *mut nk_context, height, item_width, cols);
		}
	}	
	
	pub fn layout_row_begin(&mut self, fmt: NkLayoutFormat, row_height: f32, cols: i32) {
		unsafe {
			nk_layout_row_begin(&mut self.internal as *mut nk_context, fmt, row_height, cols);
		}
	}
	
	pub fn layout_row_push(&mut self, value: f32) {
		unsafe {
			nk_layout_row_push(&mut self.internal as *mut nk_context, value);
		}
	}
	
	pub fn layout_row_end(&mut self) {
		unsafe {
			nk_layout_row_end(&mut self.internal as *mut nk_context);
		}
	}
	
	pub fn layout_row(&mut self, fmt: NkLayoutFormat, height: f32, cols_ratio: &[f32]) {
		unsafe {
			nk_layout_row(&mut self.internal as *mut nk_context, fmt, height, cols_ratio.len() as i32, &cols_ratio[0] as *const f32);
		}
	}
	
	pub fn layout_space_begin(&mut self, fmt: NkLayoutFormat, height: f32, widget_count: i32) {
		unsafe {
			nk_layout_space_begin(&mut self.internal as *mut nk_context, fmt, height, widget_count);
		}
	}
	
	pub fn layout_space_push(&mut self, space: NkRect) {
		unsafe {
			nk_layout_space_push(&mut self.internal as *mut nk_context, space);
		}
	}
	
	pub fn layout_space_end(&mut self) {
		unsafe {
			nk_layout_space_end(&mut self.internal as *mut nk_context);
		}
	}
	
	pub fn layout_space_bounds(&mut self) -> NkRect {
		unsafe {
			nk_layout_space_bounds(&mut self.internal as *mut nk_context)
		}
	}
	
	pub fn layout_space_to_screen(&mut self, space: NkVec2) -> NkVec2 {
		unsafe {
			nk_layout_space_to_screen(&mut self.internal as *mut nk_context, space)
		}
	}
	
	pub fn layout_space_to_local(&mut self, space: NkVec2) -> NkVec2 {
		unsafe {
			nk_layout_space_to_local(&mut self.internal as *mut nk_context, space)
		}
	}
	
	pub fn layout_space_rect_to_screen(&mut self, space: NkRect) -> NkRect {
		unsafe {
			nk_layout_space_rect_to_screen(&mut self.internal as *mut nk_context, space)
		}
	}
	
	pub fn layout_space_rect_to_local(&mut self, space: NkRect) -> NkRect {
		unsafe {
			nk_layout_space_rect_to_local(&mut self.internal as *mut nk_context, space)
		}
	}
	
	pub fn layout_ratio_from_pixel(&mut self, pixel_width: f32) -> f32 {
		unsafe {
			nk_layout_ratio_from_pixel(&mut self.internal as *mut nk_context, pixel_width)
		}
	}
	
	pub fn nk_group_begin(&mut self, panel: &mut NkPanel, title: NkString, flags: NkFlags) -> i32 {
         unsafe {
         	nk_group_begin(&mut self.internal as *mut nk_context, &mut panel.internal, title.as_ptr(), flags)
         }                 	
    }
	
	pub fn group_end(&mut self) {
		unsafe {
			nk_group_end(&mut self.internal as *mut nk_context);
		}
	}
	
	pub fn tree_push_hashed(&mut self, ty: NkTreeType, title: NkString, initial_state: NkCollapseState, hash: NkString, len: i32, seed: i32) -> i32 {
	   	unsafe {
	   		nk_tree_push_hashed(&mut self.internal as *mut nk_context, ty, title.as_ptr(), initial_state, hash.as_ptr(), len, seed)
	   	}
	}
	
	pub fn tree_image_push_hashed(&mut self, ty: NkTreeType, i: NkImage, title: NkString, initial_state: NkCollapseState, hash: NkString, len: i32, seed: i32) -> i32 {
	   	unsafe {
	   		nk_tree_image_push_hashed(&mut self.internal as *mut nk_context, ty, i.internal, title.as_ptr(), initial_state, hash.as_ptr(), len, seed)
	   	}
	}
	
	pub fn tree_pop(&mut self) {
		unsafe {
			nk_tree_pop(&mut self.internal as *mut nk_context);
		}
	}
	
	pub fn text(&mut self, text: &str, flags: NkFlags) {
		unsafe {
			nk_text(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32, flags);
		}
	}
	
	pub fn text_colored(&mut self, text: &str, flags: NkFlags, color: NkColor) {
		unsafe {
			nk_text_colored(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32, flags, color);
		}
	}
		
	pub fn text_wrap(&mut self, text: &str) {
		unsafe {
			nk_text_wrap(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32);
		}
	}
	
	pub fn text_wrap_colored(&mut self, text: &str, color: NkColor) {
		unsafe {
			nk_text_wrap_colored(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32, color);
		}
	}
	
	pub fn label(&mut self, text: NkString, flags: NkFlags) {
		unsafe {
			nk_label(&mut self.internal as *mut nk_context, text.as_ptr(), flags);
		}
	}
	
	pub fn label_colored(&mut self, text: NkString, flags: NkFlags, color: NkColor) {
		unsafe {
			nk_label_colored(&mut self.internal as *mut nk_context, text.as_ptr(), flags, color);
		}
	}
		
	pub fn label_wrap(&mut self, text: NkString) {
		unsafe {
			nk_label_wrap(&mut self.internal as *mut nk_context, text.as_ptr());
		}
	}
	
	pub fn label_colored_wrap(&mut self, text: NkString, color: NkColor) {
		unsafe {
			nk_label_colored_wrap(&mut self.internal as *mut nk_context, text.as_ptr(), color);
		}
	}
	
	pub fn image(&mut self, img: NkImage) {
		unsafe {
			nk_image(&mut self.internal as *mut nk_context, img.internal);
		}
	}
	
	pub fn button_text(&mut self, text: &str) -> bool {
		unsafe {
			nk_button_text(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32) != 0
		}
	}
	
	pub fn button_label(&mut self, title: NkString) -> bool {
		unsafe {
			nk_button_label(&mut self.internal as *mut nk_context, title.as_ptr()) != 0
		}
	}
	
	pub fn button_color(&mut self, color: NkColor) -> bool {
		unsafe {
			nk_button_color(&mut self.internal as *mut nk_context, color) != 0
		}
	}
	
	pub fn button_symbol(&mut self, ty: NkSymbolType) -> bool {
     	unsafe {
     		nk_button_symbol(&mut self.internal as *mut nk_context, ty) != 0
     	}
    }
	
	pub fn button_image(&mut self, img: NkImage) -> bool {
     	unsafe {
     		nk_button_image(&mut self.internal as *mut nk_context, img.internal) != 0
     	}
    }
	
	pub fn button_symbol_label(&mut self, ty: NkSymbolType, title: NkString, text_alignment: NkFlags) -> bool {
	  	unsafe {
	  		nk_button_symbol_label(&mut self.internal as *mut nk_context, ty, title.as_ptr(), text_alignment) != 0
	  	}
	}
	
	pub fn button_image_label(&mut self, img: NkImage, title: NkString, text_alignment: NkFlags) -> bool {
		unsafe {
			nk_button_image_label(&mut self.internal as *mut nk_context, img.internal, title.as_ptr(), text_alignment) != 0
		}
	}
	
	pub fn button_set_behavior(&mut self, b: NkButtonBehavior) {
		unsafe {
			nk_button_set_behavior(&mut self.internal as *mut nk_context, b);
		}
	}
	
	pub fn button_push_behavior(&mut self, b: NkButtonBehavior) -> i32 {
		unsafe {
			nk_button_push_behavior(&mut self.internal as *mut nk_context, b)
		}
	}
	
	pub fn button_pop_behavior(&mut self) -> i32 {
		unsafe {
			nk_button_pop_behavior(&mut self.internal as *mut nk_context)
		}
	}
	
	pub fn check_label(&mut self, title: NkString, active: bool) -> i32 {
		unsafe {
			nk_check_label(&mut self.internal as *mut nk_context, title.as_ptr(), if active { 1 } else { 0 })
		}
	}
	
	pub fn check_flags_label(&mut self, title: NkString, flags: u32, value: u32) -> u32 {
		unsafe {
			nk_check_flags_label(&mut self.internal as *mut nk_context, title.as_ptr(), flags, value)
		}
	}
	
	pub fn checkbox_label(&mut self, title: NkString, active: &mut bool) -> bool {
		let mut i = if *active { 1 } else { 0 };
		let r = unsafe {
			nk_checkbox_label(&mut self.internal as *mut nk_context, title.as_ptr(), &mut i as *mut i32) != 0
		};
		
		*active = i != 0;
		r
	}
	
	pub fn checkbox_flags_label(&mut self, title: NkString, flags: &mut u32, value: u32) -> bool {
		unsafe {
			nk_checkbox_flags_label(&mut self.internal as *mut nk_context, title.as_ptr(), flags, value) != 0
		}
	}
	
	pub fn radio_label(&mut self, title: NkString, active: &mut bool) -> bool {
		let mut i = if *active { 1 } else { 0 };
		let r = unsafe {
			nk_radio_label(&mut self.internal as *mut nk_context, title.as_ptr(), &mut i as *mut i32) != 0
		};
		
		*active = i != 0;
		r
	}
	
	pub fn option_label(&mut self, title: NkString, active: bool) -> bool {
		unsafe {
			nk_option_label(&mut self.internal as *mut nk_context, title.as_ptr(), if active { 1 } else { 0 }) > 0
		}
	}
	
	pub fn selectable_label(&mut self, title: NkString, align: NkFlags, value: &mut i32) -> bool {
		unsafe {
			nk_selectable_label(&mut self.internal as *mut nk_context, title.as_ptr(), align, value) != 0
		}
	}
	
	pub fn selectable_image_label(&mut self, img: NkImage, title: NkString, align: NkFlags, value: &mut i32) -> bool {
		unsafe {
			nk_selectable_image_label(&mut self.internal as *mut nk_context, img.internal, title.as_ptr(), align, value) != 0
		}
	}
	
	pub fn select_label(&mut self, title: NkString, align: NkFlags, value: i32) -> i32 {
		unsafe {
			nk_select_label(&mut self.internal as *mut nk_context, title.as_ptr(), align, value) 
		}
	}
	
	pub fn select_image_label(&mut self, img: NkImage, title: NkString, align: NkFlags, value: i32) -> i32 {
		unsafe {
			nk_select_image_label(&mut self.internal as *mut nk_context, img.internal, title.as_ptr(), align, value) 
		}
	}
	
	pub fn slide_float(&mut self, min: f32, val: f32, max: f32, step: f32) -> f32 {
		unsafe {
			nk_slide_float(&mut self.internal as *mut nk_context, min, val, max, step)
		}
	}
	
	pub fn slide_int(&mut self, min: i32, val: i32, max: i32, step: i32) -> i32 {
		unsafe {
			nk_slide_int(&mut self.internal as *mut nk_context, min, val, max, step)
		}
	}
	
	pub fn slider_float(&mut self, min: f32, val: &mut f32, max: f32, step: f32) -> bool {
		unsafe {
			nk_slider_float(&mut self.internal as *mut nk_context, min, val, max, step) != 0
		}
	}
	
	pub fn slider_int(&mut self, min: i32, val: &mut i32, max: i32, step: i32) -> bool {
		unsafe {
			nk_slider_int(&mut self.internal as *mut nk_context, min, val, max, step) != 0
		}
	}
	
	pub fn progress(&mut self, cur: &mut usize, max: usize, is_modifyable: bool) -> bool {
		unsafe {
			nk_progress(&mut self.internal as *mut nk_context, cur, max, if is_modifyable { 1 } else { 0 }) != 0
		}
	}
	
	pub fn prog(&mut self, cur: usize, max: usize, is_modifyable: bool) -> usize {
		unsafe {
			nk_prog(&mut self.internal as *mut nk_context, cur, max, if is_modifyable { 1 } else { 0 })
		}
	}
	
	pub fn color_picker(&mut self, color: NkColor, fmt: NkColorFormat) -> NkColor {
		unsafe {
			nk_color_picker(&mut self.internal as *mut nk_context, color, fmt)
		}
	}
	
	pub fn color_pick(&mut self, fmt: NkColorFormat) -> (bool, NkColor) {
		let mut c = NkColor::default();
		let changed = unsafe {
			nk_color_pick(&mut self.internal as *mut nk_context, &mut c as *mut nk_color, fmt)
		};
		(changed != 0, c)
	}
	
	pub fn property_int(&mut self, name: NkString, min: i32, val: &mut i32, max: i32, step: i32, inc_per_pixel: f32) {
		unsafe {
			nk_property_int(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel);
		}
	}
	
	pub fn property_float(&mut self, name: NkString, min: f32, val: &mut f32, max: f32, step: f32, inc_per_pixel: f32) {
		unsafe {
			nk_property_float(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel)
		}
	}
	
	pub fn property_double(&mut self, name: NkString, min: f64, val: &mut f64, max: f64, step: f64, inc_per_pixel: f32) {
		unsafe {
			nk_property_double(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel)
		}
	}
	
	pub fn propertyi(&mut self, name: NkString, min: i32, val: i32, max: i32, step: i32, inc_per_pixel: f32) -> i32 {
		unsafe {
			nk_propertyi(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel)
		}
	}
	
	pub fn propertyf(&mut self, name: NkString, min: f32, val: f32, max: f32, step: f32, inc_per_pixel: f32) -> f32 {
		unsafe {
			nk_propertyf(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel)
		}
	}
	
	pub fn propertyd(&mut self, name: NkString, min: f64, val: f64, max: f64, step: f64, inc_per_pixel: f32) -> f64 {
		unsafe {
			nk_propertyd(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel)
		}
	}
	
	pub fn edit_string(&mut self, flags: NkFlags, buffer: &mut [u8], len: &mut i32, filter: NkPluginFilter) -> NkFlags {
		unsafe {
			nk_edit_string(&mut self.internal as *mut nk_context, flags, &mut buffer[0] as *mut _ as *mut i8, len, buffer.len() as i32, filter)
		}
	}
	
	pub fn edit_buffer(&mut self, flags: NkFlags, editor: &NkTextEdit, filter: NkPluginFilter) -> NkFlags {
		unsafe {
			nk_edit_buffer(&mut self.internal as *mut nk_context, flags, editor.internal, filter)
		}
	}
	
	pub fn chart_begin(&mut self, ty: NkChartType, num: i32, min: f32, max: f32) -> bool {
		unsafe {
			nk_chart_begin(&mut self.internal as *mut nk_context, ty, num, min, max) > 0
		}
	}
	
	pub fn chart_begin_colored(&mut self, ty: NkChartType, color: NkColor, active: NkColor, num: i32, min: f32, max: f32) -> bool {
		unsafe {
			nk_chart_begin_colored(&mut self.internal as *mut nk_context, ty, color, active, num, min, max) > 0
		}
	}
	
	pub fn chart_add_slot(&mut self, ty: NkChartType, count: i32, min_value: f32, max_value: f32) {
		unsafe {
			nk_chart_add_slot(&mut self.internal as *mut nk_context, ty, count, min_value, max_value);
		}
	}
    
	pub fn chart_add_slot_colored(&mut self, ty: NkChartType, color: NkColor, active: NkColor, count: i32, min_value: f32, max_value: f32) {
		unsafe {
			nk_chart_add_slot_colored(&mut self.internal as *mut nk_context, ty, color, active, count, min_value, max_value);
		}
	}
	
	pub fn chart_push(&mut self, value: f32) -> NkFlags {
		unsafe {
			nk_chart_push(&mut self.internal as *mut nk_context, value)
		}
	}
	
	pub fn chart_push_slot(&mut self, value: f32, count: i32) -> nk_flags {
		unsafe {
			nk_chart_push_slot(&mut self.internal as *mut nk_context, value, count)
		}
	}
	
	pub fn chart_end(&mut self) {
		unsafe {
			nk_chart_end(&mut self.internal as *mut nk_context);    
		}
	}
    
    pub fn plot(&mut self, ty: NkChartType, values: &[f32]) {
    	unsafe {
    		nk_plot(&mut self.internal as *mut nk_context, ty, &values[0] as *const f32, values.len() as i32, 0);
    	}
    }
    
    /*pub fn plot_function(&mut self, ty: NkChartType, userdata: *mut ::std::os::raw::c_void, value_getter: NkPlotValueGetter, count: i32, offset: i32) {
    	unsafe {
    		nk_plot_function(&mut self.internal as *mut nk_context, ty: NkChartType, userdata, value_getter, count, offset);
    	}
    }*/
    
    pub fn popup_begin(&mut self, panel: &mut NkPanel, ty: NkPopupType, title: NkString, flags: NkFlags, bounds: NkRect) -> bool {
    	unsafe {
    		nk_popup_begin(&mut self.internal as *mut nk_context, &mut panel.internal, ty, title.as_ptr(), flags, bounds) > 0
    	}
    }
    
    pub fn popup_close(&mut self) {
    	unsafe {
    		nk_popup_close(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn popup_end(&mut self) {
    	unsafe {
    		nk_popup_end(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn combo(&mut self, items: &mut NkStringArray, selected: i32, item_height: i32, max_height: i32) -> i32 {
    	unsafe {
    		nk_combo(&mut self.internal as *mut nk_context, items.as_mut(), items.len() as i32, selected, item_height, max_height)
    	}
    }
    
    pub fn combo_separator(&mut self, items_separated_by_separator: NkString, separator: char, selected: i32, item_height: i32, max_height: i32) -> i32 {
    	let len = String::from_utf8_lossy(items_separated_by_separator.bytes.as_ref()).as_ref().split(separator).count();
    	unsafe {
    		nk_combo_separator(&mut self.internal as *mut nk_context, items_separated_by_separator.as_ptr(), separator as i32, selected, len as i32, item_height, max_height)
    	}
    }
    
    pub fn combo_begin_label(&mut self, panel: &mut NkPanel, selected: NkString, max_height: i32) -> bool {
    	unsafe {
    		nk_combo_begin_label(&mut self.internal as *mut nk_context, &mut panel.internal, selected.as_ptr(), max_height) > 0
    	}
    }
    
    pub fn combo_begin_color(&mut self, panel: &mut NkPanel, color: NkColor, max_height: i32) -> bool {
    	unsafe {
    		nk_combo_begin_color(&mut self.internal as *mut nk_context, &mut panel.internal, color, max_height) > 0
    	}
    }
    
    pub fn combo_begin_symbol(&mut self, panel: &mut NkPanel, sym: NkSymbolType, max_height: i32) -> bool {
    	unsafe {
    		nk_combo_begin_symbol(&mut self.internal as *mut nk_context, &mut panel.internal, sym, max_height) > 0
    	}
    }
    
    pub fn combo_begin_symbol_label(&mut self, panel: &mut NkPanel, label: NkString, sym: NkSymbolType, max_height: i32) -> bool {
    	unsafe {
    		nk_combo_begin_symbol_label(&mut self.internal as *mut nk_context, &mut panel.internal, label.as_ptr(), sym, max_height) > 0
    	}
    }
    
    pub fn combo_begin_image(&mut self, panel: &mut NkPanel, img: NkImage, max_height: i32) -> bool {
    	unsafe {
    		nk_combo_begin_image(&mut self.internal as *mut nk_context, &mut panel.internal, img.internal, max_height) > 0
    	}
    }
    
    pub fn combo_begin_image_label(&mut self, panel: &mut NkPanel, label: NkString, img: NkImage, max_height: i32) -> bool {
    	unsafe {
    		nk_combo_begin_image_label(&mut self.internal as *mut nk_context, &mut panel.internal, label.as_ptr(), img.internal, max_height) > 0
    	}
    }
    
    pub fn combo_item_label(&mut self, label: NkString, alignment: NkFlags) -> bool {
    	unsafe {
    		nk_combo_item_label(&mut self.internal as *mut nk_context, label.as_ptr(), alignment) > 0
    	}
    }
    
    pub fn combo_item_image_label(&mut self, img: NkImage, label: NkString, alignment: NkFlags) -> bool {
    	unsafe {
    		nk_combo_item_image_label(&mut self.internal as *mut nk_context, img.internal, label.as_ptr(), alignment) > 0
    	}
    }
    
    pub fn combo_item_symbol_label(&mut self, sym: NkSymbolType, label: NkString, alignment: NkFlags) -> bool {
    	unsafe {
    		nk_combo_item_symbol_label(&mut self.internal as *mut nk_context, sym, label.as_ptr(), alignment) > 0
    	}
    }
    
    pub fn combo_close(&mut self) {
    	unsafe {
    		nk_combo_close(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn combo_end(&mut self) {
    	unsafe {
    		nk_combo_end(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn contextual_begin(&mut self, panel: &mut NkPanel, flags: NkFlags, bounds: NkVec2, trigger_bounds: NkRect) -> bool {
    	unsafe {
    		nk_contextual_begin(&mut self.internal as *mut nk_context, &mut panel.internal, flags, bounds, trigger_bounds) > 0
    	}
    }
    
    pub fn contextual_item_label(&mut self, label: NkString, align: NkFlags) -> bool {
    	unsafe {
    		nk_contextual_item_label(&mut self.internal as *mut nk_context, label.as_ptr(), align) > 0
    	}
    }
    
    pub fn contextual_item_image_label(&mut self, img: NkImage, label: NkString, align: NkFlags) -> bool {
    	unsafe {
    		nk_contextual_item_image_label(&mut self.internal as *mut nk_context, img.internal, label.as_ptr(), align) > 0
    	}
    }
    
    pub fn contextual_item_symbol_label(&mut self, sym: NkSymbolType, label: NkString, align: NkFlags) -> bool {
    	unsafe {
    		nk_contextual_item_symbol_label(&mut self.internal as *mut nk_context, sym, label.as_ptr(), align) > 0
    	}
    }
    
    pub fn contextual_close(&mut self) {
    	unsafe {
    		nk_contextual_close(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn contextual_end(&mut self) {
    	unsafe {
    		nk_contextual_end(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn tooltip(&mut self, text: NkString) {
    	unsafe {
    		nk_tooltip(&mut self.internal as *mut nk_context, text.as_ptr());
    	}
    }
    
    pub fn tooltip_begin(&mut self, panel: &mut NkPanel, width: f32) -> bool {
    	unsafe {
    		nk_tooltip_begin(&mut self.internal as *mut nk_context, &mut panel.internal, width) > 0
    	}
    }
    
    pub fn tooltip_end(&mut self) {
    	unsafe {
    		nk_tooltip_end(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn menubar_begin(&mut self) {
    	unsafe {
    		nk_menubar_begin(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn menubar_end(&mut self) {
    	unsafe {
    		nk_menubar_end(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn menu_begin_label(&mut self, panel: &mut NkPanel, title: NkString, align: NkFlags, width: f32) -> bool {
    	unsafe {
    		nk_menu_begin_label(&mut self.internal as *mut nk_context, &mut panel.internal, title.as_ptr(), align, width) > 0
    	}
    }
    
    pub fn menu_begin_image(&mut self, panel: &mut NkPanel, title: NkString, img: NkImage, width: f32) -> bool {
    	unsafe {
    		nk_menu_begin_image(&mut self.internal as *mut nk_context, &mut panel.internal, title.as_ptr(), img.internal, width) > 0
    	}
    }
    
    pub fn menu_begin_image_label(&mut self, panel: &mut NkPanel, title: NkString, align: NkFlags, img: NkImage, width: f32) -> bool {
    	unsafe {
    		nk_menu_begin_image_label(&mut self.internal as *mut nk_context, &mut panel.internal, title.as_ptr(), align, img.internal, width) > 0
    	}
    }
    
    pub fn menu_begin_symbol(&mut self, panel: &mut NkPanel, title: NkString, sym: NkSymbolType, width: f32) -> bool {
    	unsafe {
    		nk_menu_begin_symbol(&mut self.internal as *mut nk_context, &mut panel.internal, title.as_ptr(), sym, width) > 0
    	}
    }
    
    pub fn menu_begin_symbol_label(&mut self, panel: &mut NkPanel, title: NkString, align: NkFlags, sym: NkSymbolType, width: f32) -> bool {
    	unsafe {
    		nk_menu_begin_symbol_label(&mut self.internal as *mut nk_context, &mut panel.internal, title.as_ptr(), align, sym, width) > 0
    	}
    }
    
    pub fn menu_item_label(&mut self, title: NkString, align: NkFlags) -> bool {
    	unsafe {
    		nk_menu_item_label(&mut self.internal as *mut nk_context, title.as_ptr(), align) > 0
    	}
    }
    
    pub fn menu_item_image_label(&mut self, img: NkImage, title: NkString, align: NkFlags) -> bool {
    	unsafe {
    		nk_menu_item_image_label(&mut self.internal as *mut nk_context, img.internal, title.as_ptr(), align) > 0
    	}
    }
    
    pub fn menu_item_symbol_label(&mut self, sym: NkSymbolType, title: NkString, align: NkFlags) -> bool {
    	unsafe {
    		nk_menu_item_symbol_label(&mut self.internal as *mut nk_context, sym, title.as_ptr(), align) > 0
    	}
    }
    
    pub fn menu_close(&mut self) {
    	unsafe {
    		nk_menu_close(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn menu_end(&mut self) {
    	unsafe {
    		nk_menu_end(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn convert(&mut self, cmds: &mut NkBuffer, vertices: &mut NkBuffer, elements: &mut NkBuffer, config: &NkConvertConfig) {
    	unsafe {
    		nk_convert(&mut self.internal as *mut nk_context, &mut cmds.internal as *mut nk_buffer, &mut vertices.internal as *mut nk_buffer, &mut elements.internal as *mut nk_buffer, &config.internal as *const nk_convert_config);
    	}
    }
    
    pub fn input_begin(&mut self) {
    	unsafe {
    		nk_input_begin(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn input_motion(&mut self, x: i32, y: i32) {
    	unsafe {
    		nk_input_motion(&mut self.internal as *mut nk_context, x, y);
    	}
    }
    
    pub fn input_key(&mut self, key: NkKey, down: bool) {
    	unsafe {
    		nk_input_key(&mut self.internal as *mut nk_context, key, if down { 1 } else { 0 });
    	}
    }
    
    pub fn input_button(&mut self, b: NkButton, x: i32, y: i32, down: bool) {
    	unsafe {
    		nk_input_button(&mut self.internal as *mut nk_context, b, x, y, if down { 1 } else { 0 });
    	}
    }
    
    pub fn input_scroll(&mut self, y: f32) {
    	unsafe {
    		nk_input_scroll(&mut self.internal as *mut nk_context, y);
    	}
    }
    
    pub fn input_char(&mut self, c: char) {
    	unsafe {
    		nk_input_char(&mut self.internal as *mut nk_context, c as i8);
    	}
    }
    
    pub fn input_glyph(&mut self, g: NkGlyph) {
    	unsafe {
    		nk_input_glyph(&mut self.internal as *mut nk_context, g);
    	}
    }
    
    pub fn input_unicode(&mut self, r: NkRune) {
    	unsafe {
    		nk_input_unicode(&mut self.internal as *mut nk_context, r);
    	}
    }
    
    pub fn input_end(&mut self) {
    	unsafe {
    		nk_input_end(&mut self.internal as *mut nk_context);
    	}
    }  
    
    pub fn style_default(&mut self) {
    	unsafe {
    		nk_style_default(&mut self.internal as *mut nk_context);
    	}
    }  
    
    pub fn style_from_table(&mut self, table: Option<&NkColorMap>) {
    	unsafe {
    		nk_style_from_table(&mut self.internal as *mut nk_context, match table {
	    		Some(map) => &map.internal[0] as *const nk_color,
	    		None => ::std::ptr::null(),
    		});
    	}
    }
    
    pub fn style_load_cursor(&mut self, cur: NkStyleCursor, res: &NkCursor) {
    	unsafe {
    		nk_style_load_cursor(&mut self.internal as *mut nk_context, cur, &res.internal as *const nk_cursor);
    	}
    }
    
    pub fn style_load_all_cursors(&mut self, table: &mut NkCursorMap) {
    	unsafe {
    		nk_style_load_all_cursors(&mut self.internal as *mut nk_context, &mut table.internal[0] as *mut nk_cursor);
    	}
    }
    
    pub fn style_set_font(&mut self, font: &NkUserFont) {
    	unsafe {
    		nk_style_set_font(&mut self.internal as *mut nk_context, font.internal);
    	}
    }
    
    pub fn style_set_cursor(&mut self, cur: NkStyleCursor) -> bool {
    	unsafe {
    		nk_style_set_cursor(&mut self.internal as *mut nk_context, cur) > 0
    	}
    }
    
    pub fn style_show_cursor(&mut self) {
    	unsafe {
    		nk_style_show_cursor(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn style_hide_cursor(&mut self) {
    	unsafe {
    		nk_style_hide_cursor(&mut self.internal as *mut nk_context);
    	}
    }
    
    pub fn style_push_font(&mut self, font: &mut NkUserFont) -> bool {
    	unsafe {
    		nk_style_push_font(&mut self.internal as *mut nk_context, font.internal) > 0
    	}
    }
    
    pub fn style_push_float(&mut self, addr: &mut f32, val: f32) -> bool {
    	unsafe {
    		nk_style_push_float(&mut self.internal as *mut nk_context, addr as *mut f32, val) > 0
    	}
    }
    
    pub fn style_push_vec2(&mut self, addr: &mut NkVec2, val: NkVec2) -> bool {
    	unsafe {
    		nk_style_push_vec2(&mut self.internal as *mut nk_context, addr as *mut nk_vec2, val) > 0
    	}
    }
    
    pub fn style_push_style_item(&mut self, addr: &mut NkStyleItem, val: NkStyleItem) -> bool {
    	unsafe {
    		nk_style_push_style_item(&mut self.internal as *mut nk_context, &mut addr.internal as *mut nk_style_item, val.internal) > 0
    	}
    }
    
    pub fn style_push_flags(&mut self, addr: &mut NkFlags, val: NkFlags) -> bool {
    	unsafe {
    		nk_style_push_flags(&mut self.internal as *mut nk_context, addr as *mut nk_flags, val) > 0
    	}
    }
    
    pub fn style_push_color(&mut self, addr: &mut NkColor, val: NkColor) -> bool {
    	unsafe {
    		nk_style_push_color(&mut self.internal as *mut nk_context, addr as *mut nk_color, val) > 0
    	}
    }
    
    pub fn style_pop_font(&mut self) -> bool {
    	unsafe {
    		nk_style_pop_font(&mut self.internal as *mut nk_context) > 0
    	}
    }
    
    pub fn style_pop_float(&mut self) -> bool {
    	unsafe {
    		nk_style_pop_float(&mut self.internal as *mut nk_context) > 0
    	}
    }
    
    pub fn style_pop_vec2(&mut self) -> bool {
    	unsafe {
    		nk_style_pop_vec2(&mut self.internal as *mut nk_context) > 0
    	}
    }
    
    pub fn style_pop_style_item(&mut self) -> bool {
    	unsafe {
    		nk_style_pop_style_item(&mut self.internal as *mut nk_context) > 0
    	}
    }
    
    pub fn style_pop_flags(&mut self) -> bool {
    	unsafe {
    		nk_style_pop_flags(&mut self.internal as *mut nk_context) > 0
    	}
    }
    
    pub fn style_pop_color(&mut self) -> bool {
    	unsafe {
    		nk_style_pop_color(&mut self.internal as *mut nk_context) > 0
    	}
    }
    
    pub fn widget_bounds(&mut self) -> NkRect {
    	unsafe {
    		nk_widget_bounds(&mut self.internal as *mut nk_context)
    	}
    }
    
    pub fn widget_position(&mut self) -> NkVec2 {
    	unsafe {
    		nk_widget_position(&mut self.internal as *mut nk_context)
    	}
    }
    
    pub fn widget_size(&mut self) -> NkVec2 {
    	unsafe {
    		nk_widget_size(&mut self.internal as *mut nk_context)
    	}
    }
    
    pub fn widget_is_hovered(&mut self) -> bool {
    	unsafe {
    		nk_widget_is_hovered(&mut self.internal as *mut nk_context) > 0
    	}
    }
    
    pub fn widget_is_mouse_clicked(&mut self, b: NkButton) -> bool {
    	unsafe {
    		nk_widget_is_mouse_clicked(&mut self.internal as *mut nk_context, b) > 0
    	}
    }
    
    pub fn widget_has_mouse_click_down(&mut self, b: NkButton, down: bool) -> bool {
    	unsafe {
    		nk_widget_has_mouse_click_down(&mut self.internal as *mut nk_context, b, if down { 1 } else { 0 }) > 0
    	}
    }
    
    pub fn widget(&self, arg1: &mut NkRect) -> NkWidgetLayoutState {
    	unsafe {
    		nk_widget(arg1, &self.internal as *const nk_context)
    	}
    }
    
    pub fn spacing(&mut self, cols: i32) {
    	unsafe {
    		nk_spacing(&mut self.internal as *mut nk_context, cols);
    	}
    }
    
    pub fn draw_begin(&self, buf: &NkBuffer) -> Option<NkDrawCommand> {
    	let n = unsafe {
    		nk__draw_begin(&self.internal as *const nk_context, &buf.internal as *const nk_buffer)
    	};
    	
    	if n.is_null() {
    		None
    	} else {
    		Some(NkDrawCommand { internal: n })
    	}
    }
    pub fn draw_next(&self, prev: &NkDrawCommand, buf: &NkBuffer) -> Option<NkDrawCommand> {
    	let n = unsafe {
    		nk__draw_next(prev.internal as *const nk_draw_command, &buf.internal as *const nk_buffer, &self.internal as *const nk_context)
    	};
    	
    	if n.is_null() {
    		None
    	} else {
    		Some(NkDrawCommand { internal: n })
    	}
    }
    
    pub fn next_cmd(&mut self, arg2: &NkCommand) -> Option<NkCommand> {
    	let r = unsafe { nk__next(&mut self.internal as *mut nk_context, arg2.internal) };
    	if r.is_null() {
    		None
    	} else {
    		Some(NkCommand::new(r))
    	}
    }
    
    pub fn begin_cmd(&mut self) -> Option<NkCommand> {
    	let r = unsafe { nk__begin(&mut self.internal as *mut nk_context) };
    	if r.is_null() {
    		None
    	} else {
    		Some(NkCommand::new(r))
    	}
    }
    
    pub fn draw_command_iterator<'a>(&'a mut self, buf: &'a mut NkBuffer) -> NkDrawCommandIterator<'a> {
    	NkDrawCommandIterator {
    		ctx: self,
    		buf: buf,
    	}
    }
    
    pub fn command_iterator<'a>(&'a mut self) -> NkCommandIterator<'a> {
    	NkCommandIterator {
    		ctx: self,
    	}
    }
}

//============================================================================================

pub struct NkCommandIterator<'a> {
	ctx: &'a mut NkContext,
}

impl <'a> IntoIterator for NkCommandIterator<'a> {
    type Item = NkCommand;
    type IntoIter = NkCommandIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
    	let cmd = self.ctx.begin_cmd();
        NkCommandIntoIter { 
        	ctx: self.ctx,
        	cmd: cmd,
        }
    }
}

pub struct NkCommandIntoIter<'a> {
    ctx: &'a mut NkContext,
    cmd: Option<NkCommand>,
}

impl <'a> Iterator for NkCommandIntoIter<'a> {
    type Item = NkCommand;
    fn next(&mut self) -> Option<NkCommand> {
    	let r = self.cmd.clone();
        
        self.cmd = if let Some(ref p) = self.cmd {
        	self.ctx.next_cmd(p)
        } else {
	        None
        };
        
        r
    }
}

//============================================================================================

pub struct NkDrawCommandIterator<'a> {
	ctx: &'a mut NkContext,
	buf: &'a mut NkBuffer,
}

impl <'a> IntoIterator for NkDrawCommandIterator<'a> {
    type Item = NkDrawCommand;
    type IntoIter = NkDrawCommandIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
    	let cmd = self.ctx.draw_begin(self.buf);
        NkDrawCommandIntoIter { 
        	ctx: self.ctx,
        	buf: self.buf,
        	cmd: cmd,
        }
    }
}

pub struct NkDrawCommandIntoIter<'a> {
    ctx: &'a mut NkContext,
    buf: &'a mut NkBuffer,
    cmd: Option<NkDrawCommand>,
}

impl <'a> Iterator for NkDrawCommandIntoIter<'a> {
    type Item = NkDrawCommand;
    fn next(&mut self) -> Option<NkDrawCommand> {
    	let r = self.cmd.clone();
        
        self.cmd = if let Some(ref p) = self.cmd {
        	self.ctx.draw_next(p, self.buf)
        } else {
	        None
        };
        
        r
    }
}

//=============================================================================================

#[allow(dead_code)]
pub struct NkWindow {
	internal: *mut nk_window,
}

impl NkWindow {
	fn new(w: *mut nk_window) -> NkWindow {
		NkWindow {
			internal: w,
		}
	}
}

//=============================================================================================

pub struct NkPanel {
	internal: nk_panel,
}

impl Default for NkPanel {
	fn default() -> Self {
		NkPanel {
			internal: nk_panel::default(),
		}
	}
}

impl NkPanel {
	pub fn bounds(&self) -> NkRect {
		self.internal.bounds
	}
}

//=============================================================================================

pub struct NkPanelRef {
	internal: *mut nk_panel,
}

impl NkPanelRef {
	fn new(p: *mut nk_panel) -> NkPanelRef {
		NkPanelRef {
			internal: p
		}
	}
	
	pub fn bounds(&self) -> NkRect {
		unsafe { (*self.internal).bounds }
	}
}

//=============================================================================================

#[derive(Clone)]
pub struct NkCommand {
	internal: *const nk_command,
}

impl NkCommand {
	fn new(i: *const nk_command) -> NkCommand {
		NkCommand {
			internal: i,
		}
	}
}

impl Debug for NkCommand {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        unsafe { (*self.internal).fmt(f) }
    }
}

//=============================================================================================

pub struct NkCommandBuffer {
	internal: *mut nk_command_buffer,
}

impl NkCommandBuffer {
	fn new(b: *mut nk_command_buffer) -> NkCommandBuffer {
		NkCommandBuffer {
			internal: b
		}
	}
	
	pub fn stroke_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, line_thickness: f32, color: NkColor) {
		unsafe {
			nk_stroke_line(self.internal, x0, y0, x1, y1, line_thickness, color);
		}
	}
	
	pub fn stroke_curve(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, line_thickness: f32, color: NkColor) {
		unsafe {
			nk_stroke_curve(self.internal, x0, y0, x1, y1, x2, y2, x3, y3, line_thickness, color);
		}
	}
	
	pub fn stroke_rect(&mut self, bounds: NkRect, rounding: f32, line_thickness: f32, color: NkColor) {
		unsafe {
			nk_stroke_rect(self.internal, bounds, rounding, line_thickness, color);
		}
	}
	
	pub fn stroke_circle(&mut self, arg2: NkRect, line_thickness: f32, color: NkColor) {
		unsafe {
			nk_stroke_circle(self.internal, arg2, line_thickness, color);
		}
	}
	
	pub fn stroke_arc(&mut self, cx: f32, cy: f32, radius: f32, a_min: f32, a_max: f32, line_thickness: f32, color: NkColor) {
		unsafe {
			nk_stroke_arc(self.internal, cx, cy, radius, a_min, a_max, line_thickness, color);
		}
	}
	
	pub fn stroke_triangle(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, line_thichness: f32, color: NkColor) {
		unsafe {
			nk_stroke_triangle(self.internal, x0, y0, x1, y1, x2, y2, line_thichness, color);
		}
	}
	
	pub fn stroke_polyline(&mut self, points: &mut [f32], line_thickness: f32, color: NkColor) {
		unsafe {
			nk_stroke_polyline(self.internal, &mut points[0] as *mut f32, points.len() as ::std::os::raw::c_int, line_thickness, color);
		}
	}
	
	pub fn stroke_polygon(&mut self, points: &mut [f32], line_thickness: f32, color: NkColor) {
		unsafe {
			nk_stroke_polygon(self.internal, &mut points[0] as *mut f32, points.len() as ::std::os::raw::c_int, line_thickness, color);
		}
	}
	
	pub fn fill_rect(&mut self, arg2: NkRect, rounding: f32, color: NkColor) {
		unsafe {
			nk_fill_rect(self.internal, arg2, rounding, color);
		}
	}
	
	pub fn fill_rect_multi_color(&mut self, arg2: NkRect, left: NkColor, top: NkColor, right: NkColor, bottom: NkColor) {
		unsafe {
			nk_fill_rect_multi_color(self.internal, arg2, left, top, right, bottom);
		}
	}
	
	pub fn fill_circle(&mut self, arg2: NkRect, color: NkColor) {
		unsafe {
			nk_fill_circle(self.internal, arg2, color);
		}
	}
	
	pub fn fill_arc(&mut self, cx: f32, cy: f32, radius: f32, a_min: f32, a_max: f32, color: NkColor) {
		unsafe {
			nk_fill_arc(self.internal, cx, cy, radius, a_min, a_max, color);    
		}
	}
	
	pub fn fill_triangle(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, color: NkColor) {
		unsafe {
			nk_fill_triangle(self.internal, x0, y0, x1, y1, x2, y2, color);    
		}
	}
    
    pub fn fill_polygon(&mut self, points: &mut [f32], color: NkColor) {
    	unsafe {
    		nk_fill_polygon(self.internal, &mut points[0] as *mut f32, points.len() as ::std::os::raw::c_int, color);
    	}
    }
    
    pub fn push_scissor(&mut self, arg2: NkRect) {
    	unsafe {
    		nk_push_scissor(self.internal, arg2);
    	}
    }
    
    pub fn draw_image(&mut self, arg2: NkRect, arg3: &NkImage, arg4: NkColor) {
    	unsafe {
    		nk_draw_image(self.internal, arg2, &arg3.internal as *const nk_image, arg4);
    	}
    }
    
    pub fn draw_text(&mut self, arg2: NkRect, text: &str, arg3: &NkUserFont, arg4: NkColor, arg5: NkColor) {
    	unsafe {
    		nk_draw_text(self.internal, arg2, text.as_ptr() as *const i8, text.as_bytes().len() as ::std::os::raw::c_int, arg3.internal as *const nk_user_font, arg4, arg5);
    	}
    }
}

//=============================================================================================

pub fn color_rgb(r: i32, g: i32, b: i32) -> NkColor {
	unsafe {
		nk_rgb(r,g,b)
	}
}

pub fn color_rgb_iv(rgb: &i32) -> NkColor {
	unsafe {
		nk_rgb_iv(rgb as *const i32)
	}
}

pub fn color_rgb_bv(rgb: &u8) -> NkColor {
	 unsafe {
		nk_rgb_bv(rgb as *const u8)
	}
}

pub fn color_rgb_fv(rgb: &f32) -> NkColor {
	unsafe {
		nk_rgb_fv(rgb as *const f32)
	}
}

pub fn color_rgb_f(r: f32, g: f32, b: f32) -> NkColor {
	unsafe {
		nk_rgb_f(r,g,b)
	}
}

pub fn color_rgb_hex(rgb: NkString) -> NkColor {
	unsafe {
		nk_rgb_hex(rgb.as_ptr())
	}
}

pub fn color_rgba(r: i32, g: i32, b: i32, a: i32) -> NkColor {
	unsafe {
		nk_rgba(r,g,b,a)
	}
}

pub fn color_rgba_u32(rgba: u32) -> NkColor {
	unsafe {
		nk_rgba_u32(rgba)
	}
}


pub fn color_rgba_iv(rgba: &i32) -> NkColor {
	unsafe {
		nk_rgba_iv(rgba as *const i32)
	}
}

pub fn color_rgba_bv(rgb: &u8) -> NkColor {
	unsafe {
		nk_rgba_bv(rgb as *const u8)
	}
}

pub fn color_rgba_fv(rgb: &f32) -> NkColor {
	unsafe {
		nk_rgba_fv(rgb as *const f32)
	}
}

pub fn color_rgba_f(r: f32, g: f32, b: f32, a: f32) -> NkColor {
	unsafe {
		nk_rgba_f(r,g,b,a)
	}
}

pub fn color_rgba_hex(rgba: NkString) -> NkColor {
	unsafe {
		nk_rgba_hex(rgba.as_ptr())
	}
}

pub fn color_hsv(h: i32, s: i32, v: i32) -> NkColor {
	unsafe {
		nk_hsv(h,s,v)
	}
}

pub fn color_hsv_iv(hsv: &i32) -> NkColor {
	unsafe {
		nk_hsv_iv(hsv as *const i32)
	}
}

pub fn color_hsv_bv(hsv: &u8) -> NkColor {
	unsafe {
		nk_hsv_bv(hsv as *const u8)
	}
}

pub fn color_hsv_fv(hsv: &f32) -> NkColor {
	unsafe {
		nk_hsv_fv(hsv as *const f32)
	}
}

pub fn color_hsv_f(h: f32, s: f32, v: f32) -> NkColor {
	unsafe {
		nk_hsv_f(h,s,v)
	}
}

pub fn color_hsva(h: i32, s: i32, v: i32, a: i32) -> NkColor {
	unsafe {
		nk_hsva(h,s,v,a)
	}
}

pub fn color_hsva_iv(hsva: &i32) -> NkColor {
	unsafe {
		nk_hsva_iv(hsva as *const i32)
	}
}

pub fn color_hsva_bv(hsv: &u8) -> NkColor {
	unsafe {
		nk_hsva_bv(hsv as *const u8)
	}
}

pub fn color_hsva_fv(hsv: &f32) -> NkColor {
	unsafe {
		nk_hsva_fv(hsv as *const f32)
	}
}

pub fn color_hsva_f(h: f32, s: f32, v: f32, a: f32) -> NkColor {
	unsafe {
		nk_hsva_f(h,s,v,a)
	}
}

pub fn style_get_color_by_name(c: NkStyleColor) -> Cow<'static, str> {
	unsafe {
		//NkString::from_bytes_unchecked()
		//CString::from_raw(nk_style_get_color_by_name(c))
		::std::ffi::CStr::from_ptr(nk_style_get_color_by_name(c)).to_string_lossy()
	}
}

//=============================================================================================

#[derive(Clone, Debug)]
pub struct NkImage {
	internal: nk_image,
}

impl NkImage {
	pub fn with_id(id: i32) -> NkImage {
		NkImage {
			internal: unsafe {
				nk_image_id(id)
			}
		}
	}
	
	pub fn with_ptr(ptr: *mut c_void) -> NkImage {
		NkImage {
			internal: unsafe {
				nk_image_ptr(ptr)
			}
		}
	}
	
	pub fn id(&mut self) -> i32 {
		unsafe { *(self.internal.handle.id()) }
	}
	
	pub fn ptr(&mut self) -> *mut c_void {
		unsafe { *(self.internal.handle.ptr()) }
	}
}

//=============================================================================================

#[derive(Clone, Debug)]
pub struct NkFontGlyph {
	internal: *const nk_font_glyph,
}

/*impl Default for NkFontGlyph {
	fn default() -> Self {
		NkFontGlyph {
			internal: nk_font_glyph::default()
		}
	}
}*/

impl NkFontGlyph {
	fn new(g: *const nk_font_glyph) -> NkFontGlyph {
		NkFontGlyph {
			internal: g
		}
	}
	
	pub fn get_codepoint(&self) -> char {
		unsafe { 
			::std::char::from_u32((&*self.internal).codepoint).unwrap()
		}
	}
    pub fn get_xadvance(&self) -> f32 {
    	unsafe { (&*self.internal).xadvance }
    }
    pub fn x0(&self) -> f32 {
    	unsafe { (&*self.internal).x0 }
    }
    pub fn y0(&self) -> f32 {
    	unsafe { (&*self.internal).y0 }
    }
    pub fn x1(&self) -> f32 {
    	unsafe { (&*self.internal).x1 }
    }
    pub fn y1(&self) -> f32 {
    	unsafe { (&*self.internal).y1 }
    }
    pub fn w(&self) -> f32 {
    	unsafe { (&*self.internal).w }
    }
    pub fn h(&self) -> f32 {
    	unsafe { (&*self.internal).h }
    }
    pub fn u0(&self) -> f32 {
    	unsafe { (&*self.internal).u0 }
    }
    pub fn v0(&self) -> f32 {
    	unsafe { (&*self.internal).v0 }
    }
    pub fn u1(&self) -> f32 {
    	unsafe { (&*self.internal).u1 }
    }
    pub fn v1(&self) -> f32 {
    	unsafe { (&*self.internal).v1 }
    }
}

//=============================================================================================

#[derive(Clone, Debug)]
pub struct NkFont {
	internal: *mut nk_font
}

impl NkFont {
	fn new(font: *mut nk_font) -> NkFont {
		NkFont {
			internal: font
		}
	}
	
	pub fn find_glyph(&mut self, unicode: char) -> NkFontGlyph {
		NkFontGlyph::new(unsafe {
			nk_font_find_glyph(self.internal, unicode as u32)
		})
	}
	
	pub fn handle(&mut self) -> NkUserFont {
		let f: *mut nk_user_font = unsafe { &mut (&mut *self.internal).handle };
		NkUserFont::new(f)
	}
}

//=============================================================================================

#[derive(Clone, Debug)]
pub struct NkUserFont {
	internal: *mut nk_user_font,
}

impl NkUserFont {
	fn new(f: *mut nk_user_font) -> NkUserFont {
		NkUserFont {
			internal: f
		}
	}
}

//=============================================================================================

/*
    pub fn nk_font_default_glyph_ranges() -> *const nk_rune;
    pub fn nk_font_chinese_glyph_ranges() -> *const nk_rune;
    pub fn nk_font_cyrillic_glyph_ranges() -> *const nk_rune;
    pub fn nk_font_korean_glyph_ranges() -> *const nk_rune;
*/

//=============================================================================================

const ALIGNMENT: usize = 16;
use alloc::heap;
use std::mem;

unsafe extern "C" fn alloc_rust(_: nk_handle, _: *mut c_void, size: nk_size) -> *mut c_void {
	trace!("allocating {} bytes", size);
		
    let size_size = mem::size_of::<nk_size>();
    let size = size + size_size;

    /*let memory = if old.is_null() {
        trace!("allocating {} / {} bytes", size_size, size);
		heap::allocate(size, ALIGNMENT)
    } else {
        trace!("reallocating {} / {} bytes", size_size, size);
		let old = old as *mut u8;
        let old = old.offset(-(size_size as isize));
        let old_size = *(old as *const nk_size);
        heap::reallocate(old, old_size, size, ALIGNMENT)
    };*/
    
    let memory = heap::allocate(size, ALIGNMENT);
    trace!("allocating {} / {} bytes", size_size, size);		
    
    *(memory as *mut nk_size) = size;
    trace!("allocated {} bytes at {:p}", size, memory);
    memory.offset(size_size as isize) as *mut c_void
}

unsafe extern "C" fn free_rust(_: nk_handle, old: *mut c_void) {
    if old.is_null() {
    	trace!("no dealloc for empty pointer");
    	return;
    }

    let size_size = mem::size_of::<nk_size>();
    
    let old = old as *mut u8;
    let old = old.offset(-(size_size as isize));
    let old_size = *(old as *const nk_size);

    trace!("deallocating {} bytes from {:p}", old_size, old);
	
	heap::deallocate(old as *mut u8, old_size, ALIGNMENT);
}

unsafe extern "C" fn alloc_rust_hacky(_: nk_handle, _: *mut c_void, size: nk_size) -> *mut c_void {
	/*if old.is_null() {
		free_rust_hacky(hnd, old);
	}*/
	
	trace!("allocating {} bytes", size);
	let size_size = mem::size_of::<nk_size>();
    let size = size + size_size;

    trace!("allocating {} / {} bytes", size_size, size);
	
	let mut v: Vec<u8> = Vec::with_capacity(size);
    
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);
    
    *(ptr as *mut nk_size) = size;
    ptr.offset(size_size as isize) as *mut c_void
}

unsafe extern "C" fn free_rust_hacky(_: nk_handle, old: *mut c_void) {
    if old.is_null() {
    	trace!("no dealloc for empty pointer");
    	return;
    }

    let size_size = mem::size_of::<nk_size>();

    let old = old as *mut u8;
    let old = old.offset(-(size_size as isize));
    let old_size = *(old as *const nk_size);
    
    if old_size > 16_000_000_000 {
    	trace!("Invalid dealloc size {}", old_size);
    	return;
    }

    trace!("deallocating {} bytes", old_size);
	
	std::mem::drop(Vec::from_raw_parts(old, 0, old_size));
}

#[cfg(test)]
mod tests {
	use super::*;
	use nuklear_sys::*;
	
    #[test]
    fn test_alloc_dealloc() {
    	let mut allo = NkAllocator::new_heap();
    	let mut h = nk_handle::default();
    	
    	unsafe {
    		trace!("allocating 100500");
	    	let mut mem = allo.internal.alloc.unwrap()(h, ::std::ptr::null_mut(), 100500);
	    	trace!("freeing 100500");
	    	allo.internal.free.unwrap()(h, mem);
    	}
    }
    
    #[test]
    fn it_works() {
    	trace!("size {}", ((::std::mem::size_of::<nk_window>() / ::std::mem::size_of::<nk_uint>()) / 2));
    }
}