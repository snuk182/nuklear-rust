#![cfg_attr(feature = "rust_allocator", feature(allocator_api, alloc, heap_api))]

#[macro_use]
extern crate log;
pub extern crate nuklear_sys;

#[cfg(feature = "rust_allocator")]
mod alloc_heap;
mod alloc_vec;

use std::default::Default;
use std::os::raw::*;
use std::borrow::Cow;

use nuklear_sys::*;

//pub use nuklear_sys;
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
pub use nuklear_sys::nk_style_colors as NkStyleColor;
pub use nuklear_sys::nk_style_cursor as NkStyleCursor;
pub use nuklear_sys::nk_widget_layout_states as NkWidgetLayoutState;
pub use nuklear_sys::nk_draw_list_stroke as NkDrawListStroke;
pub use nuklear_sys::nk_anti_aliasing as NkAntiAliasing;
pub use nuklear_sys::nk_allocation_type as NkAllocationType;
pub use nuklear_sys::nk_draw_vertex_layout_attribute as NkDrawVertexLayoutAttribute;
pub use nuklear_sys::nk_draw_vertex_layout_format as NkDrawVertexLayoutFormat;
pub use nuklear_sys::nk_edit_types as NkEditType;
pub use nuklear_sys::nk_font_coord_type as NkFontCoordType;
pub use nuklear_sys::nk_style_header_align as NkStyleHeaderAlign;
pub use nuklear_sys::nk_panel_type as NkPanelType;
pub use nuklear_sys::nk_panel_row_layout_type as NkPanelRowLayoutType;
pub use nuklear_sys::nk_command_type as NkCommandType;

pub use nuklear_sys::nk_panel_flags as NkPanelFlags;
pub use nuklear_sys::nk_text_alignment as NkTextAlignment;

pub use nuklear_sys::nk_vec2 as NkVec2;
pub use nuklear_sys::nk_vec2i as NkVec2i;
pub use nuklear_sys::nk_scroll as NkScroll;
pub use nuklear_sys::nk_color as NkColor;
pub use nuklear_sys::nk_rect as NkRect;
pub use nuklear_sys::nk_recti as NkRecti;
pub use nuklear_sys::nk_style_text as NkStyleText;
pub use nuklear_sys::nk_menu_state as NkMenuState;
pub use nuklear_sys::nk_chart_slot as NkChartSlot;
pub use nuklear_sys::nk_popup_buffer as NkPopupBuffer;

pub use nuklear_sys::nk_glyph as NkGlyph;

pub use nuklear_sys::nk_plugin_filter as NkPluginFilter;

pub const NK_FILTER_DEFAULT: NkPluginFilter = Some(nk_filter_default);
pub const NK_FILTER_ASCII: NkPluginFilter = Some(nk_filter_ascii);
pub const NK_FILTER_FLOAT: NkPluginFilter = Some(nk_filter_float);
pub const NK_FILTER_DECIMAL: NkPluginFilter = Some(nk_filter_decimal);
pub const NK_FILTER_HEX: NkPluginFilter = Some(nk_filter_hex);
pub const NK_FILTER_OCT: NkPluginFilter = Some(nk_filter_oct);
pub const NK_FILTER_BINARY: NkPluginFilter = Some(nk_filter_binary);

pub const ALIGNMENT: usize = 16;

macro_rules! wrapper_impls {
	($name: ident, $typ: ty) => {
		impl AsRef<$typ> for $name {
		    fn as_ref(&self) -> &$typ {
		        &self.internal
		    }
		}
		impl AsMut<$typ> for $name {
		    fn as_mut(&mut self) -> &mut $typ {
		        &mut self.internal
		    }
		}
		impl AsRef<$name> for $typ {
		    fn as_ref(&self) -> &$name {
		        unsafe { ::std::mem::transmute(self) }
		    }
		}
		impl AsMut<$name> for $typ {
		    fn as_mut(&mut self) -> &mut $name {
		        unsafe { ::std::mem::transmute(self) }
		    }
		}
		
		impl Default for $name {
			fn default() -> Self {
				$name {
					internal: unsafe { ::std::mem::zeroed() },
				}
			}
		}		
	}
}

macro_rules! wrapper_type {
	($name: ident, $typ: ty) => {
		#[derive(Clone,Debug)]
		#[repr(C)]
		pub struct $name {
			internal: $typ
		}
		
		wrapper_impls!($name, $typ);
	}
}

macro_rules! wrapper_type_no_clone {
	($name: ident, $typ: ty) => {
		#[repr(C)]
		pub struct $name {
			internal: $typ
		}
		
		wrapper_impls!($name, $typ);
	}
}

// ==========================================================================================================

unsafe extern "C" fn nk_filter_custom(arg1: *const nk_text_edit, unicode: nk_rune) -> ::std::os::raw::c_int {
    if let Some(f) = CUSTOM_EDIT_FILTER {
        if f(::std::mem::transmute(arg1),
             ::std::char::from_u32_unchecked(unicode)) {
            1
        } else {
            0
        }
    } else {
        1
    }
}

static mut CUSTOM_EDIT_FILTER: Option<fn(&NkTextEdit, char) -> bool> = None;

// ===========================================================================================================

// unsafe extern "C" fn nk_plot_value_getter_custom(user: *mut ::std::os::raw::c_void, index: ::std::os::raw::c_int) -> f32 {
// let f = user as *const _ as &[f32];
// f[index as usize]
// }

// ===========================================================================================================

#[derive(Clone)]
pub struct NkString<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> NkString<'a> {
    pub unsafe fn from_bytes_unchecked(bytes: &'a [u8]) -> NkString<'a> {
        NkString { bytes: Cow::Borrowed(bytes) }
    }
    pub fn as_ptr(&self) -> *const c_char {
        self.bytes.as_ptr() as *const c_char
    }

    // pub fn nk_str_init(arg1: *mut nk_str, arg2: *const nk_allocator,
    // size: nk_size);
    // pub fn nk_str_init_fixed(arg1: *mut nk_str,
    // memory: *mut ::std::os::raw::c_void,
    // size: nk_size);
    // pub fn nk_str_clear(arg1: *mut nk_str);
    // pub fn nk_str_free(arg1: *mut nk_str);
    // pub fn nk_str_append_text_char(arg1: *mut nk_str,
    // arg2: *const ::std::os::raw::c_char,
    // arg3: ::std::os::raw::c_int)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_append_str_char(arg1: *mut nk_str,
    // arg2: *const ::std::os::raw::c_char)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_append_text_utf8(arg1: *mut nk_str,
    // arg2: *const ::std::os::raw::c_char,
    // arg3: ::std::os::raw::c_int)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_append_str_utf8(arg1: *mut nk_str,
    // arg2: *const ::std::os::raw::c_char)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_append_text_runes(arg1: *mut nk_str, arg2: *const nk_rune,
    // arg3: ::std::os::raw::c_int)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_append_str_runes(arg1: *mut nk_str, arg2: *const nk_rune)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_insert_at_char(arg1: *mut nk_str,
    // pos: ::std::os::raw::c_int,
    // arg2: *const ::std::os::raw::c_char,
    // arg3: ::std::os::raw::c_int)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_insert_at_rune(arg1: *mut nk_str,
    // pos: ::std::os::raw::c_int,
    // arg2: *const ::std::os::raw::c_char,
    // arg3: ::std::os::raw::c_int)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_insert_text_char(arg1: *mut nk_str,
    // pos: ::std::os::raw::c_int,
    // arg2: *const ::std::os::raw::c_char,
    // arg3: ::std::os::raw::c_int)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_insert_str_char(arg1: *mut nk_str,
    // pos: ::std::os::raw::c_int,
    // arg2: *const ::std::os::raw::c_char)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_insert_text_utf8(arg1: *mut nk_str,
    // pos: ::std::os::raw::c_int,
    // arg2: *const ::std::os::raw::c_char,
    // arg3: ::std::os::raw::c_int)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_insert_str_utf8(arg1: *mut nk_str,
    // pos: ::std::os::raw::c_int,
    // arg2: *const ::std::os::raw::c_char)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_insert_text_runes(arg1: *mut nk_str,
    // pos: ::std::os::raw::c_int,
    // arg2: *const nk_rune,
    // arg3: ::std::os::raw::c_int)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_insert_str_runes(arg1: *mut nk_str,
    // pos: ::std::os::raw::c_int,
    // arg2: *const nk_rune)
    // -> ::std::os::raw::c_int;
    // pub fn nk_str_remove_chars(arg1: *mut nk_str, len: ::std::os::raw::c_int);
    // pub fn nk_str_remove_runes(str: *mut nk_str, len: ::std::os::raw::c_int);
    // pub fn nk_str_delete_chars(arg1: *mut nk_str, pos: ::std::os::raw::c_int,
    // len: ::std::os::raw::c_int);
    // pub fn nk_str_delete_runes(arg1: *mut nk_str, pos: ::std::os::raw::c_int,
    // len: ::std::os::raw::c_int);
    // pub fn nk_str_at_char(arg1: *mut nk_str, pos: ::std::os::raw::c_int)
    // -> *mut ::std::os::raw::c_char;
    // pub fn nk_str_at_rune(arg1: *mut nk_str, pos: ::std::os::raw::c_int,
    // unicode: *mut nk_rune,
    // len: *mut ::std::os::raw::c_int)
    // -> *mut ::std::os::raw::c_char;
    // pub fn nk_str_rune_at(arg1: *const nk_str, pos: ::std::os::raw::c_int)
    // -> nk_rune;
    // pub fn nk_str_at_char_const(arg1: *const nk_str,
    // pos: ::std::os::raw::c_int)
    // -> *const ::std::os::raw::c_char;
    // pub fn nk_str_at_const(arg1: *const nk_str, pos: ::std::os::raw::c_int,
    // unicode: *mut nk_rune,
    // len: *mut ::std::os::raw::c_int)
    // -> *const ::std::os::raw::c_char;
    // pub fn nk_str_get(arg1: *mut nk_str) -> *mut ::std::os::raw::c_char;
    // pub fn nk_str_get_const(arg1: *const nk_str)
    // -> *const ::std::os::raw::c_char;
    // pub fn nk_str_len(arg1: *mut nk_str) -> ::std::os::raw::c_int;
    // pub fn nk_str_len_char(arg1: *mut nk_str) -> ::std::os::raw::c_int;
    //
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

// ======================================================================================

#[derive(Clone)]
pub struct NkStringArray<'a> {
    arr: Vec<NkString<'a>>,
    ptrs: Vec<*const c_char>,
}

impl<'a> NkStringArray<'a> {
    pub fn as_ptr(&self) -> *const *const c_char {
        self.ptrs.as_slice() as *const _ as *const *const c_char
    }
    pub fn as_mut(&mut self) -> *mut *const c_char {
        self.ptrs.as_mut_slice() as *mut _ as *mut *const c_char
    }
    pub fn len(&self) -> usize {
        self.ptrs.len()
    }
}

impl<'a> From<&'a [&'a str]> for NkStringArray<'a> {
    fn from(value: &[&'a str]) -> NkStringArray<'a> {
        let mut r = NkStringArray {
            arr: Vec::with_capacity(value.len()),
            ptrs: Vec::with_capacity(value.len()),
        };

        for s in value {
            r.arr.push(NkString::from(*s));
            r.ptrs.push(r.arr[r.arr.len() - 1].as_ptr());
        }

        r
    }
}

// ======================================================================================

#[derive(Debug, Clone, PartialEq, Copy)]
enum NkHandleKind {
    Empty,
    Ptr,
    Id,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NkHandle {
    internal: nk_handle,
    kind: NkHandleKind,
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
    pub fn id(&mut self) -> Option<i32> {
        match self.kind {
            NkHandleKind::Id | NkHandleKind::Unknown => Some(unsafe { *(self.internal.id.as_ref()) }),
            _ => None,
        }
    }

    pub fn ptr(&mut self) -> Option<*mut c_void> {
        match self.kind {
            NkHandleKind::Ptr | NkHandleKind::Unknown => Some(unsafe { *(self.internal.ptr.as_mut()) }),
            _ => None,
        }
    }

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

// ==================================================================================

wrapper_type!(NkInput, nk_input);

impl NkInput {
    pub fn mouse(&self) -> NkMouse {
        NkMouse { internal: self.internal.mouse }
    }

    pub fn has_mouse_click(&self, b: NkButton) -> bool {
        unsafe { nk_input_has_mouse_click(&self.internal, b) != 0 }
    }

    pub fn has_mouse_click_in_rect(&self, b: NkButton, rect: NkRect) -> bool {
        unsafe { nk_input_has_mouse_click_in_rect(&self.internal, b, rect) != 0 }
    }

    pub fn has_mouse_click_down_in_rect(&self, b: NkButton, rect: NkRect, down: bool) -> bool {
        unsafe { nk_input_has_mouse_click_down_in_rect(&self.internal, b, rect, if down { 1 } else { 0 }) != 0 }
    }

    pub fn is_mouse_click_in_rect(&self, b: NkButton, rect: NkRect) -> bool {
        unsafe { nk_input_is_mouse_click_in_rect(&self.internal, b, rect) != 0 }
    }

    pub fn is_mouse_click_down_in_rect(&self, b: NkButton, rect: NkRect, down: bool) -> bool {
        unsafe { nk_input_is_mouse_click_down_in_rect(&self.internal, b, rect, down as ::std::os::raw::c_int) != 0 }
    }

    pub fn any_mouse_click_in_rect(&self, rect: NkRect) -> bool {
        unsafe { nk_input_any_mouse_click_in_rect(&self.internal, rect) != 0 }
    }

    pub fn is_mouse_prev_hovering_rect(&self, rect: NkRect) -> bool {
        unsafe { nk_input_is_mouse_prev_hovering_rect(&self.internal, rect) != 0 }
    }

    pub fn is_mouse_hovering_rect(&self, rect: NkRect) -> bool {
        unsafe { nk_input_is_mouse_hovering_rect(&self.internal, rect) != 0 }
    }

    pub fn is_mouse_clicked(&self, b: NkButton, rect: NkRect) -> bool {
        unsafe { nk_input_mouse_clicked(&self.internal, b, rect) != 0 }
    }

    pub fn is_mouse_down(&self, b: NkButton) -> bool {
        unsafe { nk_input_is_mouse_down(&self.internal, b) != 0 }
    }

    pub fn is_mouse_pressed(&self, b: NkButton) -> bool {
        unsafe { nk_input_is_mouse_pressed(&self.internal, b) != 0 }
    }

    pub fn nk_input_is_mouse_released(&self, b: NkButton) -> bool {
        unsafe { nk_input_is_mouse_released(&self.internal, b) != 0 }
    }

    pub fn is_key_pressed(&self, k: NkKey) -> bool {
        unsafe { nk_input_is_key_pressed(&self.internal, k) != 0 }
    }

    pub fn is_key_released(&self, k: NkKey) -> bool {
        unsafe { nk_input_is_key_released(&self.internal, k) != 0 }
    }

    pub fn is_key_down(&self, k: NkKey) -> bool {
        unsafe { nk_input_is_key_down(&self.internal, k) != 0 }
    }
}

// =====================================================================

wrapper_type!(NkDrawCommand, nk_draw_command);

impl NkDrawCommand {
    pub fn clip_rect(&self) -> &NkRect {
        &self.internal.clip_rect
    }

    pub fn elem_count(&self) -> u32 {
        self.internal.elem_count
    }

    pub fn texture(&self) -> NkHandle {
        NkHandle {
            kind: NkHandleKind::Unknown,
            internal: self.internal.texture,
        }
    }
}

// =====================================================================

#[derive(Copy, Clone, Debug)]
pub struct NkMouseButton {
    pub down: bool,
    pub clicked: bool,
    pub clicked_pos: NkVec2,
}

impl NkMouseButton {
    fn from_native(n: nk_mouse_button) -> NkMouseButton {
        NkMouseButton {
            down: n.down > 0,
            clicked: n.clicked > 0,
            clicked_pos: n.clicked_pos,
        }
    }
}

wrapper_type!(NkMouse, nk_mouse);

impl NkMouse {
    pub fn pos(&self) -> &NkVec2 {
        &self.internal.pos
    }

    pub fn prev(&self) -> &NkVec2 {
        &self.internal.prev
    }

    pub fn delta(&self) -> &NkVec2 {
        &self.internal.delta
    }

    pub fn scroll_delta(&self) -> f32 {
        self.internal.scroll_delta
    }

    pub fn buttons(&self) -> [NkMouseButton; 3] {
        [NkMouseButton::from_native(self.internal.buttons[0]), NkMouseButton::from_native(self.internal.buttons[1]), NkMouseButton::from_native(self.internal.buttons[2])]
    }

    pub fn grabbed(&self) -> bool {
        self.internal.grabbed > 0
    }

    // pub fn grab(&mut self) {
    // self.internal.grab = 1;
    // self.internal.ungrab = 0;
    // }
    //
    // pub fn ungrab(&mut self) {
    // self.internal.grab = 0;
    // self.internal.ungrab = 1;
    // }
}

// =====================================================================

// =====================================================================

wrapper_type!(NkStyle, nk_style);

impl NkStyle {
    pub fn window(&mut self) -> &mut NkStyleWindow {
        unsafe { ::std::mem::transmute(&mut self.internal.window) }
    }

    pub fn font(&self) -> &NkUserFont {
        unsafe { ::std::mem::transmute(self.internal.font) }
    }

    pub fn cursors(&self) -> &NkCursorMap {
        unsafe { ::std::mem::transmute(&self.internal.cursors) }
    }

    pub fn cursor_active(&self) -> &NkCursor {
        unsafe { ::std::mem::transmute(&self.internal.cursor_active) }
    }

    pub fn cursor_visible(&self) -> bool {
        self.internal.cursor_visible > 0
    }

    pub fn text(&self) -> &NkStyleText {
        &self.internal.text
    }

    pub fn button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.button) }
    }

    pub fn contextual_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.contextual_button) }
    }

    pub fn menu_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.menu_button) }
    }

    pub fn option(&self) -> &NkStyleToggle {
        unsafe { ::std::mem::transmute(&self.internal.option) }
    }

    pub fn checkbox(&self) -> &NkStyleToggle {
        unsafe { ::std::mem::transmute(&self.internal.checkbox) }
    }

    pub fn selectable(&self) -> &NkStyleSelectable {
        unsafe { ::std::mem::transmute(&self.internal.selectable) }
    }

    pub fn slider(&self) -> &NkStyleSlider {
        unsafe { ::std::mem::transmute(&self.internal.slider) }
    }

    pub fn progress(&self) -> &NkStyleProgress {
        unsafe { ::std::mem::transmute(&self.internal.progress) }
    }

    pub fn property(&self) -> &NkStyleProperty {
        unsafe { ::std::mem::transmute(&self.internal.property) }
    }

    pub fn edit(&self) -> &NkStyleEdit {
        unsafe { ::std::mem::transmute(&self.internal.edit) }
    }

    pub fn chart(&self) -> &NkStyleChart {
        unsafe { ::std::mem::transmute(&self.internal.chart) }
    }

    pub fn scroll_h(&self) -> &NkStyleScrollbar {
        unsafe { ::std::mem::transmute(&self.internal.scrollh) }
    }

    pub fn scroll_v(&self) -> &NkStyleScrollbar {
        unsafe { ::std::mem::transmute(&self.internal.scrollv) }
    }

    pub fn tab(&self) -> &NkStyleTab {
        unsafe { ::std::mem::transmute(&self.internal.tab) }
    }

    pub fn combo(&self) -> &NkStyleCombo {
        unsafe { ::std::mem::transmute(&self.internal.combo) }
    }
}

// =====================================================================

wrapper_type!(NkStyleCombo, nk_style_combo);

impl NkStyleCombo {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn label_normal(&self) -> &NkColor {
        &self.internal.label_normal
    }

    pub fn label_hover(&self) -> &NkColor {
        &self.internal.label_hover
    }

    pub fn label_active(&self) -> &NkColor {
        &self.internal.label_active
    }

    pub fn symbol_normal(&self) -> &NkColor {
        &self.internal.symbol_normal
    }

    pub fn symbol_hover(&self) -> &NkColor {
        &self.internal.symbol_hover
    }

    pub fn symbol_active(&self) -> &NkColor {
        &self.internal.symbol_active
    }

    pub fn button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.button) }
    }

    pub fn sym_normal(&self) -> &NkSymbolType {
        &self.internal.sym_normal
    }

    pub fn sym_hover(&self) -> &NkSymbolType {
        &self.internal.sym_hover
    }

    pub fn sym_active(&self) -> &NkSymbolType {
        &self.internal.sym_active
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn content_padding(&self) -> &NkVec2 {
        &self.internal.content_padding
    }

    pub fn button_padding(&self) -> &NkVec2 {
        &self.internal.button_padding
    }

    pub fn spacing(&self) -> &NkVec2 {
        &self.internal.spacing
    }

    // ===== setters ======

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: NkStyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_label_normal(&mut self, c: NkColor) {
        self.internal.label_normal = c
    }

    pub fn set_label_hover(&mut self, c: NkColor) {
        self.internal.label_hover = c
    }

    pub fn set_label_active(&mut self, c: NkColor) {
        self.internal.label_active = c
    }

    pub fn set_symbol_normal(&mut self, c: NkColor) {
        self.internal.symbol_normal = c
    }

    pub fn set_symbol_hover(&mut self, c: NkColor) {
        self.internal.symbol_hover = c
    }

    pub fn set_symbol_active(&mut self, c: NkColor) {
        self.internal.symbol_active = c
    }

    pub fn set_button(&mut self, s: NkStyleButton) {
        self.internal.button = s.internal
    }

    pub fn set_sym_normal(&mut self, t: NkSymbolType) {
        self.internal.sym_normal = t
    }

    pub fn set_sym_hover(&mut self, t: NkSymbolType) {
        self.internal.sym_hover = t
    }

    pub fn set_sym_active(&mut self, t: NkSymbolType) {
        self.internal.sym_active = t
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_content_padding(&mut self, v: NkVec2) {
        self.internal.content_padding = v
    }

    pub fn set_button_padding(&mut self, v: NkVec2) {
        self.internal.button_padding = v
    }

    pub fn set_spacing(&mut self, v: NkVec2) {
        self.internal.spacing = v
    }
}

// =====================================================================

wrapper_type!(NkStyleTab, nk_style_tab);

impl NkStyleTab {
    // ===== getters =====

    pub fn background(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.background) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn text(&self) -> &NkColor {
        &self.internal.text
    }

    pub fn tab_maximize_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.tab_maximize_button) }
    }

    pub fn tab_minimize_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.tab_minimize_button) }
    }

    pub fn node_maximize_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.node_maximize_button) }
    }

    pub fn node_minimize_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.node_minimize_button) }
    }

    pub fn sym_minimize(&self) -> &NkSymbolType {
        &self.internal.sym_minimize
    }

    pub fn sym_maximize(&self) -> &NkSymbolType {
        &self.internal.sym_maximize
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn indent(&self) -> f32 {
        self.internal.indent
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn spacing(&self) -> &NkVec2 {
        &self.internal.spacing
    }

    // ===== setters =====

    pub fn set_background(&mut self, i: NkStyleItem) {
        self.internal.background = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_text(&mut self, c: NkColor) {
        self.internal.text = c
    }

    pub fn set_tab_maximize_button(&mut self, s: NkStyleButton) {
        self.internal.tab_maximize_button = s.internal
    }

    pub fn set_tab_minimize_button(&mut self, s: NkStyleButton) {
        self.internal.tab_minimize_button = s.internal
    }

    pub fn set_node_maximize_button(&mut self, s: NkStyleButton) {
        self.internal.node_maximize_button = s.internal
    }

    pub fn set_node_minimize_button(&mut self, s: NkStyleButton) {
        self.internal.node_minimize_button = s.internal
    }

    pub fn set_sym_minimize(&mut self, t: NkSymbolType) {
        self.internal.sym_minimize = t
    }

    pub fn set_sym_maximize(&mut self, t: NkSymbolType) {
        self.internal.sym_maximize = t
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_indent(&mut self, v: f32) {
        self.internal.indent = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }

    pub fn set_spacing(&mut self, v: NkVec2) {
        self.internal.spacing = v
    }
}

// =====================================================================

wrapper_type!(NkStyleScrollbar, nk_style_scrollbar);

impl NkStyleScrollbar {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn cursor_normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_normal) }
    }

    pub fn cursor_hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_hover) }
    }

    pub fn cursor_active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_active) }
    }

    pub fn cursor_border_color(&self) -> &NkColor {
        &self.internal.cursor_border_color
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn border_cursor(&self) -> f32 {
        self.internal.border_cursor
    }

    pub fn rounding_cursor(&self) -> f32 {
        self.internal.rounding_cursor
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn show_buttons(&self) -> bool {
        self.internal.show_buttons > 0
    }

    pub fn inc_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.inc_button) }
    }

    pub fn dec_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.dec_button) }
    }

    pub fn inc_symbol(&self) -> &NkSymbolType {
        &self.internal.inc_symbol
    }

    pub fn dec_symbol(&self) -> &NkSymbolType {
        &self.internal.dec_symbol
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: NkStyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_cursor_normal(&mut self, i: NkStyleItem) {
        self.internal.cursor_normal = i.internal;
    }

    pub fn set_cursor_hover(&mut self, i: NkStyleItem) {
        self.internal.cursor_hover = i.internal;
    }

    pub fn set_cursor_active(&mut self, i: NkStyleItem) {
        self.internal.cursor_active = i.internal;
    }

    pub fn set_cursor_border_color(&mut self, c: NkColor) {
        self.internal.cursor_border_color = c
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_border_cursor(&mut self, v: f32) {
        self.internal.border_cursor = v
    }

    pub fn set_rounding_cursor(&mut self, v: f32) {
        self.internal.rounding_cursor = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }

    pub fn set_show_buttons(&mut self, show: bool) {
        self.internal.show_buttons = if show { 1 } else { 0 }
    }

    pub fn set_inc_button(&mut self, s: NkStyleButton) {
        self.internal.inc_button = s.internal
    }

    pub fn set_dec_button(&mut self, s: NkStyleButton) {
        self.internal.dec_button = s.internal
    }

    pub fn set_inc_symbol(&mut self, t: NkSymbolType) {
        self.internal.inc_symbol = t
    }

    pub fn set_dec_symbol(&mut self, t: NkSymbolType) {
        self.internal.dec_symbol = t
    }
}

// =====================================================================

wrapper_type!(NkStyleChart, nk_style_chart);

impl NkStyleChart {
    // ===== getters =====

    pub fn background(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.background) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn selected_color(&self) -> &NkColor {
        &self.internal.selected_color
    }

    pub fn color(&self) -> &NkColor {
        &self.internal.color
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    // ===== setters =====

    pub fn set_background(&mut self, i: NkStyleItem) {
        self.internal.background = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_selected_color(&mut self, c: NkColor) {
        self.internal.selected_color = c
    }

    pub fn set_color(&mut self, c: NkColor) {
        self.internal.color = c
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }
}

// =====================================================================

wrapper_type!(NkStyleEdit, nk_style_edit);

impl NkStyleEdit {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn scrollbar(&self) -> &NkStyleScrollbar {
        unsafe { ::std::mem::transmute(&self.internal.scrollbar) }
    }

    pub fn cursor_normal(&self) -> &NkColor {
        &self.internal.cursor_normal
    }

    pub fn cursor_hover(&self) -> &NkColor {
        &self.internal.cursor_hover
    }

    pub fn cursor_text_normal(&self) -> &NkColor {
        &self.internal.cursor_text_normal
    }

    pub fn cursor_text_hover(&self) -> &NkColor {
        &self.internal.cursor_text_hover
    }

    pub fn text_normal(&self) -> &NkColor {
        &self.internal.text_normal
    }

    pub fn text_hover(&self) -> &NkColor {
        &self.internal.text_hover
    }

    pub fn text_active(&self) -> &NkColor {
        &self.internal.text_active
    }

    pub fn selected_normal(&self) -> &NkColor {
        &self.internal.selected_normal
    }

    pub fn selected_hover(&self) -> &NkColor {
        &self.internal.selected_hover
    }

    pub fn selected_text_normal(&self) -> &NkColor {
        &self.internal.selected_text_normal
    }

    pub fn selected_text_hover(&self) -> &NkColor {
        &self.internal.selected_text_hover
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn cursor_size(&self) -> f32 {
        self.internal.cursor_size
    }

    pub fn scrollbar_size(&self) -> &NkVec2 {
        &self.internal.scrollbar_size
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn row_padding(&self) -> f32 {
        self.internal.row_padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: NkStyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_cursor_normal(&mut self, i: NkColor) {
        self.internal.cursor_normal = i;
    }

    pub fn set_cursor_hover(&mut self, i: NkColor) {
        self.internal.cursor_hover = i;
    }

    pub fn set_cursor_text_normal(&mut self, i: NkColor) {
        self.internal.cursor_text_normal = i;
    }

    pub fn set_cursor_text_hover(&mut self, i: NkColor) {
        self.internal.cursor_text_hover = i;
    }

    pub fn set_text_normal(&mut self, i: NkColor) {
        self.internal.text_normal = i;
    }

    pub fn set_text_hover(&mut self, i: NkColor) {
        self.internal.text_hover = i;
    }

    pub fn set_text_active(&mut self, i: NkColor) {
        self.internal.text_active = i;
    }

    pub fn set_selected_normal(&mut self, i: NkColor) {
        self.internal.selected_normal = i;
    }

    pub fn set_selected_hover(&mut self, i: NkColor) {
        self.internal.selected_hover = i;
    }

    pub fn set_selected_text_normal(&mut self, i: NkColor) {
        self.internal.selected_text_normal = i;
    }

    pub fn set_selected_text_hover(&mut self, i: NkColor) {
        self.internal.selected_text_hover = i;
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_cursor_size(&mut self, v: f32) {
        self.internal.cursor_size = v
    }

    pub fn set_scrollbar_size(&mut self, v: NkVec2) {
        self.internal.scrollbar_size = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }

    pub fn set_row_padding(&mut self, v: f32) {
        self.internal.row_padding = v
    }
}

// =====================================================================

wrapper_type!(NkStyleProperty, nk_style_property);

impl NkStyleProperty {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn label_normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.label_normal) }
    }

    pub fn label_hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.label_hover) }
    }

    pub fn label_active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.label_active) }
    }

    pub fn sym_left(&self) -> &NkSymbolType {
        &self.internal.sym_left
    }

    pub fn sym_right(&self) -> &NkSymbolType {
        &self.internal.sym_right
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn edit(&self) -> &NkStyleEdit {
        unsafe { ::std::mem::transmute(&self.internal.edit) }
    }

    pub fn inc_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.inc_button) }
    }

    pub fn dec_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.dec_button) }
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: NkStyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_label_normal(&mut self, c: NkColor) {
        self.internal.label_normal = c
    }

    pub fn set_label_hover(&mut self, c: NkColor) {
        self.internal.label_hover = c
    }

    pub fn set_label_active(&mut self, c: NkColor) {
        self.internal.label_active = c
    }

    pub fn set_sym_left(&mut self, t: NkSymbolType) {
        self.internal.sym_left = t
    }

    pub fn set_sym_right(&mut self, t: NkSymbolType) {
        self.internal.sym_right = t
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }

    pub fn set_inc_button(&mut self, s: NkStyleButton) {
        self.internal.inc_button = s.internal
    }

    pub fn set_dec_button(&mut self, s: NkStyleButton) {
        self.internal.dec_button = s.internal
    }
}

// =====================================================================

wrapper_type!(NkStyleProgress, nk_style_progress);

impl NkStyleProgress {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn cursor_normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_normal) }
    }

    pub fn cursor_hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_hover) }
    }

    pub fn cursor_active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_active) }
    }

    pub fn cursor_border_color(&self) -> &NkColor {
        &self.internal.cursor_border_color
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn cursor_border(&self) -> f32 {
        self.internal.cursor_border
    }

    pub fn cursor_rounding(&self) -> f32 {
        self.internal.cursor_rounding
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: NkStyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_cursor_normal(&mut self, i: NkStyleItem) {
        self.internal.cursor_normal = i.internal;
    }

    pub fn set_cursor_hover(&mut self, i: NkStyleItem) {
        self.internal.cursor_hover = i.internal;
    }

    pub fn set_cursor_active(&mut self, i: NkStyleItem) {
        self.internal.cursor_active = i.internal;
    }

    pub fn set_cursor_border_color(&mut self, c: NkColor) {
        self.internal.cursor_border_color = c
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_cursor_border(&mut self, v: f32) {
        self.internal.cursor_border = v
    }

    pub fn set_cursor_rounding(&mut self, v: f32) {
        self.internal.cursor_rounding = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }
}

// =====================================================================

wrapper_type!(NkStyleSlider, nk_style_slider);

impl NkStyleSlider {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn bar_normal(&self) -> &NkColor {
        &self.internal.bar_normal
    }

    pub fn bar_hover(&self) -> &NkColor {
        &self.internal.bar_hover
    }

    pub fn bar_active(&self) -> &NkColor {
        &self.internal.bar_active
    }

    pub fn bar_filled(&self) -> &NkColor {
        &self.internal.bar_filled
    }

    pub fn cursor_normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_normal) }
    }

    pub fn cursor_hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_hover) }
    }

    pub fn cursor_active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_active) }
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn bar_height(&self) -> f32 {
        self.internal.bar_height
    }

    pub fn spacing(&self) -> &NkVec2 {
        &self.internal.spacing
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn cursor_size(&self) -> &NkVec2 {
        &self.internal.cursor_size
    }

    pub fn show_buttons(&self) -> bool {
        self.internal.show_buttons > 0
    }

    pub fn inc_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.inc_button) }
    }

    pub fn dec_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.dec_button) }
    }

    pub fn inc_symbol(&self) -> &NkSymbolType {
        &self.internal.inc_symbol
    }

    pub fn dec_symbol(&self) -> &NkSymbolType {
        &self.internal.dec_symbol
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: NkStyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_bar_normal(&mut self, c: NkColor) {
        self.internal.bar_normal = c
    }

    pub fn set_bar_hover(&mut self, c: NkColor) {
        self.internal.bar_hover = c
    }

    pub fn set_bar_active(&mut self, c: NkColor) {
        self.internal.bar_active = c
    }

    pub fn set_bar_filled(&mut self, c: NkColor) {
        self.internal.bar_filled = c
    }

    pub fn set_cursor_normal(&mut self, i: NkStyleItem) {
        self.internal.cursor_normal = i.internal;
    }

    pub fn set_cursor_hover(&mut self, i: NkStyleItem) {
        self.internal.cursor_hover = i.internal;
    }

    pub fn set_cursor_active(&mut self, i: NkStyleItem) {
        self.internal.cursor_active = i.internal;
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_bar_height(&mut self, v: f32) {
        self.internal.bar_height = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }

    pub fn set_spacing(&mut self, v: NkVec2) {
        self.internal.spacing = v
    }

    pub fn set_cursor_size(&mut self, v: NkVec2) {
        self.internal.cursor_size = v
    }

    pub fn set_show_buttons(&mut self, show: bool) {
        self.internal.show_buttons = if show { 1 } else { 0 }
    }

    pub fn set_inc_button(&mut self, s: NkStyleButton) {
        self.internal.inc_button = s.internal
    }

    pub fn set_dec_button(&mut self, s: NkStyleButton) {
        self.internal.dec_button = s.internal
    }

    pub fn set_inc_symbol(&mut self, t: NkSymbolType) {
        self.internal.inc_symbol = t
    }

    pub fn set_dec_symbol(&mut self, t: NkSymbolType) {
        self.internal.dec_symbol = t
    }
}

// =====================================================================

wrapper_type!(NkStyleSelectable, nk_style_selectable);

impl NkStyleSelectable {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn pressed(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.pressed) }
    }

    pub fn normal_active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal_active) }
    }

    pub fn hover_active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover_active) }
    }

    pub fn pressed_active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.pressed_active) }
    }

    pub fn text_normal(&self) -> &NkColor {
        &self.internal.text_normal
    }

    pub fn text_hover(&self) -> &NkColor {
        &self.internal.text_hover
    }

    pub fn text_pressed(&self) -> &NkColor {
        &self.internal.text_pressed
    }

    pub fn text_normal_active(&self) -> &NkColor {
        &self.internal.text_normal_active
    }

    pub fn text_hover_active(&self) -> &NkColor {
        &self.internal.text_hover_active
    }

    pub fn text_pressed_active(&self) -> &NkColor {
        &self.internal.text_pressed_active
    }

    pub fn text_background(&self) -> &NkColor {
        &self.internal.text_background
    }

    pub fn text_alignment(&self) -> u32 {
        self.internal.text_alignment
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn touch_padding(&self) -> &NkVec2 {
        &self.internal.touch_padding
    }

    pub fn image_padding(&self) -> &NkVec2 {
        &self.internal.image_padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_pressed(&mut self, i: NkStyleItem) {
        self.internal.pressed = i.internal;
    }

    pub fn set_normal_active(&mut self, i: NkStyleItem) {
        self.internal.normal_active = i.internal;
    }

    pub fn set_hover_active(&mut self, i: NkStyleItem) {
        self.internal.hover_active = i.internal;
    }

    pub fn set_pressed_active(&mut self, i: NkStyleItem) {
        self.internal.pressed_active = i.internal;
    }

    pub fn set_text_normal(&mut self, c: NkColor) {
        self.internal.text_normal = c
    }

    pub fn set_text_hover(&mut self, c: NkColor) {
        self.internal.text_hover = c
    }

    pub fn set_text_pressed(&mut self, c: NkColor) {
        self.internal.text_pressed = c
    }

    pub fn set_text_normal_active(&mut self, c: NkColor) {
        self.internal.text_normal_active = c
    }

    pub fn set_text_hover_active(&mut self, c: NkColor) {
        self.internal.text_hover_active = c
    }

    pub fn set_text_pressed_active(&mut self, c: NkColor) {
        self.internal.text_pressed_active = c
    }

    pub fn set_text_background(&mut self, c: NkColor) {
        self.internal.text_background = c
    }

    pub fn set_text_alignment(&mut self, v: u32) {
        self.internal.text_alignment = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }

    pub fn set_touch_padding(&mut self, v: NkVec2) {
        self.internal.touch_padding = v
    }

    pub fn set_image_padding(&mut self, v: NkVec2) {
        self.internal.image_padding = v
    }
}

// =====================================================================

wrapper_type!(NkStyleButton, nk_style_button);

impl NkStyleButton {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn text_background(&self) -> &NkColor {
        &self.internal.text_background
    }

    pub fn text_normal(&self) -> &NkColor {
        &self.internal.text_normal
    }

    pub fn text_hover(&self) -> &NkColor {
        &self.internal.text_hover
    }

    pub fn text_active(&self) -> &NkColor {
        &self.internal.text_active
    }

    pub fn text_alignment(&self) -> u32 {
        self.internal.text_alignment
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn touch_padding(&self) -> &NkVec2 {
        &self.internal.touch_padding
    }

    pub fn image_padding(&self) -> &NkVec2 {
        &self.internal.image_padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: NkStyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_text_background(&mut self, c: NkColor) {
        self.internal.text_background = c
    }

    pub fn set_text_normal(&mut self, c: NkColor) {
        self.internal.text_normal = c
    }

    pub fn set_text_hover(&mut self, c: NkColor) {
        self.internal.text_hover = c
    }

    pub fn set_text_active(&mut self, c: NkColor) {
        self.internal.text_active = c
    }

    pub fn set_text_alignment(&mut self, c: u32) {
        self.internal.text_alignment = c
    }

    pub fn set_border(&mut self, c: f32) {
        self.internal.border = c
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }

    pub fn set_touch_padding(&mut self, v: NkVec2) {
        self.internal.touch_padding = v
    }

    pub fn set_image_padding(&mut self, v: NkVec2) {
        self.internal.image_padding = v
    }
}

// =====================================================================

wrapper_type!(NkStyleToggle, nk_style_toggle);

impl NkStyleToggle {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn cursor_normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_normal) }
    }

    pub fn cursor_hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_hover) }
    }

    pub fn text_normal(&self) -> &NkColor {
        &self.internal.text_normal
    }

    pub fn text_hover(&self) -> &NkColor {
        &self.internal.text_hover
    }

    pub fn text_active(&self) -> &NkColor {
        &self.internal.text_active
    }

    pub fn text_background(&self) -> &NkColor {
        &self.internal.text_background
    }

    pub fn text_alignment(&self) -> u32 {
        self.internal.text_alignment
    }

    pub fn spacing(&self) -> f32 {
        self.internal.spacing
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn touch_padding(&self) -> &NkVec2 {
        &self.internal.touch_padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: NkStyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: NkColor) {
        self.internal.border_color = c
    }

    pub fn set_cursor_normal(&mut self, i: NkStyleItem) {
        self.internal.cursor_normal = i.internal;
    }

    pub fn set_cursor_hover(&mut self, i: NkStyleItem) {
        self.internal.cursor_hover = i.internal;
    }

    pub fn set_text_background(&mut self, c: NkColor) {
        self.internal.text_background = c
    }

    pub fn set_text_normal(&mut self, c: NkColor) {
        self.internal.text_normal = c
    }

    pub fn set_text_hover(&mut self, c: NkColor) {
        self.internal.text_hover = c
    }

    pub fn set_text_active(&mut self, c: NkColor) {
        self.internal.text_active = c
    }

    pub fn set_text_alignment(&mut self, c: u32) {
        self.internal.text_alignment = c
    }

    pub fn set_spacing(&mut self, v: f32) {
        self.internal.spacing = v
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }

    pub fn set_touch_padding(&mut self, v: NkVec2) {
        self.internal.touch_padding = v
    }
}

// =====================================================================

wrapper_type!(NkStyleWindowHeader, nk_style_window_header);

impl NkStyleWindowHeader {
    // ===== getters =====

    pub fn normal(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn close_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.close_button) }
    }

    pub fn minimize_button(&self) -> &NkStyleButton {
        unsafe { ::std::mem::transmute(&self.internal.minimize_button) }
    }

    pub fn close_symbol(&self) -> &NkSymbolType {
        &self.internal.close_symbol
    }

    pub fn minimize_symbol(&self) -> &NkSymbolType {
        &self.internal.minimize_symbol
    }

    pub fn maximize_symbol(&self) -> &NkSymbolType {
        &self.internal.maximize_symbol
    }

    pub fn label_normal(&self) -> &NkColor {
        &self.internal.label_normal
    }

    pub fn label_hover(&self) -> &NkColor {
        &self.internal.label_hover
    }

    pub fn label_active(&self) -> &NkColor {
        &self.internal.label_active
    }

    pub fn align(&self) -> &NkStyleHeaderAlign {
        &self.internal.align
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn label_padding(&self) -> &NkVec2 {
        &self.internal.label_padding
    }

    pub fn spacing(&self) -> &NkVec2 {
        &self.internal.spacing
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: NkStyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: NkStyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: NkStyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_close_symbol(&mut self, t: NkSymbolType) {
        self.internal.close_symbol = t
    }

    pub fn set_minimize_symbol(&mut self, t: NkSymbolType) {
        self.internal.minimize_symbol = t
    }

    pub fn set_maximize_symbol(&mut self, t: NkSymbolType) {
        self.internal.maximize_symbol = t
    }

    pub fn set_label_normal(&mut self, c: NkColor) {
        self.internal.label_normal = c
    }

    pub fn set_label_hover(&mut self, c: NkColor) {
        self.internal.label_hover = c
    }

    pub fn set_label_active(&mut self, c: NkColor) {
        self.internal.label_active = c
    }

    pub fn set_align(&mut self, c: NkStyleHeaderAlign) {
        self.internal.align = c
    }

    pub fn set_padding(&mut self, v: NkVec2) {
        self.internal.padding = v
    }

    pub fn set_label_padding(&mut self, v: NkVec2) {
        self.internal.label_padding = v
    }

    pub fn set_spacing(&mut self, v: NkVec2) {
        self.internal.spacing = v
    }
}

// =====================================================================

wrapper_type!(NkStyleWindow, nk_style_window);

impl NkStyleWindow {
    // ===== getters =====

    pub fn header(&self) -> &NkStyleWindowHeader {
        unsafe { ::std::mem::transmute(&self.internal.header) }
    }

    pub fn fixed_background(&self) -> NkStyleItem {
        NkStyleItem { internal: self.internal.fixed_background }
    }

    pub fn background(&self) -> &NkColor {
        &self.internal.background
    }

    pub fn border_color(&self) -> &NkColor {
        &self.internal.border_color
    }

    pub fn popup_border_color(&self) -> &NkColor {
        &self.internal.popup_border_color
    }

    pub fn combo_border_color(&self) -> &NkColor {
        &self.internal.combo_border_color
    }

    pub fn contextual_border_color(&self) -> &NkColor {
        &self.internal.contextual_border_color
    }

    pub fn menu_border_color(&self) -> &NkColor {
        &self.internal.menu_border_color
    }

    pub fn group_border_color(&self) -> &NkColor {
        &self.internal.group_border_color
    }

    pub fn tooltip_border_color(&self) -> &NkColor {
        &self.internal.tooltip_border_color
    }

    pub fn scaler(&self) -> &NkStyleItem {
        unsafe { ::std::mem::transmute(&self.internal.scaler) }
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn combo_border(&self) -> f32 {
        self.internal.combo_border
    }

    pub fn contextual_border(&self) -> f32 {
        self.internal.contextual_border
    }

    pub fn menu_border(&self) -> f32 {
        self.internal.menu_border
    }

    pub fn group_border(&self) -> f32 {
        self.internal.group_border
    }

    pub fn tooltip_border(&self) -> f32 {
        self.internal.tooltip_border
    }

    pub fn popup_border(&self) -> f32 {
        self.internal.popup_border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn spacing(&self) -> &NkVec2 {
        &self.internal.spacing
    }

    pub fn scrollbar_size(&self) -> &NkVec2 {
        &self.internal.scrollbar_size
    }

    pub fn min_size(&self) -> &NkVec2 {
        &self.internal.min_size
    }

    pub fn padding(&self) -> &NkVec2 {
        &self.internal.padding
    }

    pub fn group_padding(&self) -> &NkVec2 {
        &self.internal.group_padding
    }

    pub fn popup_padding(&self) -> &NkVec2 {
        &self.internal.popup_padding
    }

    pub fn combo_padding(&self) -> &NkVec2 {
        &self.internal.combo_padding
    }

    pub fn contextual_padding(&self) -> &NkVec2 {
        &self.internal.contextual_padding
    }

    pub fn menu_padding(&self) -> &NkVec2 {
        &self.internal.menu_padding
    }

    pub fn tooltip_padding(&self) -> &NkVec2 {
        &self.internal.tooltip_padding
    }

    // ===== setters =====

    pub fn set_fixed_background(&mut self, item: NkStyleItem) {
        self.internal.fixed_background = item.internal;
    }

    pub fn set_background(&mut self, color: NkColor) {
        self.internal.background = color;
    }

    pub fn set_border_color(&mut self, color: NkColor) {
        self.internal.border_color = color;
    }

    pub fn set_popup_border_color(&mut self, color: NkColor) {
        self.internal.popup_border_color = color;
    }

    pub fn set_combo_border_color(&mut self, color: NkColor) {
        self.internal.combo_border_color = color;
    }

    pub fn set_contextual_border_color(&mut self, color: NkColor) {
        self.internal.contextual_border_color = color;
    }

    pub fn set_menu_border_color(&mut self, color: NkColor) {
        self.internal.menu_border_color = color;
    }

    pub fn set_group_border_color(&mut self, color: NkColor) {
        self.internal.group_border_color = color;
    }

    pub fn set_tooltip_border_color(&mut self, color: NkColor) {
        self.internal.tooltip_border_color = color;
    }

    pub fn set_scaler(&mut self, i: NkStyleItem) {
        self.internal.scaler = i.internal;
    }

    pub fn set_combo_border(&mut self, v: f32) {
        self.internal.combo_border = v
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_contextual_border(&mut self, v: f32) {
        self.internal.contextual_border = v
    }

    pub fn set_menu_border(&mut self, v: f32) {
        self.internal.menu_border = v
    }

    pub fn set_group_border(&mut self, v: f32) {
        self.internal.group_border = v
    }

    pub fn set_tooltip_border(&mut self, v: f32) {
        self.internal.tooltip_border = v
    }

    pub fn set_popup_border(&mut self, v: f32) {
        self.internal.popup_border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_spacing(&mut self, spacing: NkVec2) {
        self.internal.spacing = spacing;
    }

    pub fn set_scrollbar_size(&mut self, s: NkVec2) {
        self.internal.scrollbar_size = s;
    }

    pub fn set_min_size(&mut self, s: NkVec2) {
        self.internal.min_size = s;
    }

    pub fn set_padding(&mut self, padding: NkVec2) {
        self.internal.padding = padding;
    }

    pub fn set_group_padding(&mut self, padding: NkVec2) {
        self.internal.group_padding = padding;
    }

    pub fn set_popup_padding(&mut self, padding: NkVec2) {
        self.internal.popup_padding = padding;
    }

    pub fn set_combo_padding(&mut self, padding: NkVec2) {
        self.internal.combo_padding = padding;
    }

    pub fn set_contextual_padding(&mut self, padding: NkVec2) {
        self.internal.contextual_padding = padding;
    }

    pub fn set_menu_padding(&mut self, padding: NkVec2) {
        self.internal.menu_padding = padding;
    }

    pub fn set_tooltip_padding(&mut self, padding: NkVec2) {
        self.internal.tooltip_padding = padding;
    }
}

// =====================================================================

wrapper_type!(NkDrawList, nk_draw_list);

impl NkDrawList {
    pub fn init(&mut self) {
        unsafe {
            nk_draw_list_init(&mut self.internal);
        }
    }

    pub fn setup(&mut self, config: &NkConvertConfig, cmds: &mut NkBuffer, vertices: &mut NkBuffer, elements: &mut NkBuffer) {
        unsafe {
            nk_draw_list_setup(&mut self.internal,
                               &config.internal as *const nk_convert_config,
                               &mut cmds.internal as *mut nk_buffer,
                               &mut vertices.internal as *mut nk_buffer,
                               &mut elements.internal as *mut nk_buffer)
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            nk_draw_list_clear(&mut self.internal);
        }
    }

    pub fn begin(&self, buf: &NkBuffer) -> &NkDrawCommand {
        unsafe { ::std::mem::transmute(nk__draw_list_begin(&self.internal, &buf.internal)) }
    }

    pub fn next(&self, buf: &NkBuffer, prev: &NkDrawCommand) -> &NkDrawCommand {
        unsafe { ::std::mem::transmute(nk__draw_list_next(&prev.internal, &buf.internal, &self.internal)) }
    }

    pub fn path_clear(&mut self) {
        unsafe {
            nk_draw_list_path_clear(&mut self.internal);
        }
    }

    pub fn path_line_to(&mut self, pos: NkVec2) {
        unsafe {
            nk_draw_list_path_line_to(&mut self.internal, pos);
        }
    }

    pub fn path_arc_to_fast(&mut self, center: NkVec2, radius: f32, a_min: i32, a_max: i32) {
        unsafe {
            nk_draw_list_path_arc_to_fast(&mut self.internal, center, radius, a_min, a_max);
        }
    }

    pub fn path_arc_to(&mut self, center: NkVec2, radius: f32, a_min: f32, a_max: f32, segments: u32) {
        unsafe {
            nk_draw_list_path_arc_to(&mut self.internal, center, radius, a_min, a_max, segments);
        }
    }

    pub fn path_rect_to(&mut self, a: NkVec2, b: NkVec2, rounding: f32) {
        unsafe {
            nk_draw_list_path_rect_to(&mut self.internal, a, b, rounding);
        }
    }

    pub fn path_curve_to(&mut self, p2: NkVec2, p3: NkVec2, p4: NkVec2, num_segments: u32) {
        unsafe { nk_draw_list_path_curve_to(&mut self.internal, p2, p3, p4, num_segments) }
    }

    pub fn path_fill(&mut self, col: NkColor) {
        unsafe {
            nk_draw_list_path_fill(&mut self.internal, col);
        }
    }

    pub fn path_stroke(&mut self, arg2: NkColor, closed: NkDrawListStroke, thickness: f32) {
        unsafe {
            nk_draw_list_path_stroke(&mut self.internal, arg2, closed, thickness);
        }
    }

    pub fn stroke_line(&mut self, a: NkVec2, b: NkVec2, arg2: NkColor, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_line(&mut self.internal, a, b, arg2, thickness);
        }
    }

    pub fn stroke_rect(&mut self, rect: NkRect, arg2: NkColor, rounding: f32, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_rect(&mut self.internal, rect, arg2, rounding, thickness);
        }
    }

    pub fn stroke_triangle(&mut self, a: NkVec2, b: NkVec2, c: NkVec2, arg2: NkColor, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_triangle(&mut self.internal, a, b, c, arg2, thickness);
        }
    }

    pub fn stroke_circle(&mut self, center: NkVec2, radius: f32, arg2: NkColor, segs: u32, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_circle(&mut self.internal, center, radius, arg2, segs, thickness);
        }
    }

    pub fn stroke_curve(&mut self, p0: NkVec2, cp0: NkVec2, cp1: NkVec2, p1: NkVec2, arg2: NkColor, segments: u32, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_curve(&mut self.internal,
                                      p0,
                                      cp0,
                                      cp1,
                                      p1,
                                      arg2,
                                      segments,
                                      thickness);
        }
    }

    pub fn stroke_poly_line(&mut self, points: &[NkVec2], arg2: NkColor, arg3: NkDrawListStroke, thickness: f32, aa: NkAntiAliasing) {
        unsafe {
            nk_draw_list_stroke_poly_line(&mut self.internal,
                                          points.as_ptr(),
                                          points.len() as u32,
                                          arg2,
                                          arg3,
                                          thickness,
                                          aa);
        }
    }

    pub fn fill_rect(&mut self, rect: NkRect, arg2: NkColor, rounding: f32) {
        unsafe {
            nk_draw_list_fill_rect(&mut self.internal, rect, arg2, rounding);
        }
    }

    pub fn fill_rect_multi_color(&mut self, rect: NkRect, left: NkColor, top: NkColor, right: NkColor, bottom: NkColor) {
        unsafe {
            nk_draw_list_fill_rect_multi_color(&mut self.internal, rect, left, top, right, bottom);
        }
    }

    pub fn fill_triangle(&mut self, a: NkVec2, b: NkVec2, c: NkVec2, arg2: NkColor) {
        unsafe {
            nk_draw_list_fill_triangle(&mut self.internal, a, b, c, arg2);
        }
    }

    pub fn fill_circle(&mut self, center: NkVec2, radius: f32, col: NkColor, segs: u32) {
        unsafe {
            nk_draw_list_fill_circle(&mut self.internal, center, radius, col, segs);
        }
    }

    pub fn fill_poly_convex(&mut self, points: &[NkVec2], arg2: NkColor, arg3: NkAntiAliasing) {
        unsafe {
            nk_draw_list_fill_poly_convex(&mut self.internal,
                                          points.as_ptr(),
                                          points.len() as u32,
                                          arg2,
                                          arg3);
        }
    }

    pub fn add_image(&mut self, texture: NkImage, rect: NkRect, arg2: NkColor) {
        unsafe {
            nk_draw_list_add_image(&mut self.internal, texture.internal, rect, arg2);
        }
    }

    pub fn add_text(&mut self, arg2: &NkUserFont, arg3: NkRect, text: NkString, font_height: f32, arg4: NkColor) {
        unsafe {
            nk_draw_list_add_text(&mut self.internal,
                                  &arg2.internal,
                                  arg3,
                                  text.as_ptr(),
                                  text.bytes.len() as i32,
                                  font_height,
                                  arg4);
        }
    }

    // pub fn push_userdata(&mut self, userdata: nk_handle) {
    // unsafe {
    // nk_draw_list_push_userdata(&mut self as *mut nk_draw_list, userdata.internal);
    // }
    // }
}

// ========

#[derive(Clone, Copy)]
pub struct NkColorMap {
    internal: [nk_color; 28],
}

impl Default for NkColorMap {
    fn default() -> Self {
        NkColorMap { internal: [nk_color::default(); 28] }
    }
}

impl NkColorMap {
    pub fn set(&mut self, target: NkStyleColor, color: NkColor) {
        self.internal[target as usize] = color;
    }
}

// ==================================================================================

pub struct NkCursorMap<'a> {
    internal: [Option<&'a NkCursor>; 7],
}

impl<'a> Default for NkCursorMap<'a> {
    fn default() -> Self {
        unsafe {
            let mut map = NkCursorMap { internal: [::std::mem::zeroed(); 7] };

            for i in map.internal.iter_mut() {
                ::std::ptr::write(i, None);
            }

            map
        }
    }
}

impl<'a> NkCursorMap<'a> {
    pub fn set(&mut self, target: NkStyleCursor, res: Option<&'a NkCursor>) {
        self.internal[target as usize] = unsafe { ::std::mem::transmute(res) };
    }
}

// ==================================================================================

wrapper_type!(NkCursor, nk_cursor);

impl NkCursor {
	pub fn img(&self) -> &NkImage {
		unsafe {
			::std::mem::transmute(&self.internal.img)
		}
	}
    pub fn size(&self) -> &NkVec2 {
		&self.internal.size
	}
    pub fn offset(&self) -> &NkVec2 {
		&self.internal.offset
	}
}

// ==================================================================================

wrapper_type!(NkAllocator, nk_allocator);

impl NkAllocator {
    #[cfg(feature="rust_allocator")]
    pub fn new_heap() -> NkAllocator {
        let mut a = NkAllocator::default();

        a.internal.alloc = Some(alloc_heap::alloc);
        a.internal.free = Some(alloc_heap::free);
        a.internal.userdata = nk_handle::default();
        unsafe {
            *(a.internal.userdata.ptr.as_mut()) = ::std::ptr::null_mut();
        }
        a
    }

    pub fn new_vec() -> NkAllocator {
        let mut a = NkAllocator::default();

        a.internal.alloc = Some(alloc_vec::alloc);
        a.internal.free = Some(alloc_vec::free);
        a.internal.userdata = nk_handle::default();
        unsafe {
            *(a.internal.userdata.ptr.as_mut()) = ::std::ptr::null_mut();
        }
        a
    }
}

// ============================================================================================

wrapper_type!(NkConvertConfig, nk_convert_config);

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
    // pub fn set_vertex_alignment(&mut self, val: usize) {
    // self.internal.vertex_alignment = val;
    // }
}

// ============================================================================================

#[derive(Debug, Clone)]
pub struct NkDrawVertexLayoutElements {
    arr: Vec<nk_draw_vertex_layout_element>,
}

impl NkDrawVertexLayoutElements {
    pub fn new(var: &[(NkDrawVertexLayoutAttribute, NkDrawVertexLayoutFormat, u32)]) -> NkDrawVertexLayoutElements {
        NkDrawVertexLayoutElements {
            arr: var.iter()
                .map(|&(a, f, o)| {
                         nk_draw_vertex_layout_element {
                             attribute: a,
                             format: f,
                             offset: o as usize,
                         }
                     })
                .collect::<Vec<_>>(),
        }
    }
}

// =============================================================================================

wrapper_type!(NkStyleItem, nk_style_item);

impl NkStyleItem {
    pub fn image(img: NkImage) -> NkStyleItem {
        unsafe { NkStyleItem { internal: nk_style_item_image(img.internal) } }
    }

    pub fn color(col: NkColor) -> NkStyleItem {
        unsafe { NkStyleItem { internal: nk_style_item_color(col) } }
    }

    pub fn hide() -> NkStyleItem {
        unsafe { NkStyleItem { internal: nk_style_item_hide() } }
    }
}

// =============================================================================================

wrapper_type_no_clone!(NkTextEdit, nk_text_edit);

impl Drop for NkTextEdit {
	fn drop(&mut self) {
		unsafe {
            nk_textedit_free(&mut self.internal);
        }
	}
}

impl NkTextEdit {
    pub fn init(&mut self, arg2: &mut NkAllocator, size: usize) {
        unsafe {
            nk_textedit_init(&mut self.internal,
                             &mut arg2.internal as *mut nk_allocator,
                             size);
        }
    }

    pub fn text(&mut self, arg2: &str) {
        unsafe {
            nk_textedit_text(&mut self.internal,
                             arg2.as_ptr() as *const i8,
                             arg2.as_bytes().len() as ::std::os::raw::c_int);
        }
    }

    pub fn delete(&mut self, where_: u32, len: u32) {
        unsafe {
            nk_textedit_delete(&mut self.internal,
                               where_ as ::std::os::raw::c_int,
                               len as ::std::os::raw::c_int);
        }
    }

    pub fn delete_selection(&mut self) {
        unsafe {
            nk_textedit_delete_selection(&mut self.internal);
        }
    }

    pub fn select_all(&mut self) {
        unsafe {
            nk_textedit_select_all(&mut self.internal);
        }
    }

    pub fn cut(&mut self) -> bool {
        unsafe { nk_textedit_cut(&mut self.internal) != 0 }
    }

    pub fn paste(&mut self, arg2: &str) -> bool {
        unsafe {
            nk_textedit_paste(&mut self.internal,
                              arg2.as_ptr() as *const i8,
                              arg2.as_bytes().len() as ::std::os::raw::c_int) != 0
        }
    }

    pub fn undo(&mut self) {
        unsafe {
            nk_textedit_undo(&mut self.internal);
        }
    }

    pub fn redo(&mut self) {
        unsafe {
            nk_textedit_redo(&mut self.internal);
        }
    }

    // pub fn nk_textedit_init_fixed(arg1: *mut nk_text_edit,
    // memory: *mut ::std::os::raw::c_void,
    // size: nk_size);
    //
}

// =============================================================================================

wrapper_type!(NkFontConfig, nk_font_config);

impl NkFontConfig {
    pub fn with_size(pixel_height: f32) -> NkFontConfig {
        unsafe { NkFontConfig { internal: nk_font_config(pixel_height) } }
    }

    pub fn is_ttf_data_owned_by_atlas(&self) -> bool {
        self.internal.ttf_data_owned_by_atlas > 0
    }

    pub fn size(&self) -> f32 {
        self.internal.size
    }

    pub fn oversample_v(&self) -> u8 {
        self.internal.oversample_v
    }

    pub fn oversample_h(&self) -> u8 {
        self.internal.oversample_h
    }

    pub fn glyph_range<'a>(&'a self) -> Option<&'a [(u32, u32)]> {
        if self.internal.range.is_null() {
            None
        } else {
            Some(raw_glyph_ranges_to_safe(self.internal.range))
        }
    }

    // pub fn set_next<'a>(&'a mut self, next_cfg: &mut NkFontConfig) {
    // self.internal.next = &mut next_cfg.internal;
    // }

    pub fn padding(&self) -> [u8; 3] {
        self.internal.padding
    }

    pub fn fallback_glyph(&self) -> char {
        unsafe { ::std::char::from_u32_unchecked(self.internal.fallback_glyph) }
    }

    pub fn spacing(&self) -> &NkVec2 {
        &self.internal.spacing
    }

    pub fn coord_type(&self) -> &NkFontCoordType {
        &self.internal.coord_type
    }

    pub fn is_pixel_snap(&self) -> bool {
        self.internal.pixel_snap > 0
    }

    pub fn is_merge_mode(&self) -> bool {
        self.internal.merge_mode > 0
    }

    // ==

    pub fn set_ttf_data_owned_by_atlas(&mut self, yes: bool) {
        self.internal.ttf_data_owned_by_atlas = if yes { 1 } else { 0 };
    }

    pub fn set_size(&mut self, size: f32) {
        self.internal.size = size;
    }

    pub fn set_oversample_v(&mut self, v: u8) {
        self.internal.oversample_v = v;
    }

    pub fn set_oversample_h(&mut self, h: u8) {
        self.internal.oversample_h = h;
    }

    pub fn set_glyph_range<'a>(&'a mut self, ranges: &'a [(u32, u32)]) {
        self.internal.range = ranges as *const _ as *const u32;
    }

    // pub fn set_next<'a>(&'a mut self, next_cfg: &mut NkFontConfig) {
    // self.internal.next = &mut next_cfg.internal;
    // }

    pub fn set_ttf<'a>(&'a mut self, font_bytes: &'a [u8]) {
        self.internal.ttf_size = font_bytes.len();
        self.internal.ttf_blob = font_bytes as *const _ as *mut c_void;
    }

    pub fn set_padding(&mut self, p: [u8; 3]) {
        self.internal.padding = p;
    }

    pub fn set_fallback_glyph(&mut self, g: char) {
        self.internal.fallback_glyph = g as u32;
    }

    pub fn set_spacing(&mut self, s: NkVec2) {
        self.internal.spacing = s;
    }

    pub fn set_coord_type(&mut self, t: NkFontCoordType) {
        self.internal.coord_type = t;
    }

    pub fn set_pixel_snap(&mut self, s: bool) {
        self.internal.pixel_snap = if s { 1 } else { 0 };
    }

    pub fn set_merge_mode(&mut self, m: bool) {
        self.internal.merge_mode = if m { 1 } else { 0 };
    }

    // pub ttf_data_owned_by_atlas: ::std::os::raw::c_uchar,
    // pub font: *mut nk_baked_font,
    //
}

// =============================================================================================

wrapper_type!(NkFontAtlas, nk_font_atlas);
pub type NkFontID = usize;

impl Drop for NkFontAtlas {
	fn drop(&mut self) {
		self.clear();
	}
}

impl NkFontAtlas {
    pub fn new(alloc: &mut NkAllocator) -> NkFontAtlas {
        let mut a = NkFontAtlas::default();

        a.init(alloc);

        a
    }

    pub fn add_font_with_config(&mut self, cfg: &NkFontConfig) -> Option<Box<NkFont>> {
        unsafe {
            if self.internal.font_num < 1 {
	            nk_font_atlas_begin(&mut self.internal as *mut nk_font_atlas);
	        }
	
	        let ret = nk_font_atlas_add(&mut self.internal as *mut nk_font_atlas, &cfg.internal as *const nk_font_config);
	        
	        if !ret.is_null() && self.internal.font_num > 0 { 
	        	Some(Box::from_raw(ret as *mut _ as *mut NkFont)) 
	        } else { 
	        	None 
	        }
        }
    }

    pub fn add_font_with_bytes(&mut self, font_bytes: &[u8], font_size: f32) -> Option<Box<NkFont>> {
        let mut cfg = NkFontConfig::with_size(font_size);

        cfg.internal.ttf_size = font_bytes.len();
        cfg.internal.ttf_blob = font_bytes as *const _ as *mut c_void;
        cfg.internal.size = font_size;
        cfg.internal.ttf_data_owned_by_atlas = 1;

        self.add_font_with_config(&cfg)
    }

    pub fn bake<'a>(&'a mut self, format: NkFontAtlasFormat) -> (&'a [u8], u32, u32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        let image = unsafe {
            nk_font_atlas_bake(&mut self.internal as *mut nk_font_atlas,
                               &mut width as *mut c_int,
                               &mut height as *mut c_int,
                               format)
        };

        if width < 1 || height < 1 {
            return (&[], width as u32, height as u32);
        }

        let size = (match format {
                        NkFontAtlasFormat::NK_FONT_ATLAS_ALPHA8 => 1,
                        NkFontAtlasFormat::NK_FONT_ATLAS_RGBA32 => 4,
                    } * width * height) as usize;

        (unsafe { ::std::slice::from_raw_parts(image as *const u8, size) }, width as u32, height as u32)
    }

    pub fn end(&mut self, hnd: NkHandle, null_texture: Option<&mut NkDrawNullTexture>) {
        let nullt = match null_texture {
            Some(n) => &mut n.internal as *mut nk_draw_null_texture,
            None => ::std::ptr::null_mut(),
        };
        unsafe {
            nk_font_atlas_end(&mut self.internal as *mut nk_font_atlas,
                              hnd.internal,
                              nullt);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            nk_font_atlas_clear(&mut self.internal as *mut nk_font_atlas);
        }
    }

    fn init(&mut self, arg2: &mut NkAllocator) {
        unsafe {
            nk_font_atlas_init(&mut self.internal as *mut nk_font_atlas,
                               &mut arg2.internal as *mut nk_allocator);
        }
    }

    #[allow(dead_code)]
    fn init_custom(&mut self, persistent: &mut NkAllocator, transient: &mut NkAllocator) {
        unsafe {
            nk_font_atlas_init_custom(&mut self.internal as *mut nk_font_atlas,
                                      &mut persistent.internal as *mut nk_allocator,
                                      &mut transient.internal as *mut nk_allocator);
        }
    }

    pub fn begin(&mut self) {
        unsafe {
            nk_font_atlas_begin(&mut self.internal as *mut nk_font_atlas);
        }
    }
    
    pub fn pixels(&self) -> &[u8] {
    	unsafe {
    		::std::slice::from_raw_parts(self.internal.pixel as *const _ as *const u8, (self.internal.tex_width * self.internal.tex_height * 4) as usize)
    	}
    }
    
    pub fn tex_width(&self) -> u16 {
    	self.internal.tex_width as u16
    }
    
    pub fn tex_height(&self) -> u16 {
    	self.internal.tex_height as u16
    }
    
    pub fn custom(&self) -> NkRecti {
    	self.internal.custom
    }
    
    pub fn cursors(&self) -> &[NkCursor] {
    	unsafe {
    		::std::slice::from_raw_parts(self.internal.cursors.as_ptr() as *const NkCursor, self.internal.cursors.len())
    	}
    }
    
    pub fn glyphs(&self) -> &[NkFontGlyph] {
    	unsafe {
    		::std::slice::from_raw_parts(self.internal.glyphs as *const _ as *const NkFontGlyph, self.internal.glyph_count as usize)
    	}
    }
    
    pub fn fonts(&self) -> &[NkFont] {
    	unsafe {
    		::std::slice::from_raw_parts(self.internal.fonts as *const _ as *const NkFont, self.internal.font_num as usize)
    	}
    }
    pub fn configs(&self) -> &[NkFontConfig] {
    	unsafe {
    		::std::slice::from_raw_parts(self.internal.config as *const _ as *const NkFontConfig, self.internal.font_num as usize)
    	}
    }
}

// =============================================================================================

wrapper_type!(NkDrawNullTexture, nk_draw_null_texture);

// =============================================================================================

const DEFAULT_BUFFER_SIZE: usize = 8096;

wrapper_type!(NkBuffer, nk_buffer);

impl Drop for NkBuffer {
	fn drop(&mut self) {
		unsafe {
            nk_buffer_free(&mut self.internal);
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
            nk_buffer_init(&mut a.internal as *mut nk_buffer,
                           &mut alloc.internal as *const nk_allocator,
                           buffer_size);
        }
        a
    }

    pub fn with_fixed(memory: &mut [u8]) -> NkBuffer {
        let mut a = NkBuffer::default();
        unsafe {
            nk_buffer_init_fixed(&mut a.internal as *mut nk_buffer,
                                 memory as *mut _ as *mut ::std::os::raw::c_void,
                                 memory.len());
        }
        a
    }

    pub fn total(&mut self) -> usize {
        unsafe { nk_buffer_total(&mut self.internal as *mut nk_buffer) }
    }

    pub fn info(&mut self) -> (usize, usize, usize, usize) /*size, allocated, needed, calls*/ {
        let mut s = nk_memory_status::default();
        unsafe {
            nk_buffer_info(&mut s, &mut self.internal as *mut nk_buffer);
        }
        (s.size, s.allocated, s.needed, s.calls)
    }

    // pub fn nk_buffer_push(arg1: *mut nk_buffer,
    // type_: nk_buffer_allocation_type,
    // memory: *const ::std::os::raw::c_void,
    // size: nk_size, align: nk_size);
    // pub fn nk_buffer_mark(arg1: *mut nk_buffer,
    // type_: nk_buffer_allocation_type);
    // pub fn nk_buffer_reset(arg1: *mut nk_buffer,
    // type_: nk_buffer_allocation_type);
    // pub fn nk_buffer_clear(arg1: *mut nk_buffer);
    // pub fn nk_buffer_free(arg1: *mut nk_buffer);
    // pub fn nk_buffer_memory(arg1: *mut nk_buffer)
    // -> *mut ::std::os::raw::c_void;
    // pub fn nk_buffer_memory_const(arg1: *const nk_buffer)
    // -> *const ::std::os::raw::c_void;
    // pub fn nk_buffer_total(arg1: *mut nk_buffer) -> &Nk_size;
    //
    // pub fn nk_buffer_init(arg1: *mut nk_buffer, arg2: *const nk_allocator,
    // size: nk_size);
    // pub fn nk_buffer_init_fixed(arg1: *mut nk_buffer,
    // memory: *mut ::std::os::raw::c_void,
    // size: nk_size);
    //
}

// =============================================================================================

pub struct NkContext {
    internal: nk_context,
}

impl Default for NkContext {
    fn default() -> Self {
        NkContext { internal: nk_context::default() }
    }
}

impl Drop for NkContext {
	fn drop(&mut self) {
		unsafe {
            nk_free(&mut self.internal as *mut nk_context);
        }
	}
}

impl NkContext {
    pub fn new(alloc: &mut NkAllocator, font: &NkUserFont) -> NkContext {
        let mut a = NkContext::default();

        unsafe {
            nk_init(&mut a.internal as *mut nk_context,
                    &mut alloc.internal,
                    &font.internal);
        }

        a
    }

    pub fn input(&mut self) -> &mut NkInput {
        unsafe { ::std::mem::transmute(&mut self.internal.input) }
    }

    pub fn style(&mut self) -> &mut NkStyle {
        unsafe { ::std::mem::transmute(&mut self.internal.style) }
    }

    pub fn draw_list(&mut self) -> &mut NkDrawList {
        unsafe { ::std::mem::transmute(&mut self.internal.draw_list) }
    }

    pub fn clear(&mut self) {
        unsafe {
            nk_clear(&mut self.internal as *mut nk_context);
        }
    }

    pub fn begin(&mut self, title: NkString, bounds: NkRect, flags: NkFlags) -> bool {
        unsafe {
            nk_begin(&mut self.internal as *mut nk_context,
                     title.as_ptr(),
                     bounds,
                     flags) != 0
        }
    }

    pub fn begin_titled(&mut self, name: NkString, title: NkString, bounds: NkRect, flags: NkFlags) -> i32 {
        unsafe {
            nk_begin_titled(&mut self.internal as *mut nk_context,
                            name.as_ptr(),
                            title.as_ptr(),
                            bounds,
                            flags)
        }
    }

    pub fn end(&mut self) {
        unsafe {
            nk_end(&mut self.internal as *mut nk_context);
        }
    }

    pub fn window_find(&mut self, name: NkString) -> Option<&mut NkWindow> {
        let w = unsafe { nk_window_find(&mut self.internal as *mut nk_context, name.as_ptr()) };

        unsafe {
            if w.is_null() {
                None
            } else {
                Some(::std::mem::transmute(w))
            }
        }
    }

    pub fn window_get_bounds(&self) -> NkRect {
        unsafe { nk_window_get_bounds(&self.internal as *const nk_context) }
    }

    pub fn window_get_size(&self) -> NkVec2 {
        unsafe { nk_window_get_size(&self.internal as *const nk_context) }
    }

    pub fn window_get_position(&self) -> NkVec2 {
        unsafe { nk_window_get_position(&self.internal as *const nk_context) }
    }

    pub fn window_get_width(&self) -> f32 {
        unsafe { nk_window_get_width(&self.internal as *const nk_context) }
    }

    pub fn window_get_height(&self) -> f32 {
        unsafe { nk_window_get_height(&self.internal as *const nk_context) }
    }

    pub fn window_get_panel(&mut self) -> Option<&mut NkPanel> {
        let p = unsafe { nk_window_get_panel(&mut self.internal as *mut nk_context) };

        unsafe {
            if p.is_null() {
                None
            } else {
                Some(::std::mem::transmute(p))
            }
        }
    }

    pub fn window_get_content_region(&mut self) -> NkRect {
        unsafe { nk_window_get_content_region(&mut self.internal as *mut nk_context) }
    }

    pub fn window_get_content_region_min(&mut self) -> NkVec2 {
        unsafe { nk_window_get_content_region_min(&mut self.internal as *mut nk_context) }
    }

    pub fn window_get_content_region_max(&mut self) -> NkVec2 {
        unsafe { nk_window_get_content_region_max(&mut self.internal as *mut nk_context) }
    }

    pub fn window_get_content_region_size(&mut self) -> NkVec2 {
        unsafe { nk_window_get_content_region_size(&mut self.internal as *mut nk_context) }
    }

    pub fn window_get_canvas(&mut self) -> Option<&mut NkCommandBuffer> {
        let b = unsafe { nk_window_get_canvas(&mut self.internal as *mut nk_context) };
        unsafe {
            if b.is_null() {
                None
            } else {
                Some(::std::mem::transmute(b))
            }
        }
    }

    pub fn window_has_focus(&self) -> bool {
        unsafe { nk_window_has_focus(&self.internal as *const nk_context) > 0 }
    }

    pub fn window_is_collapsed(&mut self, name: NkString) -> bool {
        unsafe { nk_window_is_collapsed(&mut self.internal as *mut nk_context, name.as_ptr()) > 0 }
    }

    pub fn window_is_closed(&mut self, name: NkString) -> bool {
        unsafe { nk_window_is_closed(&mut self.internal as *mut nk_context, name.as_ptr()) > 0 }
    }

    pub fn window_is_hidden(&mut self, name: NkString) -> bool {
        unsafe { nk_window_is_hidden(&mut self.internal as *mut nk_context, name.as_ptr()) > 0 }
    }

    pub fn window_is_active(&mut self, name: NkString) -> bool {
        unsafe { nk_window_is_active(&mut self.internal as *mut nk_context, name.as_ptr()) > 0 }
    }

    pub fn window_is_hovered(&mut self) -> bool {
        unsafe { nk_window_is_hovered(&mut self.internal as *mut nk_context) > 0 }
    }

    pub fn window_is_any_hovered(&mut self) -> bool {
        unsafe { nk_window_is_any_hovered(&mut self.internal as *mut nk_context) > 0 }
    }

    pub fn item_is_any_active(&mut self) -> bool {
        unsafe { nk_item_is_any_active(&mut self.internal as *mut nk_context) > 0 }
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
            nk_window_collapse_if(&mut self.internal as *mut nk_context,
                                  name.as_ptr(),
                                  state,
                                  if cond { 1 } else { 0 });
        }
    }

    pub fn window_show(&mut self, name: NkString, state: NkShowState) {
        unsafe {
            nk_window_show(&mut self.internal as *mut nk_context, name.as_ptr(), state);
        }
    }

    pub fn window_show_if(&mut self, name: NkString, state: NkShowState, cond: bool) {
        unsafe {
            nk_window_show_if(&mut self.internal as *mut nk_context,
                              name.as_ptr(),
                              state,
                              if cond { 1 } else { 0 });
        }
    }

    pub fn layout_row_dynamic(&mut self, height: f32, cols: i32) {
        unsafe {
            nk_layout_row_dynamic(&mut self.internal as *mut nk_context, height, cols);
        }
    }

    pub fn layout_row_static(&mut self, height: f32, item_width: i32, cols: i32) {
        unsafe {
            nk_layout_row_static(&mut self.internal as *mut nk_context,
                                 height,
                                 item_width,
                                 cols);
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
            nk_layout_row(&mut self.internal as *mut nk_context,
                          fmt,
                          height,
                          cols_ratio.len() as i32,
                          cols_ratio.as_ptr());
        }
    }

    pub fn layout_space_begin(&mut self, fmt: NkLayoutFormat, height: f32, widget_count: i32) {
        unsafe {
            nk_layout_space_begin(&mut self.internal as *mut nk_context,
                                  fmt,
                                  height,
                                  widget_count);
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
        unsafe { nk_layout_space_bounds(&mut self.internal as *mut nk_context) }
    }

    pub fn layout_space_to_screen(&mut self, space: NkVec2) -> NkVec2 {
        unsafe { nk_layout_space_to_screen(&mut self.internal as *mut nk_context, space) }
    }

    pub fn layout_space_to_local(&mut self, space: NkVec2) -> NkVec2 {
        unsafe { nk_layout_space_to_local(&mut self.internal as *mut nk_context, space) }
    }

    pub fn layout_space_rect_to_screen(&mut self, space: NkRect) -> NkRect {
        unsafe { nk_layout_space_rect_to_screen(&mut self.internal as *mut nk_context, space) }
    }

    pub fn layout_space_rect_to_local(&mut self, space: NkRect) -> NkRect {
        unsafe { nk_layout_space_rect_to_local(&mut self.internal as *mut nk_context, space) }
    }

    pub fn layout_ratio_from_pixel(&mut self, pixel_width: f32) -> f32 {
        unsafe { nk_layout_ratio_from_pixel(&mut self.internal as *mut nk_context, pixel_width) }
    }

    pub fn nk_group_begin(&mut self, title: NkString, flags: NkFlags) -> i32 {
        unsafe { nk_group_begin(&mut self.internal as *mut nk_context, title.as_ptr(), flags) }
    }

    pub fn group_end(&mut self) {
        unsafe {
            nk_group_end(&mut self.internal as *mut nk_context);
        }
    }

    pub fn tree_push_hashed(&mut self, ty: NkTreeType, title: NkString, initial_state: NkCollapseState, hash: NkString, len: i32, seed: i32) -> i32 {
        unsafe {
            nk_tree_push_hashed(&mut self.internal as *mut nk_context,
                                ty,
                                title.as_ptr(),
                                initial_state,
                                hash.as_ptr(),
                                len,
                                seed)
        }
    }

    pub fn tree_image_push_hashed(&mut self, ty: NkTreeType, i: NkImage, title: NkString, initial_state: NkCollapseState, hash: NkString, len: i32, seed: i32) -> i32 {
        unsafe {
            nk_tree_image_push_hashed(&mut self.internal as *mut nk_context,
                                      ty,
                                      i.internal,
                                      title.as_ptr(),
                                      initial_state,
                                      hash.as_ptr(),
                                      len,
                                      seed)
        }
    }

    pub fn tree_pop(&mut self) {
        unsafe {
            nk_tree_pop(&mut self.internal as *mut nk_context);
        }
    }

    pub fn text(&mut self, text: &str, flags: NkFlags) {
        unsafe {
            nk_text(&mut self.internal as *mut nk_context,
                    text.as_ptr() as *const i8,
                    text.as_bytes().len() as i32,
                    flags);
        }
    }

    pub fn text_colored(&mut self, text: &str, flags: NkFlags, color: NkColor) {
        unsafe {
            nk_text_colored(&mut self.internal as *mut nk_context,
                            text.as_ptr() as *const i8,
                            text.as_bytes().len() as i32,
                            flags,
                            color);
        }
    }

    pub fn text_wrap(&mut self, text: &str) {
        unsafe {
            nk_text_wrap(&mut self.internal as *mut nk_context,
                         text.as_ptr() as *const i8,
                         text.as_bytes().len() as i32);
        }
    }

    pub fn text_wrap_colored(&mut self, text: &str, color: NkColor) {
        unsafe {
            nk_text_wrap_colored(&mut self.internal as *mut nk_context,
                                 text.as_ptr() as *const i8,
                                 text.as_bytes().len() as i32,
                                 color);
        }
    }

    pub fn label(&mut self, text: NkString, flags: NkFlags) {
        unsafe {
            nk_label(&mut self.internal as *mut nk_context, text.as_ptr(), flags);
        }
    }

    pub fn label_colored(&mut self, text: NkString, flags: NkFlags, color: NkColor) {
        unsafe {
            nk_label_colored(&mut self.internal as *mut nk_context,
                             text.as_ptr(),
                             flags,
                             color);
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
            nk_button_text(&mut self.internal as *mut nk_context,
                           text.as_ptr() as *const i8,
                           text.as_bytes().len() as i32) != 0
        }
    }

    pub fn button_label(&mut self, title: NkString) -> bool {
        unsafe { nk_button_label(&mut self.internal as *mut nk_context, title.as_ptr()) != 0 }
    }

    pub fn button_color(&mut self, color: NkColor) -> bool {
        unsafe { nk_button_color(&mut self.internal as *mut nk_context, color) != 0 }
    }

    pub fn button_symbol(&mut self, ty: NkSymbolType) -> bool {
        unsafe { nk_button_symbol(&mut self.internal as *mut nk_context, ty) != 0 }
    }

    pub fn button_image(&mut self, img: NkImage) -> bool {
        unsafe { nk_button_image(&mut self.internal as *mut nk_context, img.internal) != 0 }
    }

    pub fn button_symbol_label(&mut self, ty: NkSymbolType, title: NkString, text_alignment: NkFlags) -> bool {
        unsafe {
            nk_button_symbol_label(&mut self.internal as *mut nk_context,
                                   ty,
                                   title.as_ptr(),
                                   text_alignment) != 0
        }
    }

    pub fn button_symbol_text(&mut self, ty: NkSymbolType, title: &str, text_alignment: NkFlags) -> bool {
        unsafe {
            nk_button_symbol_text(&mut self.internal as *mut nk_context,
                                  ty,
                                  title.as_ptr() as *const i8,
                                  title.as_bytes().len() as i32,
                                  text_alignment) != 0
        }
    }

    pub fn button_image_label(&mut self, img: NkImage, title: NkString, text_alignment: NkFlags) -> bool {
        unsafe {
            nk_button_image_label(&mut self.internal as *mut nk_context,
                                  img.internal,
                                  title.as_ptr(),
                                  text_alignment) != 0
        }
    }

    pub fn button_image_text(&mut self, img: NkImage, title: &str, text_alignment: NkFlags) -> bool {
        unsafe {
            nk_button_image_text(&mut self.internal as *mut nk_context,
                                 img.internal,
                                 title.as_ptr() as *const i8,
                                 title.as_bytes().len() as i32,
                                 text_alignment) != 0
        }
    }

    pub fn button_set_behavior(&mut self, b: NkButtonBehavior) {
        unsafe {
            nk_button_set_behavior(&mut self.internal as *mut nk_context, b);
        }
    }

    pub fn button_push_behavior(&mut self, b: NkButtonBehavior) -> i32 {
        unsafe { nk_button_push_behavior(&mut self.internal as *mut nk_context, b) }
    }

    pub fn button_pop_behavior(&mut self) -> i32 {
        unsafe { nk_button_pop_behavior(&mut self.internal as *mut nk_context) }
    }

    pub fn check_label(&mut self, title: NkString, active: bool) -> i32 {
        unsafe {
            nk_check_label(&mut self.internal as *mut nk_context,
                           title.as_ptr(),
                           if active { 1 } else { 0 })
        }
    }

    pub fn check_text(&mut self, title: &str, active: bool) -> i32 {
        unsafe {
            nk_check_text(&mut self.internal as *mut nk_context,
                          title.as_ptr() as *const i8,
                          title.as_bytes().len() as i32,
                          if active { 1 } else { 0 })
        }
    }

    pub fn check_flags_label(&mut self, title: NkString, flags: u32, value: u32) -> u32 {
        unsafe {
            nk_check_flags_label(&mut self.internal as *mut nk_context,
                                 title.as_ptr(),
                                 flags,
                                 value)
        }
    }

    pub fn check_flags_text(&mut self, title: &str, flags: u32, value: u32) -> u32 {
        unsafe {
            nk_check_flags_text(&mut self.internal as *mut nk_context,
                                title.as_ptr() as *const i8,
                                title.as_bytes().len() as i32,
                                flags,
                                value)
        }
    }

    pub fn checkbox_label(&mut self, title: NkString, active: &mut bool) -> bool {
        let mut i = if *active { 1 } else { 0 };
        let r = unsafe {
            nk_checkbox_label(&mut self.internal as *mut nk_context,
                              title.as_ptr(),
                              &mut i as *mut i32) != 0
        };

        *active = i != 0;
        r
    }

    pub fn checkbox_text(&mut self, title: &str, active: &mut bool) -> bool {
        let mut i = if *active { 1 } else { 0 };
        let r = unsafe {
            nk_checkbox_text(&mut self.internal as *mut nk_context,
                             title.as_ptr() as *const i8,
                             title.as_bytes().len() as i32,
                             &mut i as *mut i32) != 0
        };

        *active = i != 0;
        r
    }

    pub fn checkbox_flags_label(&mut self, title: NkString, flags: &mut u32, value: u32) -> bool {
        unsafe {
            nk_checkbox_flags_label(&mut self.internal as *mut nk_context,
                                    title.as_ptr(),
                                    flags,
                                    value) != 0
        }
    }

    pub fn checkbox_flags_text(&mut self, title: &str, flags: &mut u32, value: u32) -> bool {
        unsafe {
            nk_checkbox_flags_text(&mut self.internal as *mut nk_context,
                                   title.as_ptr() as *const i8,
                                   title.as_bytes().len() as i32,
                                   flags,
                                   value) != 0
        }
    }

    pub fn radio_label(&mut self, title: NkString, active: &mut bool) -> bool {
        let mut i = if *active { 1 } else { 0 };
        let r = unsafe {
            nk_radio_label(&mut self.internal as *mut nk_context,
                           title.as_ptr(),
                           &mut i as *mut i32) != 0
        };

        *active = i != 0;
        r
    }

    pub fn radio_text(&mut self, title: &str, active: &mut bool) -> bool {
        let mut i = if *active { 1 } else { 0 };
        let r = unsafe {
            nk_radio_text(&mut self.internal as *mut nk_context,
                          title.as_ptr() as *const i8,
                          title.as_bytes().len() as i32,
                          &mut i as *mut i32) != 0
        };

        *active = i != 0;
        r
    }

    pub fn option_label(&mut self, title: NkString, active: bool) -> bool {
        unsafe {
            nk_option_label(&mut self.internal as *mut nk_context,
                            title.as_ptr(),
                            if active { 1 } else { 0 }) > 0
        }
    }

    pub fn option_text(&mut self, title: &str, active: bool) -> bool {
        unsafe {
            nk_option_text(&mut self.internal as *mut nk_context,
                           title.as_ptr() as *const i8,
                           title.as_bytes().len() as i32,
                           if active { 1 } else { 0 }) > 0
        }
    }

    pub fn selectable_label(&mut self, title: NkString, align: NkFlags, value: &mut i32) -> bool {
        unsafe {
            nk_selectable_label(&mut self.internal as *mut nk_context,
                                title.as_ptr(),
                                align,
                                value) != 0
        }
    }

    pub fn selectable_text(&mut self, title: &str, align: NkFlags, value: &mut i32) -> bool {
        unsafe {
            nk_selectable_text(&mut self.internal as *mut nk_context,
                               title.as_ptr() as *const i8,
                               title.as_bytes().len() as i32,
                               align,
                               value) != 0
        }
    }

    pub fn selectable_image_label(&mut self, img: NkImage, title: NkString, align: NkFlags, value: &mut i32) -> bool {
        unsafe {
            nk_selectable_image_label(&mut self.internal as *mut nk_context,
                                      img.internal,
                                      title.as_ptr(),
                                      align,
                                      value) != 0
        }
    }

    pub fn selectable_image_text(&mut self, img: NkImage, title: &str, align: NkFlags, value: &mut i32) -> bool {
        unsafe {
            nk_selectable_image_text(&mut self.internal as *mut nk_context,
                                     img.internal,
                                     title.as_ptr() as *const i8,
                                     title.as_bytes().len() as i32,
                                     align,
                                     value) != 0
        }
    }

    pub fn select_label(&mut self, title: NkString, align: NkFlags, value: i32) -> i32 {
        unsafe {
            nk_select_label(&mut self.internal as *mut nk_context,
                            title.as_ptr(),
                            align,
                            value)
        }
    }

    pub fn select_text(&mut self, title: &str, align: NkFlags, value: i32) -> i32 {
        unsafe {
            nk_select_text(&mut self.internal as *mut nk_context,
                           title.as_ptr() as *const i8,
                           title.as_bytes().len() as i32,
                           align,
                           value)
        }
    }

    pub fn select_image_label(&mut self, img: NkImage, title: NkString, align: NkFlags, value: i32) -> i32 {
        unsafe {
            nk_select_image_label(&mut self.internal as *mut nk_context,
                                  img.internal,
                                  title.as_ptr(),
                                  align,
                                  value)
        }
    }

    pub fn select_image_text(&mut self, img: NkImage, title: &str, align: NkFlags, value: i32) -> i32 {
        unsafe {
            nk_select_image_text(&mut self.internal as *mut nk_context,
                                 img.internal,
                                 title.as_ptr() as *const i8,
                                 title.as_bytes().len() as i32,
                                 align,
                                 value)
        }
    }

    pub fn slide_float(&mut self, min: f32, val: f32, max: f32, step: f32) -> f32 {
        unsafe { nk_slide_float(&mut self.internal as *mut nk_context, min, val, max, step) }
    }

    pub fn slide_int(&mut self, min: i32, val: i32, max: i32, step: i32) -> i32 {
        unsafe { nk_slide_int(&mut self.internal as *mut nk_context, min, val, max, step) }
    }

    pub fn slider_float(&mut self, min: f32, val: &mut f32, max: f32, step: f32) -> bool {
        unsafe { nk_slider_float(&mut self.internal as *mut nk_context, min, val, max, step) != 0 }
    }

    pub fn slider_int(&mut self, min: i32, val: &mut i32, max: i32, step: i32) -> bool {
        unsafe { nk_slider_int(&mut self.internal as *mut nk_context, min, val, max, step) != 0 }
    }

    pub fn progress(&mut self, cur: &mut usize, max: usize, is_modifyable: bool) -> bool {
        unsafe {
            nk_progress(&mut self.internal as *mut nk_context,
                        cur,
                        max,
                        if is_modifyable { 1 } else { 0 }) != 0
        }
    }

    pub fn prog(&mut self, cur: usize, max: usize, is_modifyable: bool) -> usize {
        unsafe {
            nk_prog(&mut self.internal as *mut nk_context,
                    cur,
                    max,
                    if is_modifyable { 1 } else { 0 })
        }
    }

    pub fn color_picker(&mut self, color: NkColor, fmt: NkColorFormat) -> NkColor {
        unsafe { nk_color_picker(&mut self.internal as *mut nk_context, color, fmt) }
    }

    pub fn color_pick(&mut self, fmt: NkColorFormat) -> (bool, NkColor) {
        let mut c = NkColor::default();
        let changed = unsafe {
            nk_color_pick(&mut self.internal as *mut nk_context,
                          &mut c as *mut nk_color,
                          fmt)
        };
        (changed != 0, c)
    }

    pub fn property_int(&mut self, name: NkString, min: i32, val: &mut i32, max: i32, step: i32, inc_per_pixel: f32) {
        unsafe {
            nk_property_int(&mut self.internal as *mut nk_context,
                            name.as_ptr(),
                            min,
                            val,
                            max,
                            step,
                            inc_per_pixel);
        }
    }

    pub fn property_float(&mut self, name: NkString, min: f32, val: &mut f32, max: f32, step: f32, inc_per_pixel: f32) {
        unsafe {
            nk_property_float(&mut self.internal as *mut nk_context,
                              name.as_ptr(),
                              min,
                              val,
                              max,
                              step,
                              inc_per_pixel)
        }
    }

    pub fn property_double(&mut self, name: NkString, min: f64, val: &mut f64, max: f64, step: f64, inc_per_pixel: f32) {
        unsafe {
            nk_property_double(&mut self.internal as *mut nk_context,
                               name.as_ptr(),
                               min,
                               val,
                               max,
                               step,
                               inc_per_pixel)
        }
    }

    pub fn propertyi(&mut self, name: NkString, min: i32, val: i32, max: i32, step: i32, inc_per_pixel: f32) -> i32 {
        unsafe {
            nk_propertyi(&mut self.internal as *mut nk_context,
                         name.as_ptr(),
                         min,
                         val,
                         max,
                         step,
                         inc_per_pixel)
        }
    }

    pub fn propertyf(&mut self, name: NkString, min: f32, val: f32, max: f32, step: f32, inc_per_pixel: f32) -> f32 {
        unsafe {
            nk_propertyf(&mut self.internal as *mut nk_context,
                         name.as_ptr(),
                         min,
                         val,
                         max,
                         step,
                         inc_per_pixel)
        }
    }

    pub fn propertyd(&mut self, name: NkString, min: f64, val: f64, max: f64, step: f64, inc_per_pixel: f32) -> f64 {
        unsafe {
            nk_propertyd(&mut self.internal as *mut nk_context,
                         name.as_ptr(),
                         min,
                         val,
                         max,
                         step,
                         inc_per_pixel)
        }
    }

    pub fn edit_string_custom_filter(&mut self, flags: NkFlags, buffer: &mut [u8], len: &mut i32, filter: fn(&NkTextEdit, char) -> bool) -> NkFlags {
        unsafe {
            CUSTOM_EDIT_FILTER = Some(filter);
            nk_edit_string(&mut self.internal as *mut nk_context,
                           flags,
                           &mut buffer[0] as *mut _ as *mut i8,
                           len,
                           buffer.len() as i32,
                           Some(nk_filter_custom))
        }
    }

    pub fn edit_string(&mut self, flags: NkFlags, buffer: &mut [u8], len: &mut i32, filter: NkPluginFilter) -> NkFlags {
        unsafe {
            nk_edit_string(&mut self.internal as *mut nk_context,
                           flags,
                           &mut buffer[0] as *mut _ as *mut i8,
                           len,
                           buffer.len() as i32,
                           filter)
        }
    }

    pub fn edit_buffer(&mut self, flags: NkFlags, editor: &mut NkTextEdit, filter: NkPluginFilter) -> NkFlags {
        unsafe {
            nk_edit_buffer(&mut self.internal as *mut nk_context,
                           flags,
                           &mut editor.internal,
                           filter)
        }
    }

    pub fn chart_begin(&mut self, ty: NkChartType, num: i32, min: f32, max: f32) -> bool {
        unsafe { nk_chart_begin(&mut self.internal as *mut nk_context, ty, num, min, max) > 0 }
    }

    pub fn chart_begin_colored(&mut self, ty: NkChartType, color: NkColor, active: NkColor, num: i32, min: f32, max: f32) -> bool {
        unsafe {
            nk_chart_begin_colored(&mut self.internal as *mut nk_context,
                                   ty,
                                   color,
                                   active,
                                   num,
                                   min,
                                   max) > 0
        }
    }

    pub fn chart_add_slot(&mut self, ty: NkChartType, count: i32, min_value: f32, max_value: f32) {
        unsafe {
            nk_chart_add_slot(&mut self.internal as *mut nk_context,
                              ty,
                              count,
                              min_value,
                              max_value);
        }
    }

    pub fn chart_add_slot_colored(&mut self, ty: NkChartType, color: NkColor, active: NkColor, count: i32, min_value: f32, max_value: f32) {
        unsafe {
            nk_chart_add_slot_colored(&mut self.internal as *mut nk_context,
                                      ty,
                                      color,
                                      active,
                                      count,
                                      min_value,
                                      max_value);
        }
    }

    pub fn chart_push(&mut self, value: f32) -> NkFlags {
        unsafe { nk_chart_push(&mut self.internal as *mut nk_context, value) }
    }

    pub fn chart_push_slot(&mut self, value: f32, count: i32) -> NkFlags {
        unsafe { nk_chart_push_slot(&mut self.internal as *mut nk_context, value, count) }
    }

    pub fn chart_end(&mut self) {
        unsafe {
            nk_chart_end(&mut self.internal as *mut nk_context);
        }
    }

    pub fn plot(&mut self, ty: NkChartType, values: &[f32]) {
        unsafe {
            nk_plot(&mut self.internal as *mut nk_context,
                    ty,
                    values.as_ptr(),
                    values.len() as i32,
                    0);
        }
    }

    // pub fn plot_function(&mut self, ty: NkChartType, userdata: &[f32], offset: i32) {
    // unsafe {
    // nk_plot_function(&mut self.internal as *mut nk_context, ty, userdata as *const _ as *mut ::std::os::raw::c_void, Some(nk_plot_value_getter_custom), userdata.len() as i32, offset);
    // }
    // }

    pub fn popup_begin(&mut self, ty: NkPopupType, title: NkString, flags: NkFlags, bounds: NkRect) -> bool {
        unsafe {
            nk_popup_begin(&mut self.internal as *mut nk_context,
                           ty,
                           title.as_ptr(),
                           flags,
                           bounds) > 0
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

    pub fn combo(&mut self, items: &mut NkStringArray, selected: i32, item_height: i32, size: NkVec2) -> i32 {
        unsafe {
            nk_combo(&mut self.internal as *mut nk_context,
                     items.as_mut(),
                     items.len() as i32,
                     selected,
                     item_height,
                     size)
        }
    }

    pub fn combo_separator(&mut self, items_separated_by_separator: NkString, separator: char, selected: i32, item_height: i32, size: NkVec2) -> i32 {
        let len = String::from_utf8_lossy(items_separated_by_separator.bytes.as_ref())
            .as_ref()
            .split(separator)
            .count();
        unsafe {
            nk_combo_separator(&mut self.internal as *mut nk_context,
                               items_separated_by_separator.as_ptr(),
                               separator as i32,
                               selected,
                               len as i32,
                               item_height,
                               size)
        }
    }

    pub fn combo_begin_label(&mut self, selected: NkString, size: NkVec2) -> bool {
        unsafe {
            nk_combo_begin_label(&mut self.internal as *mut nk_context,
                                 selected.as_ptr(),
                                 size) > 0
        }
    }

    pub fn combo_begin_text(&mut self, selected: &str, size: NkVec2) -> bool {
        unsafe {
            nk_combo_begin_text(&mut self.internal as *mut nk_context,
                                selected.as_ptr() as *const i8,
                                selected.as_bytes().len() as i32,
                                size) > 0
        }
    }

    pub fn combo_begin_color(&mut self, color: NkColor, size: NkVec2) -> bool {
        unsafe { nk_combo_begin_color(&mut self.internal as *mut nk_context, color, size) > 0 }
    }

    pub fn combo_begin_symbol(&mut self, sym: NkSymbolType, size: NkVec2) -> bool {
        unsafe { nk_combo_begin_symbol(&mut self.internal as *mut nk_context, sym, size) > 0 }
    }

    pub fn combo_begin_symbol_label(&mut self, label: NkString, sym: NkSymbolType, size: NkVec2) -> bool {
        unsafe {
            nk_combo_begin_symbol_label(&mut self.internal as *mut nk_context,
                                        label.as_ptr(),
                                        sym,
                                        size) > 0
        }
    }

    pub fn combo_begin_symbol_text(&mut self, label: &str, sym: NkSymbolType, size: NkVec2) -> bool {
        unsafe {
            nk_combo_begin_symbol_text(&mut self.internal as *mut nk_context,
                                       label.as_ptr() as *const i8,
                                       label.as_bytes().len() as i32,
                                       sym,
                                       size) > 0
        }
    }

    pub fn combo_begin_image(&mut self, img: NkImage, size: NkVec2) -> bool {
        unsafe { nk_combo_begin_image(&mut self.internal as *mut nk_context, img.internal, size) > 0 }
    }

    pub fn combo_begin_image_label(&mut self, label: NkString, img: NkImage, size: NkVec2) -> bool {
        unsafe {
            nk_combo_begin_image_label(&mut self.internal as *mut nk_context,
                                       label.as_ptr(),
                                       img.internal,
                                       size) > 0
        }
    }

    pub fn combo_begin_image_text(&mut self, label: &str, img: NkImage, size: NkVec2) -> bool {
        unsafe {
            nk_combo_begin_image_text(&mut self.internal as *mut nk_context,
                                      label.as_ptr() as *const i8,
                                      label.as_bytes().len() as i32,
                                      img.internal,
                                      size) > 0
        }
    }

    pub fn combo_item_label(&mut self, label: NkString, alignment: NkFlags) -> bool {
        unsafe {
            nk_combo_item_label(&mut self.internal as *mut nk_context,
                                label.as_ptr(),
                                alignment) > 0
        }
    }

    pub fn combo_item_text(&mut self, label: &str, alignment: NkFlags) -> bool {
        unsafe {
            nk_combo_item_text(&mut self.internal as *mut nk_context,
                               label.as_ptr() as *const i8,
                               label.as_bytes().len() as i32,
                               alignment) > 0
        }
    }

    pub fn combo_item_image_label(&mut self, img: NkImage, label: NkString, alignment: NkFlags) -> bool {
        unsafe {
            nk_combo_item_image_label(&mut self.internal as *mut nk_context,
                                      img.internal,
                                      label.as_ptr(),
                                      alignment) > 0
        }
    }

    pub fn combo_item_image_text(&mut self, img: NkImage, label: &str, alignment: NkFlags) -> bool {
        unsafe {
            nk_combo_item_image_text(&mut self.internal as *mut nk_context,
                                     img.internal,
                                     label.as_ptr() as *const i8,
                                     label.as_bytes().len() as i32,
                                     alignment) > 0
        }
    }

    pub fn combo_item_symbol_label(&mut self, sym: NkSymbolType, label: NkString, alignment: NkFlags) -> bool {
        unsafe {
            nk_combo_item_symbol_label(&mut self.internal as *mut nk_context,
                                       sym,
                                       label.as_ptr(),
                                       alignment) > 0
        }
    }

    pub fn combo_item_symbol_text(&mut self, sym: NkSymbolType, label: &str, alignment: NkFlags) -> bool {
        unsafe {
            nk_combo_item_symbol_text(&mut self.internal as *mut nk_context,
                                      sym,
                                      label.as_ptr() as *const i8,
                                      label.as_bytes().len() as i32,
                                      alignment) > 0
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

    pub fn contextual_begin(&mut self, flags: NkFlags, bounds: NkVec2, trigger_bounds: NkRect) -> bool {
        unsafe {
            nk_contextual_begin(&mut self.internal as *mut nk_context,
                                flags,
                                bounds,
                                trigger_bounds) > 0
        }
    }

    pub fn contextual_item_label(&mut self, label: NkString, align: NkFlags) -> bool {
        unsafe { nk_contextual_item_label(&mut self.internal as *mut nk_context, label.as_ptr(), align) > 0 }
    }

    pub fn contextual_item_text(&mut self, label: &str, align: NkFlags) -> bool {
        unsafe {
            nk_contextual_item_text(&mut self.internal as *mut nk_context,
                                    label.as_ptr() as *const i8,
                                    label.as_bytes().len() as i32,
                                    align) > 0
        }
    }

    pub fn contextual_item_image_label(&mut self, img: NkImage, label: NkString, align: NkFlags) -> bool {
        unsafe {
            nk_contextual_item_image_label(&mut self.internal as *mut nk_context,
                                           img.internal,
                                           label.as_ptr(),
                                           align) > 0
        }
    }

    pub fn contextual_item_image_text(&mut self, img: NkImage, label: &str, align: NkFlags) -> bool {
        unsafe {
            nk_contextual_item_image_text(&mut self.internal as *mut nk_context,
                                          img.internal,
                                          label.as_ptr() as *const i8,
                                          label.as_bytes().len() as i32,
                                          align) > 0
        }
    }

    pub fn contextual_item_symbol_label(&mut self, sym: NkSymbolType, label: NkString, align: NkFlags) -> bool {
        unsafe {
            nk_contextual_item_symbol_label(&mut self.internal as *mut nk_context,
                                            sym,
                                            label.as_ptr(),
                                            align) > 0
        }
    }

    pub fn contextual_item_symbol_text(&mut self, sym: NkSymbolType, label: &str, align: NkFlags) -> bool {
        unsafe {
            nk_contextual_item_symbol_text(&mut self.internal as *mut nk_context,
                                           sym,
                                           label.as_ptr() as *const i8,
                                           label.as_bytes().len() as i32,
                                           align) > 0
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

    pub fn tooltip_begin(&mut self, width: f32) -> bool {
        unsafe { nk_tooltip_begin(&mut self.internal as *mut nk_context, width) > 0 }
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

    pub fn menu_begin_label(&mut self, title: NkString, align: NkFlags, size: NkVec2) -> bool {
        unsafe {
            nk_menu_begin_label(&mut self.internal as *mut nk_context,
                                title.as_ptr(),
                                align,
                                size) > 0
        }
    }

    pub fn menu_begin_text(&mut self, title: &str, align: NkFlags, size: NkVec2) -> bool {
        unsafe {
            nk_menu_begin_text(&mut self.internal as *mut nk_context,
                               title.as_ptr() as *const i8,
                               title.len() as i32,
                               align,
                               size) > 0
        }
    }

    pub fn menu_begin_image(&mut self, title: NkString, img: NkImage, size: NkVec2) -> bool {
        unsafe {
            nk_menu_begin_image(&mut self.internal as *mut nk_context,
                                title.as_ptr(),
                                img.internal,
                                size) > 0
        }
    }

    pub fn menu_begin_image_label(&mut self, title: NkString, align: NkFlags, img: NkImage, size: NkVec2) -> bool {
        unsafe {
            nk_menu_begin_image_label(&mut self.internal as *mut nk_context,
                                      title.as_ptr(),
                                      align,
                                      img.internal,
                                      size) > 0
        }
    }

    pub fn menu_begin_image_text(&mut self, title: &str, align: NkFlags, img: NkImage, size: NkVec2) -> bool {
        unsafe {
            nk_menu_begin_image_text(&mut self.internal as *mut nk_context,
                                     title.as_ptr() as *const i8,
                                     title.len() as i32,
                                     align,
                                     img.internal,
                                     size) > 0
        }
    }

    pub fn menu_begin_symbol(&mut self, title: NkString, sym: NkSymbolType, size: NkVec2) -> bool {
        unsafe {
            nk_menu_begin_symbol(&mut self.internal as *mut nk_context,
                                 title.as_ptr(),
                                 sym,
                                 size) > 0
        }
    }

    pub fn menu_begin_symbol_label(&mut self, title: NkString, align: NkFlags, sym: NkSymbolType, size: NkVec2) -> bool {
        unsafe {
            nk_menu_begin_symbol_label(&mut self.internal as *mut nk_context,
                                       title.as_ptr(),
                                       align,
                                       sym,
                                       size) > 0
        }
    }

    pub fn menu_begin_symbol_text(&mut self, title: &str, align: NkFlags, sym: NkSymbolType, size: NkVec2) -> bool {
        unsafe {
            nk_menu_begin_symbol_text(&mut self.internal as *mut nk_context,
                                      title.as_ptr() as *const i8,
                                      title.len() as i32,
                                      align,
                                      sym,
                                      size) > 0
        }
    }

    pub fn menu_item_label(&mut self, title: NkString, align: NkFlags) -> bool {
        unsafe { nk_menu_item_label(&mut self.internal as *mut nk_context, title.as_ptr(), align) > 0 }
    }

    pub fn menu_item_text(&mut self, title: &str, align: NkFlags) -> bool {
        unsafe {
            nk_menu_item_text(&mut self.internal as *mut nk_context,
                              title.as_ptr() as *const i8,
                              title.len() as i32,
                              align) > 0
        }
    }

    pub fn menu_item_image_label(&mut self, img: NkImage, title: NkString, align: NkFlags) -> bool {
        unsafe {
            nk_menu_item_image_label(&mut self.internal as *mut nk_context,
                                     img.internal,
                                     title.as_ptr(),
                                     align) > 0
        }
    }

    pub fn menu_item_image_text(&mut self, img: NkImage, title: &str, align: NkFlags) -> bool {
        unsafe {
            nk_menu_item_image_text(&mut self.internal as *mut nk_context,
                                    img.internal,
                                    title.as_ptr() as *const i8,
                                    title.len() as i32,
                                    align) > 0
        }
    }

    pub fn menu_item_symbol_label(&mut self, sym: NkSymbolType, title: NkString, align: NkFlags) -> bool {
        unsafe {
            nk_menu_item_symbol_label(&mut self.internal as *mut nk_context,
                                      sym,
                                      title.as_ptr(),
                                      align) > 0
        }
    }

    pub fn menu_item_symbol_text(&mut self, sym: NkSymbolType, title: &str, align: NkFlags) -> bool {
        unsafe {
            nk_menu_item_symbol_text(&mut self.internal as *mut nk_context,
                                     sym,
                                     title.as_ptr() as *const i8,
                                     title.len() as i32,
                                     align) > 0
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
            nk_convert(&mut self.internal as *mut nk_context,
                       &mut cmds.internal as *mut nk_buffer,
                       &mut vertices.internal as *mut nk_buffer,
                       &mut elements.internal as *mut nk_buffer,
                       &config.internal as *const nk_convert_config);
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
            nk_input_key(&mut self.internal as *mut nk_context,
                         key,
                         if down { 1 } else { 0 });
        }
    }

    pub fn input_button(&mut self, b: NkButton, x: i32, y: i32, down: bool) {
        unsafe {
            nk_input_button(&mut self.internal as *mut nk_context,
                            b,
                            x,
                            y,
                            if down { 1 } else { 0 });
        }
    }

    pub fn input_scroll(&mut self, y: f32) {
        unsafe {
            nk_input_scroll(&mut self.internal as *mut nk_context, y);
        }
    }

    pub fn input_char(&mut self, c: u8) {
        unsafe {
            nk_input_char(&mut self.internal as *mut nk_context, c as i8);
        }
    }

    pub fn input_glyph(&mut self, g: NkGlyph) {
        unsafe {
            nk_input_glyph(&mut self.internal as *mut nk_context,
                           &g[0] as *const _ as *mut i8);
        }
    }

    pub fn input_unicode(&mut self, r: char) {
        unsafe {
            nk_input_unicode(&mut self.internal as *mut nk_context, r as u32);
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

    pub fn style_from_table(&mut self, table: &NkColorMap) {
        unsafe {
            nk_style_from_table(&mut self.internal as *mut nk_context,
                                &table.internal[0] as *const nk_color);
        }
    }

    pub fn style_load_cursor(&mut self, cur: NkStyleCursor, res: &NkCursor) {
        unsafe {
            nk_style_load_cursor(&mut self.internal as *mut nk_context, cur, &res.internal);
        }
    }

    pub fn style_load_all_cursors(&mut self, table: &mut NkCursorMap) {
        unsafe {
            nk_style_load_all_cursors(&mut self.internal as *mut nk_context,
                                      table.internal.as_mut_ptr() as *mut nk_cursor);
        }
    }

    pub fn style_set_font(&mut self, font: &NkUserFont) {
        unsafe {
            nk_style_set_font(&mut self.internal as *mut nk_context, &font.internal);
        }
    }

    pub fn style_set_cursor(&mut self, cur: NkStyleCursor) -> bool {
        unsafe { nk_style_set_cursor(&mut self.internal as *mut nk_context, cur) > 0 }
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
        unsafe { nk_style_push_font(&mut self.internal as *mut nk_context, &mut font.internal) > 0 }
    }

    pub fn style_push_float(&mut self, addr: &mut f32, val: f32) -> bool {
        unsafe { nk_style_push_float(&mut self.internal as *mut nk_context, addr as *mut f32, val) > 0 }
    }

    pub fn style_push_vec2(&mut self, addr: &mut NkVec2, val: NkVec2) -> bool {
        unsafe {
            nk_style_push_vec2(&mut self.internal as *mut nk_context,
                               addr as *mut nk_vec2,
                               val) > 0
        }
    }

    pub fn style_push_style_item(&mut self, addr: &mut NkStyleItem, val: NkStyleItem) -> bool {
        unsafe {
            nk_style_push_style_item(&mut self.internal as *mut nk_context,
                                     &mut addr.internal as *mut nk_style_item,
                                     val.internal) > 0
        }
    }

    pub fn style_push_flags(&mut self, addr: &mut NkFlags, val: NkFlags) -> bool {
        unsafe {
            nk_style_push_flags(&mut self.internal as *mut nk_context,
                                addr as *mut nk_flags,
                                val) > 0
        }
    }

    pub fn style_push_color(&mut self, addr: &mut NkColor, val: NkColor) -> bool {
        unsafe {
            nk_style_push_color(&mut self.internal as *mut nk_context,
                                addr as *mut nk_color,
                                val) > 0
        }
    }

    pub fn style_pop_font(&mut self) -> bool {
        unsafe { nk_style_pop_font(&mut self.internal as *mut nk_context) > 0 }
    }

    pub fn style_pop_float(&mut self) -> bool {
        unsafe { nk_style_pop_float(&mut self.internal as *mut nk_context) > 0 }
    }

    pub fn style_pop_vec2(&mut self) -> bool {
        unsafe { nk_style_pop_vec2(&mut self.internal as *mut nk_context) > 0 }
    }

    pub fn style_pop_style_item(&mut self) -> bool {
        unsafe { nk_style_pop_style_item(&mut self.internal as *mut nk_context) > 0 }
    }

    pub fn style_pop_flags(&mut self) -> bool {
        unsafe { nk_style_pop_flags(&mut self.internal as *mut nk_context) > 0 }
    }

    pub fn style_pop_color(&mut self) -> bool {
        unsafe { nk_style_pop_color(&mut self.internal as *mut nk_context) > 0 }
    }

    pub fn widget_bounds(&mut self) -> NkRect {
        unsafe { nk_widget_bounds(&mut self.internal as *mut nk_context) }
    }

    pub fn widget_position(&mut self) -> NkVec2 {
        unsafe { nk_widget_position(&mut self.internal as *mut nk_context) }
    }

    pub fn widget_size(&mut self) -> NkVec2 {
        unsafe { nk_widget_size(&mut self.internal as *mut nk_context) }
    }

    pub fn widget_width(&mut self) -> f32 {
        unsafe { nk_widget_width(&mut self.internal as *mut nk_context) }
    }
    pub fn widget_height(&mut self) -> f32 {
        unsafe { nk_widget_height(&mut self.internal as *mut nk_context) }
    }

    pub fn widget_is_hovered(&mut self) -> bool {
        unsafe { nk_widget_is_hovered(&mut self.internal as *mut nk_context) > 0 }
    }

    pub fn widget_is_mouse_clicked(&mut self, b: NkButton) -> bool {
        unsafe { nk_widget_is_mouse_clicked(&mut self.internal as *mut nk_context, b) > 0 }
    }

    pub fn widget_has_mouse_click_down(&mut self, b: NkButton, down: bool) -> bool {
        unsafe {
            nk_widget_has_mouse_click_down(&mut self.internal as *mut nk_context,
                                           b,
                                           if down { 1 } else { 0 }) > 0
        }
    }

    pub fn widget(&self, arg1: &mut NkRect) -> NkWidgetLayoutState {
        unsafe { nk_widget(arg1, &self.internal as *const nk_context) }
    }

    pub fn spacing(&mut self, cols: i32) {
        unsafe {
            nk_spacing(&mut self.internal as *mut nk_context, cols);
        }
    }

    pub fn draw_begin(&self, buf: &NkBuffer) -> Option<&NkDrawCommand> {
        let n = unsafe { nk__draw_begin(&self.internal, &buf.internal) };

        unsafe {
            if n.is_null() {
                None
            } else {
                Some(::std::mem::transmute(n))
            }
        }
    }
    pub fn draw_next<'a>(&self, prev: &NkDrawCommand, buf: &NkBuffer) -> Option<&'a NkDrawCommand> {
        let n = unsafe { nk__draw_next(&prev.internal, &buf.internal, &self.internal) };

        unsafe {
            if n.is_null() {
                None
            } else {
                Some(::std::mem::transmute(n))
            }
        }
    }

    pub fn next_cmd<'a, 'b>(&self, arg2: &'b NkCommand) -> Option<&'a NkCommand> {
        let r = unsafe {
            nk__next(&self.internal as *const _ as *mut nk_context,
                     &arg2.internal)
        };
        unsafe {
            if r.is_null() {
                None
            } else {
                Some(::std::mem::transmute(r))
            }
        }
    }

    pub fn begin_cmd<'a>(&'a self) -> Option<&'a NkCommand> {
        let r = unsafe { nk__begin(&self.internal as *const _ as *mut nk_context) };
        unsafe {
            if r.is_null() {
                None
            } else {
                Some(::std::mem::transmute(r))
            }
        }
    }

    pub fn draw_command_iterator<'a>(&'a mut self, buf: &'a NkBuffer) -> NkDrawCommandIterator<'a> {
        NkDrawCommandIterator {
            ctx: self,
            buf: buf,
        }
    }

    pub fn command_iterator<'a>(&'a mut self) -> NkCommandIterator<'a> {
        NkCommandIterator { ctx: self }
    }
}

// ============================================================================================

pub struct NkCommandIterator<'a> {
    ctx: &'a NkContext,
}

impl<'a> IntoIterator for NkCommandIterator<'a> {
    type Item = &'a NkCommand;
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
    ctx: &'a NkContext,
    cmd: Option<&'a NkCommand>,
}

impl<'a> Iterator for NkCommandIntoIter<'a> {
    type Item = &'a NkCommand;
    fn next(&mut self) -> Option<&'a NkCommand> {
        let r = self.cmd.clone();

        self.cmd = if let Some(p) = self.cmd {
            self.ctx.next_cmd(p)
        } else {
            None
        };

        r
    }
}

// ============================================================================================

pub struct NkDrawCommandIterator<'a> {
    ctx: &'a mut NkContext,
    buf: &'a NkBuffer,
}

impl<'a> IntoIterator for NkDrawCommandIterator<'a> {
    type Item = &'a NkDrawCommand;
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
    ctx: &'a NkContext,
    buf: &'a NkBuffer,
    cmd: Option<&'a NkDrawCommand>,
}

impl<'a> Iterator for NkDrawCommandIntoIter<'a> {
    type Item = &'a NkDrawCommand;
    fn next(&mut self) -> Option<&'a NkDrawCommand> {
        let r = self.cmd.clone();

        self.cmd = if let Some(ref p) = self.cmd {
            self.ctx.draw_next(p, self.buf)
        } else {
            None
        };

        r
    }
}

// =============================================================================================

wrapper_type_no_clone!(NkWindow, nk_window);

impl NkWindow {
    pub fn seq(&self) -> u32 {
        self.internal.seq
    }
    pub fn name<'a>(&'a self) -> &'a str {
        unsafe {
            let name = ::std::mem::transmute::<&[i8], &[u8]>(&self.internal.name_string);
            let mut len = name.len();
            let mut ch = 0;
            while ch == 0 && len > 0 {
                len -= 1;
                ch = name[len];
            }
            if len < name.len() {
                len += 1;
            }
            ::std::str::from_utf8_unchecked(&name[0..len])
        }
    }
    pub fn flags(&self) -> &NkFlags {
        &self.internal.flags
    }
    pub fn bounds(&self) -> &NkRect {
        &self.internal.bounds
    }
    pub fn scrollbar(&self) -> &NkScroll {
        &self.internal.scrollbar
    }
    pub fn scrollbar_hiding_timer(&self) -> f32 {
        self.internal.scrollbar_hiding_timer
    }
    pub fn buffer(&self) -> &NkCommandBuffer {
    	unsafe {
    		::std::mem::transmute(&self.internal.buffer)
    	}
    }
    pub fn layout(&self) -> &NkPanel {
    	unsafe {
    		::std::mem::transmute(self.internal.layout)
    	}
    }
    pub fn layout_mut(&mut self) -> &mut NkPanel {
    	unsafe {
    		::std::mem::transmute(self.internal.layout)
    	}
    }
    pub fn property(&self) -> &NkPropertyState {
    	unsafe {
    		::std::mem::transmute(&self.internal.property)
    	}
    }
    pub fn popup(&self) -> &NkPopupState {
    	unsafe {
    		::std::mem::transmute(&self.internal.popup)
    	}
    }
    pub fn edit(&self) -> &NkEditState {
    	unsafe {
    		::std::mem::transmute(&self.internal.edit)
    	}
    }
    pub fn scrolled(&self) -> u32 {
    	self.internal.scrolled
    }
    pub fn tables(&self) -> &[NkTable] {
    	unsafe {
    		::std::slice::from_raw_parts(self.internal.tables as *mut _ as *const NkTable, self.internal.table_count as usize)
    	}
    }
    pub fn table_size(&self) -> u16 {
    	self.internal.table_size
    }
    pub fn prev(&self) -> &NkWindow {
    	unsafe {
    		::std::mem::transmute(self.internal.prev)
    	}
    }
    pub fn next(&self) -> &NkWindow {
    	unsafe {
    		::std::mem::transmute(self.internal.next)
    	}
    }
    pub fn parent(&self) -> &NkWindow {
    	unsafe {
    		::std::mem::transmute(self.internal.parent)
    	}
    }

    pub fn set_flags(&mut self, flags: NkFlags) {
        self.internal.flags = flags;
    }
    pub fn set_bounds(&mut self, rect: NkRect) {
        self.internal.bounds = rect;
    }
    pub fn set_scrollbar(&mut self, scroll: NkScroll) {
        self.internal.scrollbar = scroll;
    }
    pub fn set_scrollbar_hiding_timer(&mut self, value: f32) {
        self.internal.scrollbar_hiding_timer = value;
    }
}

wrapper_type_no_clone!(NkPropertyState, nk_property_state);
wrapper_type!(NkPopupState, nk_popup_state);
wrapper_type!(NkEditState, nk_edit_state);
wrapper_type_no_clone!(NkTable, nk_table);

// =============================================================================================

wrapper_type!(NkRowLayout, nk_row_layout);

impl NkRowLayout {
    pub fn layout_type(&self) -> &NkPanelRowLayoutType {
        &self.internal.type_
    }
    pub fn layout_type_mut(&mut self) -> &mut NkPanelRowLayoutType {
        &mut self.internal.type_
    }

    pub fn index(&self) -> i32 {
        self.internal.index
    }
    pub fn set_index(&mut self, i: i32) {
        self.internal.index = i
    }

    pub fn height(&self) -> f32 {
        self.internal.height
    }
    pub fn set_height(&mut self, i: f32) {
        self.internal.height = i
    }

    pub fn columns(&self) -> i32 {
        self.internal.columns
    }
    pub fn set_columns(&mut self, i: i32) {
        self.internal.columns = i
    }

    pub fn ratio(&self) -> &f32 {
        unsafe { &*self.internal.ratio }
    }

    pub fn item_width(&self) -> f32 {
        self.internal.item_width
    }
    pub fn set_item_width(&mut self, i: f32) {
        self.internal.item_width = i
    }

    pub fn item_height(&self) -> f32 {
        self.internal.item_height
    }
    pub fn set_item_height(&mut self, i: f32) {
        self.internal.item_height = i
    }

    pub fn item_offset(&self) -> f32 {
        self.internal.item_offset
    }
    pub fn set_item_offset(&mut self, i: f32) {
        self.internal.item_offset = i
    }

    pub fn filled(&self) -> f32 {
        self.internal.filled
    }
    pub fn set_filled(&mut self, i: f32) {
        self.internal.filled = i
    }

    pub fn item(&self) -> &NkRect {
        &self.internal.item
    }
    pub fn item_mut(&mut self) -> &mut NkRect {
        &mut self.internal.item
    }

    pub fn tree_depth(&self) -> i32 {
        self.internal.tree_depth
    }
    pub fn set_tree_depth(&mut self, i: i32) {
        self.internal.tree_depth = i
    }

    pub fn templates(&self) -> &[f32] {
        &self.internal.templates
    }
    pub fn templates_mut(&mut self) -> &mut [f32] {
        &mut self.internal.templates
    }
}

// =============================================================================================

wrapper_type!(NkPanel, nk_panel);

impl NkPanel {
    pub fn bounds(&self) -> &NkRect {
        &self.internal.bounds
    }
    pub fn bounds_mut(&mut self) -> &mut NkRect {
        &mut self.internal.bounds
    }
    pub fn set_bounds(&mut self, b: NkRect) {
        self.internal.bounds = b
    }

    pub fn panel_type(&self) -> &NkPanelType {
        &self.internal.type_
    }
    pub fn panel_type_mut(&mut self) -> &mut NkPanelType {
        &mut self.internal.type_
    }
    pub fn set_panel_type(&mut self, t: NkPanelType) {
        self.internal.type_ = t
    }

    pub fn flags(&self) -> &NkFlags {
        &self.internal.flags
    }
    pub fn flags_mut(&mut self) -> &mut NkFlags {
        &mut self.internal.flags
    }
    pub fn set_flags(&mut self, f: NkFlags) {
        self.internal.flags = f
    }

    pub fn offset_x(&self) -> u32 {
        unsafe { *self.internal.offset_x }
    }
    pub fn set_offset_x(&mut self, o: u32) {
        unsafe { *self.internal.offset_x = o }
    }

    pub fn offset_y(&self) -> u32 {
        unsafe { *self.internal.offset_y }
    }
    pub fn set_offset_y(&mut self, o: u32) {
        unsafe { *self.internal.offset_y = o }
    }

    pub fn at_x(&self) -> f32 {
        self.internal.at_x
    }
    pub fn set_at_x(&mut self, f: f32) {
        self.internal.at_x = f
    }

    pub fn at_y(&self) -> f32 {
        self.internal.at_y
    }
    pub fn set_at_y(&mut self, f: f32) {
        self.internal.at_y = f
    }

    pub fn max_x(&self) -> f32 {
        self.internal.max_x
    }
    pub fn set_max_x(&mut self, f: f32) {
        self.internal.max_x = f
    }

    pub fn footer_height(&self) -> f32 {
        self.internal.footer_height
    }
    pub fn set_footer_height(&mut self, f: f32) {
        self.internal.footer_height = f
    }

    pub fn header_height(&self) -> f32 {
        self.internal.header_height
    }
    pub fn set_header_height(&mut self, f: f32) {
        self.internal.header_height = f
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }
    pub fn set_border(&mut self, f: f32) {
        self.internal.border = f
    }

    pub fn has_scrolling(&self) -> bool {
        self.internal.has_scrolling == nk_true as u32
    }
    pub fn set_has_scrolling(&mut self, f: bool) {
        self.internal.has_scrolling = if f { nk_true as u32 } else { nk_false as u32 }
    }

    pub fn clip(&self) -> &NkRect {
        &self.internal.clip
    }
    pub fn clip_mut(&mut self) -> &mut NkRect {
        &mut self.internal.clip
    }
    pub fn set_clip(&mut self, f: NkRect) {
        self.internal.clip = f
    }

    pub fn menu(&self) -> &NkMenuState {
        &self.internal.menu
    }
    pub fn menu_mut(&mut self) -> &mut NkMenuState {
        &mut self.internal.menu
    }
    pub fn set_menu(&mut self, f: NkMenuState) {
        self.internal.menu = f
    }

    pub fn row(&self) -> &NkRowLayout {
        unsafe { ::std::mem::transmute(&self.internal.row) }
    }

    pub fn chart(&self) -> &NkChart {
        unsafe { ::std::mem::transmute(&self.internal.chart) }
    }

    pub fn popup_buffer(&self) -> &NkPopupBuffer {
        &self.internal.popup_buffer
    }

    pub fn popup_buffer_mut(&mut self) -> &mut NkPopupBuffer {
        &mut self.internal.popup_buffer
    }

    pub fn buffer(&self) -> Option<&NkCommandBuffer> {
        unsafe {
            let ptr = self.internal.buffer;
            if !ptr.is_null() {
                Some(::std::mem::transmute(ptr))
            } else {
                None
            }
        }
    }

    pub fn parent(&self) -> Option<&NkPanel> {
        unsafe {
            let ptr = self.internal.parent;
            if !ptr.is_null() {
                Some(::std::mem::transmute(ptr))
            } else {
                None
            }
        }
    }
}

// =============================================================================================

wrapper_type!(NkChart, nk_chart);

impl NkChart {
    pub fn x(&self) -> f32 {
        self.internal.x
    }
    pub fn set_x(&mut self, f: f32) {
        self.internal.x = f
    }

    pub fn y(&self) -> f32 {
        self.internal.y
    }
    pub fn set_y(&mut self, f: f32) {
        self.internal.y = f
    }

    pub fn w(&self) -> f32 {
        self.internal.w
    }
    pub fn set_w(&mut self, f: f32) {
        self.internal.w = f
    }

    pub fn h(&self) -> f32 {
        self.internal.h
    }
    pub fn set_h(&mut self, f: f32) {
        self.internal.h = f
    }

    pub fn slot(&self) -> u32 {
        self.internal.slot as u32
    }

    pub fn slots(&self) -> &[NkChartSlot] {
        &self.internal.slots
    }
}

// =============================================================================================

macro_rules! emit_nk_command {
	($rs_ty: ident, $nat_ty: ty) => {
		wrapper_type!($rs_ty, $nat_ty);
		
		impl AsRef<$rs_ty> for NkCommand {
			fn as_ref(&self) -> &$rs_ty {
				unsafe { ::std::mem::transmute(&self.internal) }
			}
		}
		
		impl $rs_ty {
		    pub fn header(&self) -> &NkCommand {
			    unsafe { ::std::mem::transmute(&self.internal.header) }
		    }
		}		
	}
}

wrapper_type!(NkCommand, nk_command);

impl NkCommand {
    pub fn get_type(&self) -> NkCommandType {
        self.internal.type_
    }
}

emit_nk_command!(NkCommandScissor, nk_command_scissor);
impl NkCommandScissor {
    pub fn x(&self) -> i16 {
        self.internal.x
    }
    pub fn y(&self) -> i16 {
        self.internal.y
    }
    pub fn w(&self) -> u16 {
        self.internal.w
    }
    pub fn h(&self) -> u16 {
        self.internal.h
    }
}

emit_nk_command!(NkCommandLine, nk_command_line);
impl NkCommandLine {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn begin(&self) -> NkVec2i {
        self.internal.begin
    }
    pub fn end(&self) -> NkVec2i {
        self.internal.end
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandCurve, nk_command_curve);
impl NkCommandCurve {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn begin(&self) -> NkVec2i {
        self.internal.begin
    }
    pub fn end(&self) -> NkVec2i {
        self.internal.end
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
    pub fn ctrl(&self) -> &[NkVec2i] {
        &self.internal.ctrl
    }
}

emit_nk_command!(NkCommandRect, nk_command_rect);
impl NkCommandRect {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn rounding(&self) -> u16 {
        self.internal.rounding
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
    pub fn x(&self) -> i16 {
        self.internal.x
    }
    pub fn y(&self) -> i16 {
        self.internal.y
    }
    pub fn w(&self) -> u16 {
        self.internal.w
    }
    pub fn h(&self) -> u16 {
        self.internal.h
    }
}

emit_nk_command!(NkCommandRectFilled, nk_command_rect_filled);
impl NkCommandRectFilled {
    pub fn rounding(&self) -> u16 {
        self.internal.rounding
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
    pub fn x(&self) -> i16 {
        self.internal.x
    }
    pub fn y(&self) -> i16 {
        self.internal.y
    }
    pub fn w(&self) -> u16 {
        self.internal.w
    }
    pub fn h(&self) -> u16 {
        self.internal.h
    }
}

emit_nk_command!(NkCommandRectMultiColor, nk_command_rect_multi_color);
impl NkCommandRectMultiColor {
    pub fn left(&self) -> NkColor {
        self.internal.left
    }
    pub fn top(&self) -> NkColor {
        self.internal.top
    }
    pub fn right(&self) -> NkColor {
        self.internal.right
    }
    pub fn bottom(&self) -> NkColor {
        self.internal.bottom
    }
    pub fn x(&self) -> i16 {
        self.internal.x
    }
    pub fn y(&self) -> i16 {
        self.internal.y
    }
    pub fn w(&self) -> u16 {
        self.internal.w
    }
    pub fn h(&self) -> u16 {
        self.internal.h
    }
}

emit_nk_command!(NkCommandTriangle, nk_command_triangle);
impl NkCommandTriangle {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn a(&self) -> NkVec2i {
        self.internal.a
    }
    pub fn b(&self) -> NkVec2i {
        self.internal.b
    }
    pub fn c(&self) -> NkVec2i {
        self.internal.c
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandTriangleFilled, nk_command_triangle_filled);
impl NkCommandTriangleFilled {
    pub fn a(&self) -> NkVec2i {
        self.internal.a
    }
    pub fn b(&self) -> NkVec2i {
        self.internal.b
    }
    pub fn c(&self) -> NkVec2i {
        self.internal.c
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandCircle, nk_command_circle);
impl NkCommandCircle {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn x(&self) -> i16 {
        self.internal.x
    }
    pub fn y(&self) -> i16 {
        self.internal.y
    }
    pub fn w(&self) -> u16 {
        self.internal.w
    }
    pub fn h(&self) -> u16 {
        self.internal.h
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandCircleFilled, nk_command_circle_filled);
impl NkCommandCircleFilled {
    pub fn x(&self) -> i16 {
        self.internal.x
    }
    pub fn y(&self) -> i16 {
        self.internal.y
    }
    pub fn w(&self) -> u16 {
        self.internal.w
    }
    pub fn h(&self) -> u16 {
        self.internal.h
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandArc, nk_command_arc);
impl NkCommandArc {
    pub fn cx(&self) -> i16 {
        self.internal.cx
    }
    pub fn cy(&self) -> i16 {
        self.internal.cy
    }
    pub fn r(&self) -> u16 {
        self.internal.r
    }
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn a(&self) -> &[f32] {
        &self.internal.a
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandArcFilled, nk_command_arc_filled);
impl NkCommandArcFilled {
    pub fn cx(&self) -> i16 {
        self.internal.cx
    }
    pub fn cy(&self) -> i16 {
        self.internal.cy
    }
    pub fn r(&self) -> u16 {
        self.internal.r
    }
    pub fn a(&self) -> &[f32] {
        &self.internal.a
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandPolygon, nk_command_polygon);
impl NkCommandPolygon {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn points(&self) -> &[NkVec2i] {
        unsafe {
            ::std::slice::from_raw_parts(self.internal.points.as_ptr(),
                                         self.internal.point_count as usize)
        }
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandPolygonFilled, nk_command_polygon_filled);
impl NkCommandPolygonFilled {
    pub fn points(&self) -> &[NkVec2i] {
        unsafe {
            ::std::slice::from_raw_parts(self.internal.points.as_ptr(),
                                         self.internal.point_count as usize)
        }
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandPolyline, nk_command_polyline);
impl NkCommandPolyline {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn points(&self) -> &[NkVec2i] {
        unsafe {
            ::std::slice::from_raw_parts(self.internal.points.as_ptr(),
                                         self.internal.point_count as usize)
        }
    }
    pub fn color(&self) -> NkColor {
        self.internal.color
    }
}

emit_nk_command!(NkCommandImage, nk_command_image);
impl NkCommandImage {
    pub fn x(&self) -> i16 {
        self.internal.x
    }
    pub fn y(&self) -> i16 {
        self.internal.y
    }
    pub fn w(&self) -> u16 {
        self.internal.w
    }
    pub fn h(&self) -> u16 {
        self.internal.h
    }
    pub fn col(&self) -> NkColor {
        self.internal.col
    }
    pub fn img(&self) -> NkImage {
        NkImage { internal: self.internal.img }
    }
}

emit_nk_command!(NkCommandText, nk_command_text);
impl NkCommandText {
    pub fn x(&self) -> i16 {
        self.internal.x
    }
    pub fn y(&self) -> i16 {
        self.internal.y
    }
    pub fn w(&self) -> u16 {
        self.internal.w
    }
    pub fn h(&self) -> u16 {
        self.internal.h
    }
    pub fn height(&self) -> f32 {
        self.internal.height
    }
    pub fn chars(&self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts(self.internal.string.as_ptr() as *const u8,
                                         self.internal.length as usize)
        }
    }
    pub fn background(&self) -> NkColor {
        self.internal.background
    }
    pub fn foreground(&self) -> NkColor {
        self.internal.foreground
    }
    pub fn font(&self) -> &NkUserFont {
        unsafe { ::std::mem::transmute(self.internal.font) }
    }
}

// =============================================================================================

wrapper_type!(NkCommandBuffer, nk_command_buffer);

impl NkCommandBuffer {
    pub fn stroke_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, line_thickness: f32, color: NkColor) {
        unsafe {
            nk_stroke_line(&mut self.internal, x0, y0, x1, y1, line_thickness, color);
        }
    }

    pub fn stroke_curve(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, line_thickness: f32, color: NkColor) {
        unsafe {
            nk_stroke_curve(&mut self.internal,
                            x0,
                            y0,
                            x1,
                            y1,
                            x2,
                            y2,
                            x3,
                            y3,
                            line_thickness,
                            color);
        }
    }

    pub fn stroke_rect(&mut self, bounds: NkRect, rounding: f32, line_thickness: f32, color: NkColor) {
        unsafe {
            nk_stroke_rect(&mut self.internal, bounds, rounding, line_thickness, color);
        }
    }

    pub fn stroke_circle(&mut self, arg2: NkRect, line_thickness: f32, color: NkColor) {
        unsafe {
            nk_stroke_circle(&mut self.internal, arg2, line_thickness, color);
        }
    }

    pub fn stroke_arc(&mut self, cx: f32, cy: f32, radius: f32, a_min: f32, a_max: f32, line_thickness: f32, color: NkColor) {
        unsafe {
            nk_stroke_arc(&mut self.internal,
                          cx,
                          cy,
                          radius,
                          a_min,
                          a_max,
                          line_thickness,
                          color);
        }
    }

    pub fn stroke_triangle(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, line_thichness: f32, color: NkColor) {
        unsafe {
            nk_stroke_triangle(&mut self.internal,
                               x0,
                               y0,
                               x1,
                               y1,
                               x2,
                               y2,
                               line_thichness,
                               color);
        }
    }

    pub fn stroke_polyline(&mut self, points: &mut [f32], line_thickness: f32, color: NkColor) {
        unsafe {
            nk_stroke_polyline(&mut self.internal,
                               &mut points[0] as *mut f32,
                               points.len() as ::std::os::raw::c_int,
                               line_thickness,
                               color);
        }
    }

    pub fn stroke_polygon(&mut self, points: &mut [f32], line_thickness: f32, color: NkColor) {
        unsafe {
            nk_stroke_polygon(&mut self.internal,
                              &mut points[0] as *mut f32,
                              points.len() as ::std::os::raw::c_int,
                              line_thickness,
                              color);
        }
    }

    pub fn fill_rect(&mut self, arg2: NkRect, rounding: f32, color: NkColor) {
        unsafe {
            nk_fill_rect(&mut self.internal, arg2, rounding, color);
        }
    }

    pub fn fill_rect_multi_color(&mut self, arg2: NkRect, left: NkColor, top: NkColor, right: NkColor, bottom: NkColor) {
        unsafe {
            nk_fill_rect_multi_color(&mut self.internal, arg2, left, top, right, bottom);
        }
    }

    pub fn fill_circle(&mut self, arg2: NkRect, color: NkColor) {
        unsafe {
            nk_fill_circle(&mut self.internal, arg2, color);
        }
    }

    pub fn fill_arc(&mut self, cx: f32, cy: f32, radius: f32, a_min: f32, a_max: f32, color: NkColor) {
        unsafe {
            nk_fill_arc(&mut self.internal, cx, cy, radius, a_min, a_max, color);
        }
    }

    pub fn fill_triangle(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, color: NkColor) {
        unsafe {
            nk_fill_triangle(&mut self.internal, x0, y0, x1, y1, x2, y2, color);
        }
    }

    pub fn fill_polygon(&mut self, points: &mut [f32], color: NkColor) {
        unsafe {
            nk_fill_polygon(&mut self.internal,
                            &mut points[0] as *mut f32,
                            points.len() as ::std::os::raw::c_int,
                            color);
        }
    }

    pub fn push_scissor(&mut self, arg2: NkRect) {
        unsafe {
            nk_push_scissor(&mut self.internal, arg2);
        }
    }

    pub fn draw_image(&mut self, arg2: NkRect, arg3: &NkImage, arg4: NkColor) {
        unsafe {
            nk_draw_image(&mut self.internal,
                          arg2,
                          &arg3.internal as *const nk_image,
                          arg4);
        }
    }

    pub fn draw_text(&mut self, arg2: NkRect, text: &str, arg3: &NkUserFont, arg4: NkColor, arg5: NkColor) {
        unsafe {
            nk_draw_text(&mut self.internal,
                         arg2,
                         text.as_ptr() as *const i8,
                         text.as_bytes().len() as ::std::os::raw::c_int,
                         &arg3.internal,
                         arg4,
                         arg5);
        }
    }
}

// =============================================================================================

pub fn color_rgb(r: i32, g: i32, b: i32) -> NkColor {
    unsafe { nk_rgb(r, g, b) }
}

pub fn color_rgb_iv(rgb: &i32) -> NkColor {
    unsafe { nk_rgb_iv(rgb as *const i32) }
}

pub fn color_rgb_bv(rgb: &u8) -> NkColor {
    unsafe { nk_rgb_bv(rgb as *const u8) }
}

pub fn color_rgb_fv(rgb: &f32) -> NkColor {
    unsafe { nk_rgb_fv(rgb as *const f32) }
}

pub fn color_rgb_f(r: f32, g: f32, b: f32) -> NkColor {
    unsafe { nk_rgb_f(r, g, b) }
}

pub fn color_rgb_hex(rgb: NkString) -> NkColor {
    unsafe { nk_rgb_hex(rgb.as_ptr()) }
}

pub fn color_rgba(r: i32, g: i32, b: i32, a: i32) -> NkColor {
    unsafe { nk_rgba(r, g, b, a) }
}

pub fn color_rgba_u32(rgba: u32) -> NkColor {
    unsafe { nk_rgba_u32(rgba) }
}


pub fn color_rgba_iv(rgba: &i32) -> NkColor {
    unsafe { nk_rgba_iv(rgba as *const i32) }
}

pub fn color_rgba_bv(rgb: &u8) -> NkColor {
    unsafe { nk_rgba_bv(rgb as *const u8) }
}

pub fn color_rgba_fv(rgb: &f32) -> NkColor {
    unsafe { nk_rgba_fv(rgb as *const f32) }
}

pub fn color_rgba_f(r: f32, g: f32, b: f32, a: f32) -> NkColor {
    unsafe { nk_rgba_f(r, g, b, a) }
}

pub fn color_rgba_hex(rgba: NkString) -> NkColor {
    unsafe { nk_rgba_hex(rgba.as_ptr()) }
}

pub fn color_hsv(h: i32, s: i32, v: i32) -> NkColor {
    unsafe { nk_hsv(h, s, v) }
}

pub fn color_hsv_iv(hsv: &i32) -> NkColor {
    unsafe { nk_hsv_iv(hsv as *const i32) }
}

pub fn color_hsv_bv(hsv: &u8) -> NkColor {
    unsafe { nk_hsv_bv(hsv as *const u8) }
}

pub fn color_hsv_fv(hsv: &f32) -> NkColor {
    unsafe { nk_hsv_fv(hsv as *const f32) }
}

pub fn color_hsv_f(h: f32, s: f32, v: f32) -> NkColor {
    unsafe { nk_hsv_f(h, s, v) }
}

pub fn color_hsva(h: i32, s: i32, v: i32, a: i32) -> NkColor {
    unsafe { nk_hsva(h, s, v, a) }
}

pub fn color_hsva_iv(hsva: &i32) -> NkColor {
    unsafe { nk_hsva_iv(hsva as *const i32) }
}

pub fn color_hsva_bv(hsv: &u8) -> NkColor {
    unsafe { nk_hsva_bv(hsv as *const u8) }
}

pub fn color_hsva_fv(hsv: &f32) -> NkColor {
    unsafe { nk_hsva_fv(hsv as *const f32) }
}

pub fn color_hsva_f(h: f32, s: f32, v: f32, a: f32) -> NkColor {
    unsafe { nk_hsva_f(h, s, v, a) }
}

pub fn style_get_color_by_name(c: NkStyleColor) -> Cow<'static, str> {
    unsafe {
        // NkString::from_bytes_unchecked()
        // CString::from_raw(nk_style_get_color_by_name(c))
        ::std::ffi::CStr::from_ptr(nk_style_get_color_by_name(c)).to_string_lossy()
    }
}

// =============================================================================================

wrapper_type!(NkImage, nk_image);

impl NkImage {
    pub fn with_id(id: i32) -> NkImage {
        NkImage { internal: unsafe { nk_image_id(id) } }
    }

    pub fn with_ptr(ptr: *mut c_void) -> NkImage {
        NkImage { internal: unsafe { nk_image_ptr(ptr) } }
    }

    pub fn id(&mut self) -> i32 {
        unsafe { *(self.internal.handle.id.as_ref()) }
    }

    pub fn ptr(&mut self) -> *mut c_void {
        unsafe { *(self.internal.handle.ptr.as_mut()) }
    }
}

// =============================================================================================

wrapper_type!(NkFontGlyph, nk_font_glyph);

// impl Default for NkFontGlyph {
// fn default() -> Self {
// NkFontGlyph {
// internal: nk_font_glyph::default()
// }
// }
// }

impl NkFontGlyph {
    pub fn get_codepoint(&self) -> char {
        ::std::char::from_u32(self.internal.codepoint).unwrap()
    }
    pub fn get_xadvance(&self) -> f32 {
        self.internal.xadvance
    }
    pub fn x0(&self) -> f32 {
        self.internal.x0
    }
    pub fn y0(&self) -> f32 {
        self.internal.y0
    }
    pub fn x1(&self) -> f32 {
        self.internal.x1
    }
    pub fn y1(&self) -> f32 {
        self.internal.y1
    }
    pub fn w(&self) -> f32 {
        self.internal.w
    }
    pub fn h(&self) -> f32 {
        self.internal.h
    }
    pub fn u0(&self) -> f32 {
        self.internal.u0
    }
    pub fn v0(&self) -> f32 {
        self.internal.v0
    }
    pub fn u1(&self) -> f32 {
        self.internal.u1
    }
    pub fn v1(&self) -> f32 {
        self.internal.v1
    }
}

// =============================================================================================

wrapper_type!(NkFont, nk_font);

impl NkFont {
    pub fn find_glyph(&self, unicode: char) -> &NkFontGlyph {
        unsafe { ::std::mem::transmute(nk_font_find_glyph(&self.internal as *const _ as *mut nk_font, unicode as u32)) }
    }

    pub fn handle(&self) -> &NkUserFont {
        unsafe { ::std::mem::transmute(&self.internal.handle as *const _ as *mut nk_user_font) }
    }
}

// =============================================================================================

wrapper_type!(NkUserFont, nk_user_font);

impl NkUserFont {
    pub unsafe fn userdata_ptr(&self) -> NkHandle {
        NkHandle::from_ptr(*self.internal.userdata.ptr.as_ref())
    }

    pub unsafe fn userdata_id(&self) -> NkHandle {
        NkHandle::from_id(*self.internal.userdata.id.as_ref())
    }
}

// =============================================================================================

fn raw_glyph_ranges_to_safe<'a>(arg: *const nk_rune) -> &'a [(u32, u32)] {
    unsafe {
        let len32 = (::std::mem::size_of::<(u32, u32)>() / ::std::mem::size_of::<u32>()) as isize;

        let mut raw2 = arg.clone();

        let mut i = 0xffff;
        let mut len = 0;
        while i > 0 {
            i = *raw2;
            raw2 = raw2.offset(len32);
            if i > 0 {
                len += 1;
            }
        }

        ::std::slice::from_raw_parts(arg as *const (u32, u32), len)
    }
}

pub fn font_default_glyph_ranges<'a>() -> &'a [(u32, u32)] {
    unsafe { raw_glyph_ranges_to_safe(nk_font_default_glyph_ranges()) }
}

pub fn font_chinese_glyph_ranges<'a>() -> &'a [(u32, u32)] {
    unsafe { raw_glyph_ranges_to_safe(nk_font_chinese_glyph_ranges()) }
}

pub fn font_cyrillic_glyph_ranges<'a>() -> &'a [(u32, u32)] {
    unsafe { raw_glyph_ranges_to_safe(nk_font_cyrillic_glyph_ranges()) }
}

pub fn font_korean_glyph_ranges<'a>() -> &'a [(u32, u32)] {
    unsafe { raw_glyph_ranges_to_safe(nk_font_korean_glyph_ranges()) }
}
