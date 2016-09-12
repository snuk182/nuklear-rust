#![feature(alloc, heap_api)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate alloc;

use std::default::Default;
use libc::*;
/*
 * ==============================================================
 *
 *                          CONSTANTS
 *
 * ===============================================================
 */
pub const NK_UTF_INVALID: uint32_t = 0xFFFD; /* internal invalid utf8 rune */
pub const NK_UTF_SIZE: size_t = 4; /* describes the number of bytes a glyph consists of*/

pub const NK_INPUT_MAX: size_t = 16;
pub const NK_MAX_NUMBER_BUFFER: size_t = 64;
//const NK_SCROLLBAR_HIDING_TIMEOUT: c_float = 4.0;
/*
 * ===============================================================
 *
 *                          BASIC
 *
 * ===============================================================
 */
pub type NkShort = int16_t;
pub type NkUshort = uint16_t;
pub type NkInt = int32_t;
pub type NkUint = uint32_t;
pub type NkHash = uint32_t;
pub type NkSize = uintptr_t;
pub type NkPtr = uintptr_t;
pub type NkFlags = uint32_t;
pub type NkRune = uint32_t;
pub type NkByte = uint8_t;

pub type NkChar = c_char;

/*#ifdef NK_PRIVATE
#define 	pub fn static
#else
#define 	pub fn extern
#endif

#define NK_INTERN static
#define NK_STORAGE static
#define NK_GLOBAL static */

/* ============================================================================
 *
 *                                  API
 *
 * =========================================================================== */
pub const NK_UNDEFINED: c_float = -1.0;
//pub fn NK_FLAG(x: size_t) {1 << (x)}
//pub fn NK_STRINGIFY(x: _) {stringify!(x)}
//pub fn NK_LINE_STR(x: _) {NK_STRINGIFY(x)}
//pub fn NK_FILE_LINE __FILE__ ":" NK_LINE_STR(__LINE__) 

pub const NkFalse: ssize_t = 0;
pub const NkTrue: ssize_t = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkColor {
	r: NkByte,
	g: NkByte,
	b: NkByte,
	a: NkByte,
}

impl Default for NkColor {
	fn default() -> NkColor { NkColor {r:0, g:0, b:0, a: 0} }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkVec2 {
	x: c_float,
	y: c_float,
}

impl Default for NkVec2 {
	fn default() -> NkVec2 { NkVec2 {x:0.0, y:0.0} }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkVec2i {
	x: c_short,
	y: c_short
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct NkRect {
	x: c_float,
	y: c_float,
	w: c_float,
	h: c_float,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct NkRecti {
	x: c_short,
	y: c_short,
	w: c_short,
	h: c_short,
}

pub type NkGlyph = [c_char; NK_UTF_SIZE];

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkHandle {ptr: *mut c_void, id: c_int}

impl Default for NkHandle {
	fn default() -> NkHandle {
		NkHandle {ptr: ::std::ptr::null_mut(), id: 0}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkImage {
	handle: NkHandle,
	w: c_ushort,h: c_ushort,
	region: [c_ushort; 4]
}

impl Default for NkImage {
	fn default() -> NkImage {
		NkImage {
			handle: NkHandle::default(),
			w: 0,
			h: 0,
			region: [0; 4]
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct NkCursor {
	img: NkImage, 
	size: NkVec2,
	offset: NkVec2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct NkScroll {
	x: c_ushort, 
	y: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkHeading {
	NK_UP, 
	NK_RIGHT, 
	NK_DOWN, 
	NK_LEFT
}

//typedef int (*func)(int a, int b);
//func(a, b) -> int

pub type NkFilter =  Option<extern "C" fn(*const NkTextEdit, NkRune) -> c_int>; //typedef int(*NkFilter)(editor: *const NkTextEdit, unicode: NkRune ); 
pub type NkFilterF =  Option<extern "C" fn(NkHandle, *mut NkTextEdit)>;
pub type NkCopyF =  Option<extern "C" fn(NkHandle, *const c_char, c_int)>;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkButtonBehavior {NK_BUTTON_DEFAULT, NK_BUTTON_REPEATER}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkModify          {NK_FIXED = NkFalse, NK_MODIFIABLE = NkTrue}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkOrientation     {NK_VERTICAL, NK_HORIZONTAL}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkCollapseStates {NK_MINIMIZED = NkFalse, NK_MAXIMIZED = NkTrue}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkShowStates     {NK_HIDDEN = NkFalse, NK_SHOWN = NkTrue}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkChartType      {NK_CHART_LINES, NK_CHART_COLUMN, NK_CHART_MAX}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkChartEvent     {NK_CHART_HOVERING = 0x01, NK_CHART_CLICKED = 0x02}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkColorFormat    {NK_RGB, NK_RGBA}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkPopupType      {NK_POPUP_STATIC, NK_POPUP_DYNAMIC}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkLayoutFormat   {NK_DYNAMIC, NK_STATIC}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkTreeType       {NK_TREE_NODE, NK_TREE_TAB}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkAntiAliasing   {NK_ANTI_ALIASING_OFF, NK_ANTI_ALIASING_ON}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkAllocator {
    userdata: NkHandle,
    alloc:  Option<extern "C" fn(NkHandle, *mut c_void, NkSize) -> *mut c_void>,
    free:  Option<extern "C" fn(NkHandle, *mut c_void)>,
}

impl NkAllocator {
	pub fn new_heap() -> NkAllocator {
		NkAllocator {
			alloc:  Some(alloc_rust),
		    free:  Some(free_rust),
		    ..Default::default()
		}
	}
	
	pub fn new_vec() -> NkAllocator {
		NkAllocator {
			alloc:  Some(alloc_rust_hacky),
		    free:  Some(free_rust_hacky),
		    ..Default::default()
		}
	}
}

impl Default for NkAllocator {
	fn default() -> NkAllocator {
		NkAllocator {
		    userdata: NkHandle::default(),
		    alloc:  None,
		    free:  None,
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkDrawNullTexture {
    texture: NkHandle,/* texture handle to a texture with a white pixel */
    uv: NkVec2, /* coordinates to a white pixel in the texture  */
}

impl Default for NkDrawNullTexture {
	fn default() -> NkDrawNullTexture {
		NkDrawNullTexture {
		    texture: NkHandle::default(),/* texture handle to a texture with a white pixel */
		    uv: NkVec2::default(), /* coordinates to a white pixel in the texture  */
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkConvertColor {
    global_alpha: c_float, /* global alpha value */
    line_AA: NkAntiAliasing, /* line anti-aliasing flag can be turned off if you are tight on memory */
    shape_AA: NkAntiAliasing, /* shape anti-aliasing flag can be turned off if you are tight on memory */
    circle_segment_count: c_uint, /* number of segments used for circles: default to 22 */
    arc_segment_count: c_uint, /* number of segments used for arcs: default to 22 */
    curve_segment_count: c_uint, /* number of segments used for curves: default to 22 */
    null: NkDrawNullTexture, /* handle to texture with a white pixel for shape drawing */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkSymbolType {
    NK_SYMBOL_NONE,
    NK_SYMBOL_X,
    NK_SYMBOL_UNDERSCORE,
    NK_SYMBOL_CIRCLE,
    NK_SYMBOL_CIRCLE_FILLED,
    NK_SYMBOL_RECT,
    NK_SYMBOL_RECT_FILLED,
    NK_SYMBOL_TRIANGLE_UP,
    NK_SYMBOL_TRIANGLE_DOWN,
    NK_SYMBOL_TRIANGLE_LEFT,
    NK_SYMBOL_TRIANGLE_RIGHT,
    NK_SYMBOL_PLUS,
    NK_SYMBOL_MINUS,
    NK_SYMBOL_MAX
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkKeys {
    NK_KEY_NONE,
    NK_KEY_SHIFT,
    NK_KEY_CTRL,
    NK_KEY_DEL,
    NK_KEY_ENTER,
    NK_KEY_TAB,
    NK_KEY_BACKSPACE,
    NK_KEY_COPY,
    NK_KEY_CUT,
    NK_KEY_PASTE,
    NK_KEY_UP,
    NK_KEY_DOWN,
    NK_KEY_LEFT,
    NK_KEY_RIGHT,//13

    /* Shortcuts: text field */
    NK_KEY_TEXT_INSERT_MODE,
    NK_KEY_TEXT_REPLACE_MODE,
    NK_KEY_TEXT_RESET_MODE,
    NK_KEY_TEXT_LINE_START,
    NK_KEY_TEXT_LINE_END,
    NK_KEY_TEXT_START,
    NK_KEY_TEXT_END,
    NK_KEY_TEXT_UNDO,
    NK_KEY_TEXT_REDO,
    NK_KEY_TEXT_WORD_LEFT,
    NK_KEY_TEXT_WORD_RIGHT,//24

    /* Shortcuts: scrollbar */
    NK_KEY_SCROLL_START,
    NK_KEY_SCROLL_END,
    NK_KEY_SCROLL_DOWN,
    NK_KEY_SCROLL_UP,

    //NK_KEY_MAX, //29
}
pub const NK_KEY_MAX: c_uint = 29;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkButtons {
    NK_BUTTON_LEFT,
    NK_BUTTON_MIDDLE,
    NK_BUTTON_RIGHT,
    //NK_BUTTON_MAX
}
pub const NK_BUTTON_MAX: c_uint = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkStyleColors {
    NK_COLOR_TEXT,
    NK_COLOR_WINDOW,
    NK_COLOR_HEADER,
    NK_COLOR_BORDER,
    NK_COLOR_BUTTON,
    NK_COLOR_BUTTON_HOVER,
    NK_COLOR_BUTTON_ACTIVE,
    NK_COLOR_TOGGLE,
    NK_COLOR_TOGGLE_HOVER,
    NK_COLOR_TOGGLE_CURSOR,
    NK_COLOR_SELECT,
    NK_COLOR_SELECT_ACTIVE,
    NK_COLOR_SLIDER,
    NK_COLOR_SLIDER_CURSOR,
    NK_COLOR_SLIDER_CURSOR_HOVER,
    NK_COLOR_SLIDER_CURSOR_ACTIVE,
    NK_COLOR_PROPERTY,
    NK_COLOR_EDIT,
    NK_COLOR_EDIT_CURSOR,
    NK_COLOR_COMBO,
    NK_COLOR_CHART,
    NK_COLOR_CHART_COLOR,
    NK_COLOR_CHART_COLOR_HIGHLIGHT,
    NK_COLOR_SCROLLBAR,
    NK_COLOR_SCROLLBAR_CURSOR,
    NK_COLOR_SCROLLBAR_CURSOR_HOVER,
    NK_COLOR_SCROLLBAR_CURSOR_ACTIVE,
    NK_COLOR_TAB_HEADER,
    //NK_COLOR_COUNT
}
pub const NK_COLOR_COUNT: c_uint = 28;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkStyleCursor {
    NK_CURSOR_ARROW,
    NK_CURSOR_TEXT,
    NK_CURSOR_MOVE,
    NK_CURSOR_RESIZE_VERTICAL,
    NK_CURSOR_RESIZE_HORIZONTAL,
    NK_CURSOR_RESIZE_TOP_LEFT_DOWN_RIGHT,
    NK_CURSOR_RESIZE_TOP_RIGHT_DOWN_LEFT,
    //NK_CURSOR_COUNT
}
pub const NK_CURSOR_COUNT: usize = 7;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkWidgetLayoutStates {
    NK_WIDGET_INVALID, /* The widget cannot be seen and is completely out of view */
    NK_WIDGET_VALID, /* The widget is completely inside the window and can be updated and drawn */
    NK_WIDGET_ROM /* The widget is partially visible and cannot be updated */
}

/* widget states */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkWidgetStates {
    NK_WIDGET_STATE_MODIFIED    = 1 << 1, //NK_FLAG(1),
    NK_WIDGET_STATE_INACTIVE    = 1 << 2, //NK_FLAG(2), /* widget is neither active nor hovered */
    NK_WIDGET_STATE_ENTERED     = 1 << 3, //NK_FLAG(3), /* widget has been hovered on the current frame */
    NK_WIDGET_STATE_HOVER       = 1 << 4, //NK_FLAG(4), /* widget is being hovered */
    NK_WIDGET_STATE_ACTIVED     = 1 << 5, //NK_FLAG(5), /* widget is currently activated */
    NK_WIDGET_STATE_LEFT        = 1 << 6, //NK_FLAG(6), /* widget is from this frame on not hovered anymore */
    NK_WIDGET_STATE_HOVERED     = (1 << 4) | (1 << 1), //NK_WIDGET_STATE_HOVER|NK_WIDGET_STATE_MODIFIED, /* widget is being hovered */
    NK_WIDGET_STATE_ACTIVE      = (1 << 5) | (1 << 1), //NK_WIDGET_STATE_ACTIVED|NK_WIDGET_STATE_MODIFIED /* widget is currently activated */
}

/* text alignment */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkTextAlign {
    NK_TEXT_ALIGN_LEFT        = 0x01,
    NK_TEXT_ALIGN_CENTERED    = 0x02,
    NK_TEXT_ALIGN_RIGHT       = 0x04,
    NK_TEXT_ALIGN_TOP         = 0x08,
    NK_TEXT_ALIGN_MIDDLE      = 0x10,
    NK_TEXT_ALIGN_BOTTOM      = 0x20
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkTextAlignment {
    NK_TEXT_LEFT        = 0x10 | 0x01, //NK_TEXT_ALIGN_MIDDLE|NK_TEXT_ALIGN_LEFT,
    NK_TEXT_CENTERED    = 0x10 | 0x02, //NK_TEXT_ALIGN_MIDDLE|NK_TEXT_ALIGN_CENTERED,
    NK_TEXT_RIGHT       = 0x10 | 0x04, //NK_TEXT_ALIGN_MIDDLE|NK_TEXT_ALIGN_RIGHT
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkEditFlags {
    NK_EDIT_DEFAULT                 = 0,
    NK_EDIT_READ_ONLY               = 1 << 0, //NK_FLAG(0),
    NK_EDIT_AUTO_SELECT             = 1 << 1, //NK_FLAG(1),
    NK_EDIT_SIG_ENTER               = 1 << 2, //NK_FLAG(2),
    NK_EDIT_ALLOW_TAB               = 1 << 3, //NK_FLAG(3),
    NK_EDIT_NO_CURSOR               = 1 << 4, //NK_FLAG(4),
    NK_EDIT_SELECTABLE              = 1 << 5, //NK_FLAG(5),
    NK_EDIT_CLIPBOARD               = 1 << 6, //NK_FLAG(6),
    NK_EDIT_CTRL_ENTER_NEWLINE      = 1 << 7, //NK_FLAG(7),
    NK_EDIT_NO_HORIZONTAL_SCROLL    = 1 << 8, //NK_FLAG(8),
    NK_EDIT_ALWAYS_INSERT_MODE      = 1 << 9, //NK_FLAG(9),
    NK_EDIT_MULTILINE               = 1 << 11, //NK_FLAG(11),
    NK_EDIT_GOTO_END_ON_ACTIVATE    = 1 << 12, //NK_FLAG(12)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkEditTypes {
    NK_EDIT_SIMPLE  = 1 << 9, //NK_EDIT_ALWAYS_INSERT_MODE,
    NK_EDIT_FIELD   = (1 << 9) | (1 << 5) | (1 << 6), //NK_EDIT_SIMPLE|NK_EDIT_SELECTABLE|NK_EDIT_CLIPBOARD,
    NK_EDIT_BOX     = (1 << 9) | (1 << 5) | (1 << 11) | (1 << 3) | (1 << 6),  //NK_EDIT_ALWAYS_INSERT_MODE| NK_EDIT_SELECTABLE| NK_EDIT_MULTILINE|NK_EDIT_ALLOW_TAB|NK_EDIT_CLIPBOARD,
    NK_EDIT_EDITOR  = (1 << 5) | (1 << 11) | (1 << 3) | (1 << 6), //NK_EDIT_SELECTABLE|NK_EDIT_MULTILINE|NK_EDIT_ALLOW_TAB| NK_EDIT_CLIPBOARD
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkEditEvents {
    NK_EDIT_ACTIVE      = 1 << 0, //NK_FLAG(0), /* edit widget is currently being modified */
    NK_EDIT_INACTIVE    = 1 << 1, //NK_FLAG(1), /* edit widget is not active and is not being modified */
    NK_EDIT_ACTIVATED   = 1 << 2, //NK_FLAG(2), /* edit widget went from state inactive to state active */
    NK_EDIT_DEACTIVATED = 1 << 3, //NK_FLAG(3), /* edit widget went from state active to state inactive */
    NK_EDIT_COMMITED    = 1 << 4, //NK_FLAG(4) /* edit widget has received an enter and lost focus */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkPanelFlags {
    NK_WINDOW_BORDER            = 1 << 0, //NK_FLAG(0), /* Draws a border around the window to visually separate the window * from the background */
    NK_WINDOW_BORDER_HEADER     = 1 << 1, //NK_FLAG(1), /* Draws a border between window header and body */
    NK_WINDOW_MOVABLE           = 1 << 2, //NK_FLAG(2), /* The movable flag indicates that a window can be moved by user input or * by dragging the window header */
    NK_WINDOW_SCALABLE          = 1 << 3, //NK_FLAG(3), /* The scalable flag indicates that a window can be scaled by user input * by dragging a scaler icon at the button of the window */
    NK_WINDOW_CLOSABLE          = 1 << 4, //NK_FLAG(4), /* adds a closable icon into the header */
    NK_WINDOW_MINIMIZABLE       = 1 << 5, //NK_FLAG(5), /* adds a minimize icon into the header */
    NK_WINDOW_DYNAMIC           = 1 << 6, //NK_FLAG(6), /* special window type growing up in height while being filled to a * certain maximum height */
    NK_WINDOW_NO_SCROLLBAR      = 1 << 7, //NK_FLAG(7), /* Removes the scrollbar from the window */
    NK_WINDOW_TITLE             = 1 << 8, //NK_FLAG(8), /* Forces a header at the top at the window showing the title */
    NK_WINDOW_SCROLL_AUTO_HIDE  = 1 << 9, //NK_FLAG(9), /* Automatically hides the window scrollbar if no user interaction */
    NK_WINDOW_BACKGROUND        = 1 << 10, //NK_FLAG(10) /* Always keep window in the background */
}

extern "C" {
	/* context */
	//#ifdef NK_INCLUDE_DEFAULT_ALLOCATOR
	//pub fn nk_init_default(ctx: *mut NkContext, font: *const NkUserFont) -> c_int;
	//#endif
	pub fn nk_init_fixed(ctx: *mut NkContext, memory: *mut c_void, size: NkSize, font: *const NkUserFont);
	pub fn nk_init_custom(ctx: *mut NkContext, cmds: *mut NkBuffer, pool: *mut NkBuffer, font: *const NkUserFont);
	pub fn nk_init(ctx: *mut NkContext, allocator: *mut NkAllocator, font: *const NkUserFont);
	pub fn nk_clear(ctx: *mut NkContext);
	pub fn nk_free(ctx: *mut NkContext);
	//#ifdef NK_INCLUDE_COMMAND_USERDATA
	pub fn nk_set_user_data(ctx: *mut NkContext, handle: NkHandle);
	//#endif
}

extern "C" {
	/* window */
	pub fn nk_begin(ctx: *mut NkContext, panel: *mut NkPanel, title: *const c_char, bounds: NkRect, flags: NkFlags) -> c_int;
	pub fn nk_begin_titled(ctx: *mut NkContext, panel: *mut NkPanel, name: *const c_char, id: *const c_char, bounds: NkRect, flags: NkFlags) -> c_int;
	pub fn nk_end(ctx: *mut NkContext);

	pub fn nk_window_find(ctx: *mut NkContext, name: *const c_char) -> *mut NkWindow;
	pub fn nk_window_get_bounds(ctx: *mut NkContext) -> NkRect;
	pub fn nk_window_get_position(ctx: *mut NkContext) -> NkVec2;
	pub fn nk_window_get_size(ctx: *const NkContext) -> NkVec2;
	pub fn nk_window_get_width(ctx: *const NkContext) -> c_float;
	pub fn nk_window_get_height(ctx: *const NkContext) -> c_float;
	pub fn nk_window_get_panel(ctx: *mut NkContext) -> *mut NkPanel;
	pub fn nk_window_get_content_region(ctx: *mut NkContext) -> NkRect;
	pub fn nk_window_get_content_region_min(ctx: *mut NkContext) -> NkVec2;
	pub fn nk_window_get_content_region_max(ctx: *mut NkContext) -> NkVec2;
	pub fn nk_window_get_content_region_size(ctx: *mut NkContext) -> NkVec2;
	pub fn nk_window_get_canvas(ctx: *mut NkContext) -> *mut NkCommandBuffer;

	pub fn nk_window_has_focus(ctx: *const NkContext) -> c_int;
	pub fn nk_window_is_collapsed(ctx: *mut NkContext, x: *const c_char) -> c_int;
	pub fn nk_window_is_closed(ctx: *mut NkContext, x: *const c_char) -> c_int;
	pub fn nk_window_is_hidden(ctx: *mut NkContext, x: *const c_char) -> c_int;
	pub fn nk_window_is_active(ctx: *mut NkContext, x: *const c_char) -> c_int;
	pub fn nk_window_is_hovered(ctx: *mut NkContext) -> c_int;
	pub fn nk_window_is_any_hovered(ctx: *mut NkContext) -> c_int;
	pub fn nk_item_is_any_active(ctx: *mut NkContext) -> c_int;

	pub fn nk_window_set_bounds(ctx: *mut NkContext, bounds: NkRect);
	pub fn nk_window_set_position(ctx: *mut NkContext, pos: NkVec2);
	pub fn nk_window_set_size(ctx: *mut NkContext, size: NkVec2);
	pub fn nk_window_set_focus(ctx: *mut NkContext, name: *const c_char);

	pub fn nk_window_close(ctx: *mut NkContext, name: *const c_char);
	pub fn nk_window_collapse(ctx: *mut NkContext, name: *const c_char, state :NkCollapseStates);
	pub fn nk_window_collapse_if(ctx: *mut NkContext, name: *const c_char, state: NkCollapseStates, cond: c_int);
	pub fn nk_window_show(ctx: *mut NkContext, name: *const c_char, state: NkShowStates);
	pub fn nk_window_show_if(ctx: *mut NkContext, name: *const c_char, state: NkShowStates, cond: c_int);
}

extern "C" {
	/* Layout */
	pub fn nk_layout_row_dynamic(ctx: *mut NkContext, height: c_float, cols: c_int);
	pub fn nk_layout_row_static(ctx: *mut NkContext, height: c_float, item_width: c_int, cols: c_int);

	pub fn nk_layout_row_begin(ctx: *mut NkContext, fmt: NkLayoutFormat, row_height: c_float, cols: c_int);
	pub fn nk_layout_row_push(ctx: *mut NkContext, value: c_float);
	pub fn nk_layout_row_end(ctx: *mut NkContext);
	pub fn nk_layout_row(ctx: *mut NkContext, fmt: NkLayoutFormat, height: c_float, cols: c_int, ratio: *const c_float);

	pub fn nk_layout_space_begin(ctx: *mut NkContext, fmt: NkLayoutFormat, height: c_float, widget_count: c_int);
	pub fn nk_layout_space_push(ctx: *mut NkContext, size: NkRect);
	pub fn nk_layout_space_end(ctx: *mut NkContext);

	pub fn nk_layout_space_bounds(ctx: *mut NkContext) -> NkRect;
	pub fn nk_layout_space_to_screen(ctx: *mut NkContext, size: NkVec2) -> NkVec2;
	pub fn nk_layout_space_to_local(ctx: *mut NkContext, size: NkVec2) -> NkVec2;
	pub fn nk_layout_space_rect_to_screen(ctx: *mut NkContext, size: NkRect) -> NkRect;
	pub fn nk_layout_space_rect_to_local(ctx: *mut NkContext, size: NkRect) -> NkRect;
	pub fn nk_layout_ratio_from_pixel(ctx: *mut NkContext, pixel_width: c_float) -> c_float;
}

extern "C" {
	/* Layout: Group */
	pub fn nk_group_begin(ctx: *mut NkContext, panel: *mut NkPanel, title: *const c_char, flags: NkFlags) -> c_int;
	pub fn nk_group_end(ctx: *mut NkContext);
}

extern "C" {
	/* Layout: Tree */
	pub fn nk_tree_push(ctx: *mut NkContext, ty: NkTreeType, title: *const c_char, state: NkCollapseStates) -> c_int;
	pub fn nk_tree_push_id(ctx: *mut NkContext, ty: NkTreeType, title: *const c_char, state: NkCollapseStates, id: c_int)  -> c_int;
		
	pub fn nk_tree_image_push(ctx: *mut NkContext, ty: NkTreeType, title: *const c_char, state: NkCollapseStates) -> c_int;
	pub fn nk_tree_image_push_id(ctx: *mut NkContext, ty: NkTreeType, title: *const c_char, state: NkCollapseStates, id: c_int) -> c_int;
}

extern "C" {
	pub fn nk_tree_push_hashed(ctx: *mut NkContext, ty: NkTreeType, title: *const c_char, initial_state: NkCollapseStates, hash: *const c_char, len: c_int, seed: c_int) -> c_int;
	pub fn nk_tree_image_push_hashed(ctx: *mut NkContext, ty: NkTreeType, img: NkImage, title: *const c_char, initial_state: NkCollapseStates , hash: *const c_char, len: c_int, seed: c_int) -> c_int;
	pub fn nk_tree_pop(ctx: *mut NkContext);
}	

extern "C" {
	/* Widgets */
	pub fn nk_text(ctx: *mut NkContext, text: *const c_char, len: c_int, flags: NkFlags);
	pub fn nk_text_colored(ctx: *mut NkContext, text: *const c_char, len: c_int, flags: NkFlags, color: NkColor);
	pub fn nk_text_wrap(ctx: *mut NkContext, text: *const c_char, len: c_int);
	pub fn nk_text_wrap_colored(ctx: *mut NkContext, text: *const c_char, len: c_int, color: NkColor);

	pub fn nk_label(ctx: *mut NkContext, label: *const c_char, align: NkFlags);
	pub fn nk_label_colored(ctx: *mut NkContext, label: *const c_char, align: NkFlags, color: NkColor);
	pub fn nk_label_wrap(ctx: *mut NkContext, label: *const c_char);
	pub fn nk_label_colored_wrap(ctx: *mut NkContext, label: *const c_char, color: NkColor);
	pub fn nk_image(ctx: *mut NkContext, img: NkImage);
//#ifdef NK_INCLUDE_STANDARD_VARARGS
	pub fn nk_labelf(ctx: *mut NkContext, align: NkFlags, labels: *const c_char, ...);
	pub fn nk_labelf_colored(ctx: *mut NkContext, align: NkFlags, color: NkColor, labels: *const c_char,...);
	pub fn nk_labelf_wrap(ctx: *mut NkContext, labels: *const c_char, ...);
	pub fn nk_labelf_colored_wrap(ctx: *mut NkContext, color: NkColor, labels: *const c_char, ...);

	pub fn nk_value_bool(ctx: *mut NkContext, prefix: *const c_char, val: c_int);
	pub fn nk_value_int(ctx: *mut NkContext, prefix: *const c_char, val: c_int);
	pub fn nk_value_uint(ctx: *mut NkContext, prefix: *const c_char , val: c_uint);
	pub fn nk_value_float(ctx: *mut NkContext, prefix: *const c_char , val: c_float);
	pub fn nk_value_color_byte(ctx: *mut NkContext, prefix: *const c_char , color: NkColor);
	pub fn nk_value_color_float(ctx: *mut NkContext, prefix: *const c_char , color: NkColor);
	pub fn nk_value_color_hex(ctx: *mut NkContext, prefix: *const c_char , color: NkColor);
//#endif
}

extern "C" {
	/* Widgets: Buttons */
	pub fn nk_button_set_behavior(ctx: *mut NkContext, b: NkButtonBehavior);
	pub fn nk_button_text(ctx: *mut NkContext, title: *const c_char, len: c_int) -> c_int;
	pub fn nk_button_label(ctx: *mut NkContext, title: *const c_char) -> c_int;
	pub fn nk_button_color(ctx: *mut NkContext, color: NkColor) -> c_int;
	pub fn nk_button_symbol(ctx: *mut NkContext, ty: NkSymbolType) -> c_int;
	pub fn nk_button_image(ctx: *mut NkContext, img: NkImage) -> c_int;
	pub fn nk_button_symbol_label(ctx: *mut NkContext, ty: NkSymbolType, sym: *const c_char, text_alignment: NkFlags) -> c_int;
	pub fn nk_button_symbol_text(ctx: *mut NkContext, ty: NkSymbolType, sym: *const c_char, len: c_int, alignment: NkFlags) -> c_int;
	pub fn nk_button_image_label(ctx: *mut NkContext, img: NkImage, sym: *const c_char, text_alignment: NkFlags) -> c_int;
	pub fn nk_button_image_text(ctx: *mut NkContext, img: NkImage, sym: *const c_char, len: c_int, alignment: NkFlags) -> c_int;
}

extern "C" {
	/* Widgets: Checkbox */
	pub fn nk_check_label(ctx: *mut NkContext, label: *const c_char, active: c_int) -> c_int;
	pub fn nk_check_text(ctx: *mut NkContext, text: *const c_char, len: c_int, active: c_int) -> c_int;
	pub fn nk_check_flags_label(ctx: *mut NkContext, label: *const c_char, flags: c_uint, value: c_uint) -> c_uint;
	pub fn nk_check_flags_text(ctx: *mut NkContext, text: *const c_char, len: c_int, flags: c_uint, value: c_uint) -> c_uint;
	pub fn nk_checkbox_label(ctx: *mut NkContext, label: *const c_char, active: *mut c_int) -> c_int;
	pub fn nk_checkbox_text(ctx: *mut NkContext, text: *const c_char, len: c_int, active: *mut c_int) -> c_int;
	pub fn nk_checkbox_flags_label(ctx: *mut NkContext, label: *const c_char, flags: *mut c_uint, value: c_uint) -> c_int;
	pub fn nk_checkbox_flags_text(ctx: *mut NkContext, text: *const c_char, len: c_int, flags: *mut c_uint, value: c_uint) -> c_int;
}

extern "C" {
	/* Widgets: Radio */
	pub fn nk_radio_label(ctx: *mut NkContext, label: *const c_char, active: *mut c_int) -> c_int;
	pub fn nk_radio_text(ctx: *mut NkContext, text: *const c_char, len: c_int, active: *mut c_int) -> c_int;
	pub fn nk_option_label(ctx: *mut NkContext, label: *const c_char, active: c_int) -> c_int;
	pub fn nk_option_text(ctx: *mut NkContext, text: *const c_char, len: c_int, active: c_int) -> c_int;
}

extern "C" {
	/* Widgets: Selectable */
	pub fn nk_selectable_label(ctx: *mut NkContext, label: *const c_char, align: NkFlags, value: *mut c_int) -> c_int;
	pub fn nk_selectable_text(ctx: *mut NkContext, text: *const c_char, len: c_int, align: NkFlags, value: *mut c_int) -> c_int;
	pub fn nk_selectable_image_label(ctx: *mut NkContext, img: NkImage,  label: *const c_char, align: NkFlags, value: *mut c_int) -> c_int;
	pub fn nk_selectable_image_text(ctx: *mut NkContext, img: NkImage, text: *const c_char, len: c_int, align: NkFlags, value: *mut c_int) -> c_int;

	pub fn nk_select_label(ctx: *mut NkContext, label: *const c_char, align: NkFlags, value: c_int) -> c_int;
	pub fn nk_select_text(ctx: *mut NkContext, text: *const c_char, len: c_int, align: NkFlags, value: c_int) -> c_int;
	pub fn nk_select_image_label(ctx: *mut NkContext, img: NkImage, label: *const c_char, align: NkFlags, value: c_int) -> c_int;
	pub fn nk_select_image_text(ctx: *mut NkContext, img: NkImage, text: *const c_char, len: c_int, align: NkFlags, value: c_int) -> c_int;
}

extern "C" {
	/* Widgets: Slider */
	pub fn nk_slide_float(ctx: *mut NkContext, min: c_float, val: c_float, max: c_float, step: c_float) -> c_float;
	pub fn nk_slide_int(ctx: *mut NkContext, min: c_int, val: c_int, max: c_int, step: c_int) -> c_int;
	pub fn nk_slider_float(ctx: *mut NkContext, min: c_float, val: *mut c_float, max: c_float, step: c_float) -> c_int;
	pub fn nk_slider_int(ctx: *mut NkContext, min: c_int, val: *mut c_int, max: c_int, step: c_int) -> c_int;
}

extern "C" {
	/* Widgets: Progressbar */
	pub fn nk_progress(ctx: *mut NkContext, cur: *mut NkSize, max: NkSize, modifyable: c_int) -> c_int;
	pub fn nk_prog(ctx: *mut NkContext, cur: NkSize, max: NkSize, modifyable: c_int) -> NkSize;
}

extern "C" {
	/* Widgets: Color picker */
	pub fn nk_color_picker(ctx: *mut NkContext, color: NkColor, fmt: NkColorFormat) -> NkColor;
	pub fn nk_color_pick(ctx: *mut NkContext, color: *mut NkColor, fmt: NkColorFormat) -> c_int;
}

extern "C" {
	/* Widgets: Property */
	pub fn nk_property_int(layout: *mut NkContext, name: *const c_char, min: c_int, val: *mut c_int, max: c_int, step: c_int, inc_per_pixel: c_float);
	pub fn nk_property_float(layout: *mut NkContext, name: *const c_char, min: c_float, val: *mut c_float, max: c_float, step: c_float, inc_per_pixel: c_float);
	pub fn nk_property_double(layout: *mut NkContext, name: *const c_char, min: c_double, val: *mut c_double, max: c_double, step: c_double, inc_per_pixel: c_float);
	pub fn nk_propertyi(layout: *mut NkContext, name: *const c_char, min: c_int, val: c_int, max: c_int, step: c_int, inc_per_pixel: c_float) -> c_int;
	pub fn nk_propertyf(layout: *mut NkContext, name: *const c_char, min: c_float, val: c_float, max: c_float, step: c_float, inc_per_pixel: c_float) -> c_float;
	pub fn nk_propertyd(layout: *mut NkContext, name: *const c_char, min: c_double, val: c_double, max: c_double, step: c_double, inc_per_pixel: c_float) -> c_double;
}

extern "C" {
	/* Widgets: TextEdit */
	pub fn nk_edit_string(ctx: *mut NkContext, flags: NkFlags, buffer: *mut c_char, len: *mut c_int, max: c_int, filter: NkFilter) -> NkFlags;
	pub fn nk_edit_buffer(ctx: *mut NkContext, flags: NkFlags, text_edit: *mut NkTextEdit, filter: NkFilter) -> NkFlags;
	pub fn nk_edit_string_zero_terminated(ctx: *mut NkContext, flags: NkFlags, buffer: *mut c_char, max: c_int, filter: NkFilter) -> NkFlags;
}

extern "C" {
	/* c_chart */
	pub fn nk_chart_begin(ctx: *mut NkContext, ty: NkChartType, num: c_int, min: c_float, max: c_float) -> c_int;
	pub fn nk_chart_begin_colored(ctx: *mut NkContext, ty: NkChartType, color: NkColor, active: NkColor, num: c_int, min: c_float, max: c_float) -> c_int;
	pub fn nk_chart_add_slot(ctx: *mut NkContext, ty: NkChartType, count: c_int, min_value: c_float, max_value: c_float);
	pub fn nk_chart_add_slot_colored(ctx: *mut NkContext, ty: NkChartType, color: NkColor, active: NkColor, count: c_int, min_value: c_float, max_value: c_float);
	pub fn nk_chart_push(ctx: *mut NkContext, val: c_float) -> NkFlags;
	pub fn nk_chart_push_slot(ctx: *mut NkContext, val: c_float, slot: c_int) -> NkFlags;
	pub fn nk_chart_end(ctx: *mut NkContext);
	pub fn nk_plot(ctx: *mut NkContext, ty: NkChartType, values: *const c_float, count: c_int, offset: c_int);
	pub fn nk_plot_function(ctx: *mut NkContext, ty: NkChartType, userdata: *mut c_void, value_getter: Option<extern "C" fn(user: *mut c_void, index: c_int) -> c_float>, count: c_int, offset: c_int);
}

extern "C" {
	/* Popups */
	pub fn nk_popup_begin(ctx: *mut NkContext, panel: *mut NkPanel, ty: NkPopupType, label: *const c_char, flags: NkFlags, bounds: NkRect) -> c_int;
	pub fn nk_popup_close(ctx: *mut NkContext);
	pub fn nk_popup_end(ctx: *mut NkContext);
}

extern "C" {
	/* Combobox */
	pub fn nk_combo(ctx: *mut NkContext, items: *const *mut c_char, count: c_int, selected: c_int, item_height: c_int) -> c_int;
	pub fn nk_combo_separator(ctx: *mut NkContext, items_separated_by_separator: *const c_char, separator: c_int, selected: c_int, count: c_int, item_height: c_int) -> c_int;
	pub fn nk_combo_string(ctx: *mut NkContext, items_separated_by_zeros: *const c_char, selected: c_int, count: c_int, item_height: c_int) -> c_int;
	pub fn nk_combo_callback(ctx: *mut NkContext, item_getter: Option<extern "C" fn(v: *mut c_void, i: c_int, c: *const *mut c_char)>, userdata: *mut c_void, selected: c_int, count: c_int, item_height: c_int) -> c_int;
	pub fn nk_combobox(ctx: *mut NkContext, items: *const *mut c_char, count: c_int, selected: *mut c_int, item_height: c_int);
	pub fn nk_combobox_string(ctx: *mut NkContext, items_separated_by_zeros: *const c_char, selected: *mut c_int, count: c_int, item_height: c_int);
	pub fn nk_combobox_separator(ctx: *mut NkContext, items_separated_by_separator: *const c_char, separator: c_int, selected: *mut c_int, count: c_int, item_height: c_int);
	pub fn nk_combobox_callback(ctx: *mut NkContext, item_getter: Option<extern "C" fn(v: *mut c_void, i: c_int, c: *const *mut c_char)>, userdata: *mut c_void, selected: *mut c_int, count: c_int, item_height: c_int);
}

extern "C" {
	/* Combobox: abstract */
	pub fn nk_combo_begin_text(ctx: *mut NkContext, panel: *mut NkPanel, selected: *const c_char, len: c_int, max_height: c_int) -> c_int;
	pub fn nk_combo_begin_label(ctx: *mut NkContext, panel: *mut NkPanel, selected: *const c_char, max_height: c_int) -> c_int;
	pub fn nk_combo_begin_color(ctx: *mut NkContext, panel: *mut NkPanel, color: NkColor, max_height: c_int) -> c_int;
	pub fn nk_combo_begin_symbol(ctx: *mut NkContext, panel: *mut NkPanel, ty: NkSymbolType,  max_height: c_int) -> c_int;
	pub fn nk_combo_begin_symbol_label(ctx: *mut NkContext, panel: *mut NkPanel, selected: *const c_char, ty: NkSymbolType, height: c_int) -> c_int;
	pub fn nk_combo_begin_symbol_text(ctx: *mut NkContext, panel: *mut NkPanel, selected: *const c_char, len: c_int, ty: NkSymbolType, height: c_int) -> c_int;
	pub fn nk_combo_begin_image(ctx: *mut NkContext, panel: *mut NkPanel, img: NkImage,  max_height: c_int) -> c_int;
	pub fn nk_combo_begin_image_label(ctx: *mut NkContext, panel: *mut NkPanel, selected: *const c_char, img: NkImage, height: c_int) -> c_int;
	pub fn nk_combo_begin_image_text(ctx: *mut NkContext, panel: *mut NkPanel, selected: *const c_char, len: c_int, img: NkImage, height: c_int) -> c_int;
	pub fn nk_combo_item_label(ctx: *mut NkContext, label: *const c_char, alignment: NkFlags) -> c_int;
	pub fn nk_combo_item_text(ctx: *mut NkContext, label: *const c_char, len: c_int, alignment: NkFlags) -> c_int;
	pub fn nk_combo_item_image_label(ctx: *mut NkContext, img: NkImage, label: *const c_char, alignment: NkFlags) -> c_int;
	pub fn nk_combo_item_image_text(ctx: *mut NkContext, img: NkImage, text: *const c_char, len: c_int, alignment: NkFlags) -> c_int;
	pub fn nk_combo_item_symbol_label(ctx: *mut NkContext, ty: NkSymbolType, label: *const c_char, alignment: NkFlags) -> c_int;
	pub fn nk_combo_item_symbol_text(ctx: *mut NkContext, ty: NkSymbolType, text: *const c_char, len: c_int, alignment: NkFlags) -> c_int;
	pub fn nk_combo_close(ctx: *mut NkContext);
	pub fn nk_combo_end(ctx: *mut NkContext);
}

extern "C" {
	/* Contextual */
	pub fn nk_contextual_begin(ctx: *mut NkContext, panel: *mut NkPanel, flags: NkFlags, size: NkVec2, trigger_bounds: NkRect ) -> c_int;
	pub fn nk_contextual_item_text(ctx: *mut NkContext, text: *const c_char, len: c_int, align: NkFlags) -> c_int;
	pub fn nk_contextual_item_label(ctx: *mut NkContext, label: *const c_char, align: NkFlags) -> c_int;
	pub fn nk_contextual_item_image_label(ctx: *mut NkContext, img: NkImage, label: *const c_char, alignment: NkFlags) -> c_int;
	pub fn nk_contextual_item_image_text(ctx: *mut NkContext, img: NkImage, label: *const c_char, len: c_int, alignment: NkFlags) -> c_int;
	pub fn nk_contextual_item_symbol_label(ctx: *mut NkContext, ty: NkSymbolType, label: *const c_char, alignment: NkFlags) -> c_int;
	pub fn nk_contextual_item_symbol_text(ctx: *mut NkContext, ty: NkSymbolType, text: *const c_char, len: c_int, alignment: NkFlags) -> c_int;
	pub fn nk_contextual_close(ctx: *mut NkContext);
	pub fn nk_contextual_end(ctx: *mut NkContext);
}

extern "C" {
	/* Tooltip */
	pub fn nk_tooltip(ctx: *mut NkContext, tip: *const c_char);
	pub fn nk_tooltip_begin(ctx: *mut NkContext, panel: *mut NkPanel, width: c_float) -> c_int;
	pub fn nk_tooltip_end(ctx: *mut NkContext);
}

extern "C" {
	/* Menu */
	pub fn nk_menubar_begin(ctx: *mut NkContext);
	pub fn nk_menubar_end(ctx: *mut NkContext);
	pub fn nk_menu_begin_text(ctx: *mut NkContext, panel: *mut NkPanel, text: *const c_char, len: c_int, align: NkFlags, width: c_float) -> c_int;
	pub fn nk_menu_begin_label(ctx: *mut NkContext, panel: *mut NkPanel, label: *const c_char, align: NkFlags, width: c_float) -> c_int;
	pub fn nk_menu_begin_image(ctx: *mut NkContext, panel: *mut NkPanel, label: *const c_char, img: NkImage, width: c_float) -> c_int;
	pub fn nk_menu_begin_image_text(ctx: *mut NkContext, panel: *mut NkPanel, text: *const c_char, len: c_int,align: NkFlags, img: NkImage, width: c_float) -> c_int;
	pub fn nk_menu_begin_image_label(ctx: *mut NkContext, panel: *mut NkPanel, label: *const c_char, align: NkFlags, img: NkImage, width: c_float) -> c_int;
	pub fn nk_menu_begin_symbol(ctx: *mut NkContext, panel: *mut NkPanel, label: *const c_char, ty: NkSymbolType, width: c_float) -> c_int;
	pub fn nk_menu_begin_symbol_text(ctx: *mut NkContext, panel: *mut NkPanel, text: *const c_char, len: c_int,align: NkFlags, ty: NkSymbolType, width: c_float) -> c_int;
	pub fn nk_menu_begin_symbol_label(ctx: *mut NkContext, panel: *mut NkPanel, label: *const c_char, align: NkFlags, ty: NkSymbolType, width: c_float) -> c_int;
	pub fn nk_menu_item_text(ctx: *mut NkContext, text: *const c_char, len: c_int, align: NkFlags) -> c_int;
	pub fn nk_menu_item_label(ctx: *mut NkContext, label: *const c_char, alignment: NkFlags) -> c_int;
	pub fn nk_menu_item_image_label(ctx: *mut NkContext, img: NkImage, label: *const c_char, alignment: NkFlags) -> c_int;
	pub fn nk_menu_item_image_text(ctx: *mut NkContext, img: NkImage, label: *const c_char, len: c_int, alignment: NkFlags) -> c_int;
	pub fn nk_menu_item_symbol_text(ctx: *mut NkContext, ty: NkSymbolType, text: *const c_char, len: c_int, alignment: NkFlags) -> c_int;
	pub fn nk_menu_item_symbol_label(ctx: *mut NkContext, ty: NkSymbolType, label: *const c_char, alignment: NkFlags) -> c_int;
	pub fn nk_menu_close(ctx: *mut NkContext);
	pub fn nk_menu_end(ctx: *mut NkContext);
}

/* Drawing*/
/*fn nk_foreach(c, ctx) {
	for((c)=nk__begin(ctx); (c)!=0; (c)=nk__next(ctx, c))
}*/
//#ifdef NK_INCLUDE_VERTEX_BUFFER_OUTPUT

extern "C" {
	pub fn nk_convert(ctx: *mut NkContext, cmds: *mut NkBuffer, vertices: *mut NkBuffer, elements: *mut NkBuffer, config: *const NkConvertColor);
}
	
/*fn nk_draw_foreach(cmd,ctx, b) {
	for((cmd)=nk__draw_begin(ctx, b); (cmd)!=0; (cmd)=nk__draw_next(cmd, b, ctx))
}*/
//#endif

extern "C" {
	/* User Input */
	pub fn nk_input_begin(ctx: *mut NkContext);
	pub fn nk_input_motion(ctx: *mut NkContext, x: c_int, y: c_int);
	pub fn nk_input_key(ctx: *mut NkContext, key: NkKeys, down: c_int);
	pub fn nk_input_button(ctx: *mut NkContext, btn: NkButtons, x: c_int, y: c_int, down: c_int);
	pub fn nk_input_scroll(ctx: *mut NkContext, y: c_float);
	pub fn nk_input_char(ctx: *mut NkContext, c: c_char);
	pub fn nk_input_glyph(ctx: *mut NkContext, g: NkGlyph);
	pub fn nk_input_unicode(ctx: *mut NkContext, r: NkRune);
	pub fn nk_input_end(ctx: *mut NkContext);
}

extern "C" {
	/* Style */
	pub fn nk_style_default(ctx: *mut NkContext);
	pub fn nk_style_from_table(ctx: *mut NkContext, color: *const NkColor);
	pub fn nk_style_load_cursor(ctx: *mut NkContext, style: NkStyleCursor, cursor: *const NkCursor);
	pub fn nk_style_load_all_cursors(ctx: *mut NkContext, cursor: *mut NkCursor);
	pub fn nk_style_color_name(style: NkStyleColors) -> *const c_char;
	pub fn nk_style_set_font(ctx: *mut NkContext, font: *const NkUserFont);
	pub fn nk_style_set_cursor(ctx: *mut NkContext, style: NkStyleCursor) -> c_int;
	pub fn nk_style_show_cursor(ctx: *mut NkContext);
	pub fn nk_style_hide_cursor(ctx: *mut NkContext);
}

extern "C" {
	/* Utilities */
	pub fn nk_widget_bounds(ctx: *mut NkContext) -> NkRect;
	pub fn nk_widget_position(ctx: *mut NkContext) -> NkVec2;
	pub fn nk_widget_size(ctx: *mut NkContext) -> NkVec2;
	pub fn nk_widget_is_hovered(ctx: *mut NkContext) -> c_int;
	pub fn nk_widget_is_mouse_clicked(ctx: *mut NkContext, btn: NkButtons) -> c_int;
	pub fn nk_widget_has_mouse_click_down(ctx: *mut NkContext, btn: NkButtons, down: c_int) -> c_int;
	pub fn nk_spacing(ctx: *mut NkContext, cols: c_int);
}

extern "C" {
	/* base widget function  */
	pub fn nk_widget(rect: *mut NkRect, ctx: *mut NkContext) -> NkWidgetLayoutStates;
	pub fn nk_widget_fitting(rect: *mut NkRect, ctx: *mut NkContext, bounds: NkVec2) -> NkWidgetLayoutStates;
}

extern "C" {
	/* color (conversion user --> nuklear) */
	pub fn nk_rgb(r: c_int, g: c_int, b: c_int) -> NkColor;
	pub fn nk_rgb_iv(rgb: *const c_int) -> NkColor;
	pub fn nk_rgb_bv(rgb: *const NkByte) -> NkColor;
	pub fn nk_rgb_f(r: c_float, g: c_float, b: c_float) -> NkColor;
	pub fn nk_rgb_fv(rgb: *const c_float) -> NkColor;
	pub fn nk_rgb_hex(rgb: *const c_char) -> NkColor;

	pub fn nk_rgba(r: c_int, g: c_int, b: c_int, a: c_int) -> NkColor;
	pub fn nk_rgba_u32(rgba: NkUint) -> NkColor;
	pub fn nk_rgba_iv(rgba: *const c_int) -> NkColor;
	pub fn nk_rgba_bv(rgba: *const NkByte) -> NkColor;
	pub fn nk_rgba_f(r: c_float, g: c_float, b: c_float, a: c_float) -> NkColor;
	pub fn nk_rgba_fv(rgba: *const c_float) -> NkColor;
	pub fn nk_rgba_hex(rgb: *const c_char) -> NkColor;

	pub fn nk_hsv(h: c_int, s: c_int, v: c_int) -> NkColor;
	pub fn nk_hsv_iv(hsv: *const c_int) -> NkColor;
	pub fn nk_hsv_bv(hsv: *const NkByte) -> NkColor;
	pub fn nk_hsv_f(h: c_float, s: c_float, v: c_float) -> NkColor;
	pub fn nk_hsv_fv(hsv: *const c_float) -> NkColor;

	pub fn nk_hsva(h: c_int, s: c_int, v: c_int, a: c_int) -> NkColor;
	pub fn nk_hsva_iv(hsva: *const c_int) -> NkColor;
	pub fn nk_hsva_bv(hsva: *const NkByte) -> NkColor;
	pub fn nk_hsva_f(h: c_float, s: c_float, v: c_float, a: c_float) -> NkColor;
	pub fn nk_hsva_fv(hsva: *const c_float) -> NkColor;
}

extern "C" {
	/* color (conversion nuklear --> user) */
	pub fn nk_color_f(r: *mut c_float, g: *mut c_float, b: *mut c_float, a: *mut c_float, color: NkColor);
	pub fn nk_color_fv(rgba_out: *mut c_float, color: NkColor);
	pub fn nk_color_u32(color: NkColor) -> c_uint;
	pub fn nk_color_hex_rgba(output: *mut c_char, color: NkColor);
	pub fn nk_color_hex_rgb(output: *mut c_char, color: NkColor);

	pub fn nk_color_hsv_i(out_h: *mut c_int, out_s: *mut c_int, out_v: *mut c_int, color: NkColor);
	pub fn nk_color_hsv_b(out_h: *mut NkByte, out_s: *mut NkByte, out_v: *mut NkByte, color: NkColor);
	pub fn nk_color_hsv_iv(hsv_out: *mut c_int, color: NkColor);
	pub fn nk_color_hsv_bv(hsv_out: *mut NkByte, color: NkColor);
	pub fn nk_color_hsv_f(out_h: *mut c_float, out_s: *mut c_float, out_v: *mut c_float, color: NkColor);
	pub fn nk_color_hsv_fv(hsv_out: *mut c_float, color: NkColor);

	pub fn nk_color_hsva_i(h: *mut c_int, s: *mut c_int, v: *mut c_int, a: *mut c_int, color: NkColor);
	pub fn nk_color_hsva_b(h: *mut NkByte, s: *mut NkByte, v: *mut NkByte, a: *mut NkByte, color: NkColor);
	pub fn nk_color_hsva_iv(hsva_out: *mut c_int, color: NkColor);
	pub fn nk_color_hsva_bv(hsva_out: *mut NkByte, color: NkColor);
	pub fn nk_color_hsva_f(out_h: *mut c_float, out_s: *mut c_float, out_v: *mut c_float, out_a: *mut c_float, color: NkColor);
	pub fn nk_color_hsva_fv(hsva_out: *mut c_float, color: NkColor);
}

extern "C" {
	/* image */
	pub fn nk_handle_ptr(ptr: *mut c_void) -> NkHandle;
	pub fn nk_handle_id(id: c_int) -> NkHandle;
	pub fn nk_image_handle(h: NkHandle) -> NkImage;
	pub fn nk_image_ptr(ptr: *mut c_void) -> NkImage;
	pub fn nk_image_id(id: c_int) -> NkImage;
	pub fn nk_image_is_subimage(img: *const NkImage) -> c_int;
	pub fn nk_subimage_ptr(ptr: *mut c_void, w: c_ushort, h: c_ushort, sub_region: NkRect) -> NkImage;
	pub fn nk_subimage_id(id: c_int, w: c_ushort, h: c_ushort, sub_region: NkRect) -> NkImage;
	pub fn nk_subimage_handle(h: NkHandle, w: c_ushort, h: c_ushort, sub_region: NkRect) -> NkImage;
}

extern "C" {
	/* math */
	pub fn nk_murmur_hash(key: *const c_void, len: c_int, seed: NkHash) -> NkHash;
	pub fn nk_triangle_from_direction(result: *mut NkVec2, r: NkRect, pad_x: c_float , pad_y: c_float, head: NkHeading);

	pub fn nk_vec2(x: c_float, y: c_float) -> NkVec2;
	pub fn nk_vec2i(x: c_int, y: c_int) -> NkVec2;
	pub fn nk_vec2v(xy: *const c_float) -> NkVec2;
	pub fn nk_vec2iv(xy: *const c_int) -> NkVec2;

	pub fn nk_get_null_rect() -> NkRect;
	pub fn nk_rect(x: c_float, y: c_float, w: c_float, h: c_float) -> NkRect;
	pub fn nk_recti(x: c_int, y: c_int, w: c_int, h: c_int) -> NkRect;
	pub fn nk_recta(pos: NkVec2, size: NkVec2) -> NkRect;
	pub fn nk_rectv(xywh: *const c_float) -> NkRect;
	pub fn nk_rectiv(xywh: *const c_int) -> NkRect;
	pub fn nk_rect_pos(pos: NkRect) -> NkVec2;
	pub fn nk_rect_size(size: NkRect) -> NkVec2;
}

extern "C" {
	/* string*/
	pub fn nk_strlen(str: *const c_char) -> c_int;
	pub fn nk_stricmp(s1: *const c_char, s2: *const c_char) -> c_int;
	pub fn nk_stricmpn(s1: *const c_char, s2: *const c_char, n: c_int) -> c_int;
	pub fn nk_strtoi(str: *const c_char, endptr: *mut *mut c_char) -> c_int;
	pub fn nk_strtof(str: *const c_char, endptr: *mut *mut c_char)  -> c_float;
	pub fn nk_strtod(str: *const c_char, endptr: *mut *mut c_char) -> c_double;
	pub fn nk_strfilter(text: *const c_char, regexp: *const c_char) -> c_int;
	pub fn nk_strmatch_fuzzy_string(str: *const c_char, pattern: *const c_char, out_score: *mut c_int) -> c_int;
	pub fn nk_strmatch_fuzzy_text(txt: *const c_char, txt_len: c_int, pattern: *const c_char, out_score: *mut c_int) -> c_int;
//#ifdef NK_INCLUDE_STANDARD_VARARGS
	pub fn nk_strfmt(buf: *mut c_char, len: c_int, fmt: *const c_char, ...) -> c_int;
//#endif
}

extern "C" {
	/* UTF-8 */
	pub fn nk_utf_decode(i: *const c_char, r: *mut NkRune, len: c_int) -> c_int;
	pub fn nk_utf_encode(r: NkRune, o: *mut c_char, len: c_int) -> c_int;
	pub fn nk_utf_len(i: *const c_char, byte_len: c_int) -> c_int;
	pub fn nk_utf_at(buffer: *const c_char, length: c_int, index: c_int, unicode: *mut NkRune, len: *mut c_int) -> *const c_char;
}

/* ==============================================================
 *
 *                          MEMORY BUFFER
 *
 * ===============================================================*/
/*  A basic (double)-buffer with linear allocation and resetting as only
    freeing policy. The buffer's main purpose is to control all memory management
    inside the GUI toolkit and still leave memory control as much as possible in
    the hand of the user while also making sure the library is easy to use if
    not as much control is needed.
    In general all memory inside this library can be provided from the user in
    three different ways.

    The first way and the one providing most control is by just passing a fixed
    size memory block. In this case all control lies in the hand of the user
    since he can exactly control where the memory comes from and how much memory
    the library should consume. Of course using the fixed size API removes the
    ability to automatically resize a buffer if not enough memory is provided so
    you have to take over the resizing. While being a fixed sized buffer sounds
    quite limiting, it is very effective in this library since the actual memory
    consumption is quite stable and has a fixed upper bound for a lot of cases.

    If you don't want to think about how much memory the library should allocate
    at all time or have a very dynamic UI with unpredictable memory consumption
    habits but still want control over memory allocation you can use the dynamic
    allocator based API. The allocator consists of two callbacks for allocating
    and freeing memory and optional userdata so you can plugin your own allocator.

    The final and easiest way can be used by defining
    NK_INCLUDE_DEFAULT_ALLOCATOR which uses the standard library memory
    allocation functions malloc and free and takes over complete control over
    memory in this library.
*/
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkMemoryStatus {
    memory: *mut c_void,
    mtype: c_uint,
    size: NkSize,
    allocated: NkSize,
    needed: NkSize,
    calls: NkSize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkAllocationType {
    NK_BUFFER_FIXED,
    NK_BUFFER_DYNAMIC
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkBufferAllocationType {
    NK_BUFFER_FRONT,
    NK_BUFFER_BACK,
    //NK_BUFFER_MAX
}
pub const NK_BUFFER_MAX: usize = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkBufferMarker {
    active: c_int,
    offset: NkSize,
}

impl Default for NkBufferMarker {
	fn default() -> NkBufferMarker {
		NkBufferMarker {
		    active: 0,
		    offset: 0,
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkMemory {
	ptr: *mut c_void,
	size: NkSize
}

impl Default for NkMemory {
	fn default() -> NkMemory {
		NkMemory {
			ptr: ::std::ptr::null_mut(),
			size: 0,
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkBuffer {
    marker: [NkBufferMarker; NK_BUFFER_MAX],
    /* buffer marker to free a buffer to a certain offset */
    pool: NkAllocator,
    /* allocator callback for dynamic buffers */
    mtype: NkAllocationType,
    /* memory management type */
    memory: NkMemory,
    /* memory and size of the current memory block */
    grow_factor: c_float,
    /* growing factor for dynamic memory management */
    allocated: NkSize,
    /* total amount of memory allocated */
    needed: NkSize,
    /* totally consumed memory given that enough memory is present */
    calls: NkSize,
    /* number of allocation calls */
    size: NkSize,
    /* current size of the buffer */
}

impl Default for NkBuffer {
	fn default() -> NkBuffer {
		NkBuffer {
		    marker: [NkBufferMarker::default(); NK_BUFFER_MAX],
		    /* buffer marker to free a buffer to a certain offset */
		    pool: NkAllocator::default(),
		    /* allocator callback for dynamic buffers */
		    mtype: NkAllocationType::NK_BUFFER_FIXED,
		    /* memory management type */
		    memory: NkMemory::default(),
		    /* memory and size of the current memory block */
		    grow_factor: 0.0,
		    /* growing factor for dynamic memory management */
		    allocated: 0,
		    /* total amount of memory allocated */
		    needed: 0,
		    /* totally consumed memory given that enough memory is present */
		    calls: 0,
		    /* number of allocation calls */
		    size: 0,
		    /* current size of the buffer */
		}
	}
}

extern "C" {
//#ifdef NK_INCLUDE_DEFAULT_ALLOCATOR
	//pub fn nk_Buffer_init_default(buffer: *mut NkBuffer);
//#endif
	pub fn nk_Buffer_init(buffer: *mut NkBuffer, allocator: *const NkAllocator, size: NkSize);
	pub fn nk_Buffer_init_fixed(buffer: *mut NkBuffer, memory: *mut c_void, size: NkSize);
	pub fn nk_Buffer_info(status: *mut NkMemoryStatus, buffer: *mut NkBuffer);
	pub fn nk_Buffer_push(buffer: *mut NkBuffer, ty: NkBufferAllocationType, memory: *const c_void, size: NkSize, align: NkSize);
	pub fn nk_Buffer_mark(buffer: *mut NkBuffer, ty: NkBufferAllocationType);
	pub fn nk_Buffer_reset(buffer: *mut NkBuffer, ty: NkBufferAllocationType);
	pub fn nk_Buffer_clear(buffer: *mut NkBuffer);
	pub fn nk_Buffer_free(buffer: *mut NkBuffer);
	pub fn nk_Buffer_memory(buffer: *mut NkBuffer) -> *mut c_void;
	pub fn nk_Buffer_memory_const(buffer: *const NkBuffer) -> *const c_void;
	pub fn nk_Buffer_total(buffer: *mut NkBuffer) -> NkSize;
}

/* ==============================================================
 *
 *                          STRING
 *
 * ===============================================================*/
/*  Basic string buffer which is only used in context with the text editor
 *  to manage and manipulate dynamic or fixed size string content. This is _NOT_
 *  the default string handling method.*/
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkStr {
    buffer: NkBuffer,
    len: c_int, /* in codepoints/runes/glyphs */
}

impl Default for NkStr {
	fn default() -> NkStr {
		NkStr {
		    buffer: NkBuffer::default(),
		    len: 0, /* in codepoints/runes/glyphs */
		}
	}
}

extern "C" {
	//#ifdef NK_INCLUDE_DEFAULT_ALLOCATOR
	//pub fn nk_str_init_default(str: *mut NkStr);
//#endif
	pub fn nk_str_init(str: *mut NkStr, allocator: *const NkAllocator, size: NkSize);
	pub fn nk_str_init_fixed(str: *mut NkStr, memory: *mut c_void, size: NkSize);
	pub fn nk_str_clear(str: *mut NkStr);
	pub fn nk_str_free(str: *mut NkStr);

	pub fn nk_str_append_text_char(str: *mut NkStr, label: *const c_char, len: c_int) -> c_int;
	pub fn nk_str_append_str_char(str: *mut NkStr, c: *const c_char) -> c_int;
	pub fn nk_str_append_text_utf8(str: *mut NkStr, label: *const c_char, len: c_int) -> c_int;
	pub fn nk_str_append_str_utf8(str: *mut NkStr, c: *const c_char) -> c_int;
	pub fn nk_str_append_text_runes(str: *mut NkStr, r: *const NkRune, len :c_int) -> c_int;
	pub fn nk_str_append_str_runes(str: *mut NkStr, r: *const NkRune) -> c_int;

	pub fn nk_str_insert_at_char(str: *mut NkStr, pos: c_int, label: *const c_char, len: c_int) -> c_int;
	pub fn nk_str_insert_at_rune(str: *mut NkStr, pos: c_int, label: *const c_char, len: c_int) -> c_int;

	pub fn nk_str_insert_text_char(str: *mut NkStr, pos: c_int, label: *const c_char, len: c_int) -> c_int;
	pub fn nk_str_insert_str_char(str: *mut NkStr, pos: c_int, c: *const c_char) -> c_int;
	pub fn nk_str_insert_text_utf8(str: *mut NkStr, pos: c_int, label: *const c_char, len: c_int) -> c_int;
	pub fn nk_str_insert_str_utf8(str: *mut NkStr, pos: c_int, c: *const c_char) -> c_int;
	pub fn nk_str_insert_text_runes(str: *mut NkStr, pos: c_int, r: *const NkRune, len: c_int) -> c_int;
	pub fn nk_str_insert_str_runes(str: *mut NkStr, pos: c_int, r: *const NkRune) -> c_int;

	pub fn nk_str_remove_chars(str: *mut NkStr, len: c_int);
	pub fn nk_str_remove_runes(str: *mut NkStr, len: c_int);
	pub fn nk_str_delete_chars(str: *mut NkStr, pos: c_int, len: c_int);
	pub fn nk_str_delete_runes(str: *mut NkStr, pos: c_int, len: c_int);

	pub fn nk_str_at_char(str: *mut NkStr, pos: c_int) -> *mut c_char;
	pub fn nk_str_at_rune(str: *mut NkStr, pos: c_int, unicode: *mut NkRune, len: *mut c_int) -> *mut c_char;
	pub fn nk_str_rune_at(str: *const NkStr, pos: c_int) -> NkRune;
	pub fn nk_str_at_char_const(str: *const NkStr, pos: c_int) -> *const c_char;
	pub fn nk_str_at_const(str: *const NkStr, pos: c_int, unicode: *mut NkRune, len: *mut c_int) -> *const c_char;

	pub fn nk_str_get(str: *mut NkStr) -> *mut c_char;
	pub fn nk_str_get_const(str: *const NkStr) -> *const c_char;
	pub fn nk_str_len(str: *mut NkStr) -> c_int;
	pub fn nk_str_len_char(str: *mut NkStr) -> c_int;
}

/*===============================================================
 *
 *                      TEXT EDITOR
 *
 * ===============================================================*/
/* Editing text in this library is handled by either `nk_edit_string` or
 * `nk_edit_buffer`. But like almost everything in this library there are multiple
 * ways of doing it and a balance between control and ease of use with memory
 * as well as functionality controlled by flags.
 *
 * This library generally allows three different levels of memory control:
 * First of is the most basic way of just providing a simple c_char array with
 * string length. This method is probably the easiest way of handling simple
 * user text input. Main upside is complete control over memory while the biggest
 * downside in comparsion with the other two approaches is missing undo/redo.
 *
 * For UIs that require undo/redo the second way was created. It is based on
 * a fixed size NkTextEdit struct, which has an internal undo/redo stack.
 * This is mainly useful if you want something more like a text editor but don't want
 * to have a dynamically growing buffer.
 *
 * The final way is using a dynamically growing NkTextEdit struct, which
 * has both a default version if you don't care where memory comes from and an
 * allocator version if you do. While the text editor is quite powerful for its
 * complexity I would not recommend editing gigabytes of data with it.
 * It is rather designed for uses cases which make sense for a GUI library not for
 * an full blown text editor.
 */

pub const NK_TEXTEDIT_UNDOSTATECOUNT: size_t = 99;
pub const NK_TEXTEDIT_UNDOCHARCOUNT: size_t = 999;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkClipboard {
    userdata: NkHandle, 
    paste: NkFilterF,
    copy: NkCopyF,
}

impl Default for NkClipboard {
	fn default() -> NkClipboard {
		NkClipboard {
		    userdata: NkHandle::default(), 
		    paste: None,
		    copy: None,
	    }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkTextUndoRecord {
   where1: c_int,
   insert_length: c_short,
   delete_length: c_short,
   c_char_storage: c_short,
}

impl Default for NkTextUndoRecord {
	fn default() -> NkTextUndoRecord {
		NkTextUndoRecord {
		   where1: 0,
		   insert_length: 0,
		   delete_length: 0,
		   c_char_storage: 0,
		}
	}
}

#[repr(C)]
pub struct NkTextUndoState {
   undo_rec: [NkTextUndoRecord; NK_TEXTEDIT_UNDOSTATECOUNT],
   undo_char: [NkRune; NK_TEXTEDIT_UNDOCHARCOUNT],
   undo_point: c_short,
   redo_point: c_short,
   undo_char_point: c_short,
   redo_char_point: c_short,
}

impl Default for NkTextUndoState {
	fn default() -> NkTextUndoState {
		NkTextUndoState {
		   undo_rec: [NkTextUndoRecord::default(); NK_TEXTEDIT_UNDOSTATECOUNT],
		   undo_char: [0; NK_TEXTEDIT_UNDOCHARCOUNT],
		   undo_point: 0,
		   redo_point: 0,
		   undo_char_point: 0,
		   redo_char_point: 0,
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkTextEdit_type {
    NK_TEXT_EDIT_SINGLE_LINE,
    NK_TEXT_EDIT_MULTI_LINE
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkTextEdit_mode {
    NK_TEXT_EDIT_MODE_VIEW,
    NK_TEXT_EDIT_MODE_INSERT,
    NK_TEXT_EDIT_MODE_REPLACE
}

#[repr(C)]
pub struct NkTextEdit {
    clip: NkClipboard,
    string: NkStr,
    filter: NkFilter,
    scrollbar: NkVec2,

    cursor: c_int, 
    select_start: c_int,
    select_end: c_int,
    mode: c_uchar,
    cursor_at_end_of_line: c_uchar,
    initialized: c_uchar,
    has_preferred_x: c_uchar, 
    single_line: c_uchar,
    active: c_uchar,
    padding1: c_uchar,
    preferred_x: c_float,
    undo: NkTextUndoState 
}

impl Default for NkTextEdit {
	fn default() -> NkTextEdit {
		NkTextEdit {
		    clip: NkClipboard::default(),
		    string: NkStr::default(),
		    filter: None,
		    scrollbar: NkVec2::default(),
		
		    cursor: 0, 
		    select_start: 0,
		    select_end: 0,
		    mode: 0,
		    cursor_at_end_of_line: 0,
		    initialized: 0,
		    has_preferred_x: 0, 
		    single_line: 0,
		    active: 0,
		    padding1: 0,
		    preferred_x: 0.0,
		    undo: NkTextUndoState::default(), 
		}
	}
}

extern "C" {
	/* filter function */
	pub fn nk_Filter_default(editor: *const NkTextEdit, unicode: NkRune ) -> c_int;
	pub fn nk_Filter_ascii(editor: *const NkTextEdit, unicode: NkRune ) -> c_int;
	pub fn nk_Filter_float(editor: *const NkTextEdit, unicode: NkRune ) -> c_int;
	pub fn nk_Filter_decimal(editor: *const NkTextEdit, unicode: NkRune ) -> c_int;
	pub fn nk_Filter_hex(editor: *const NkTextEdit, unicode: NkRune ) -> c_int;
	pub fn nk_Filter_oct(editor: *const NkTextEdit, unicode: NkRune ) -> c_int;
	pub fn nk_Filter_binary(editor: *const NkTextEdit, unicode: NkRune ) -> c_int;

/* text editor */
//#ifdef NK_INCLUDE_DEFAULT_ALLOCATOR
	//pub fn nk_textedit_init_default(editor: *mut NkTextEdit);
//#endif
	pub fn nk_textedit_init(editor: *mut NkTextEdit, allocator: *mut NkAllocator, size: NkSize);
	pub fn nk_textedit_init_fixed(editor: *mut NkTextEdit, memory: *mut c_void, size: NkSize);
	pub fn nk_textedit_free(editor: *mut NkTextEdit);
	pub fn nk_textedit_text(editor: *mut NkTextEdit, label: *const c_char, total_len: c_int);
	pub fn nk_textedit_delete(editor: *mut NkTextEdit, where1: c_int, len: c_int);
	pub fn nk_textedit_delete_selection(editor: *mut NkTextEdit);
	pub fn nk_textedit_select_all(editor: *mut NkTextEdit);
	pub fn nk_textedit_cut(editor: *mut NkTextEdit) -> c_int;
	pub fn nk_textedit_paste(editor: *mut NkTextEdit, txt: *const c_char, len: c_int) -> c_int;
	pub fn nk_textedit_undo(editor: *mut NkTextEdit);
	pub fn nk_textedit_redo(editor: *mut NkTextEdit);
}

/* ===============================================================
 *
 *                          FONT
 *
 * ===============================================================*/
/*  Font handling in this library was designed to be quite customizable and lets
    you decide what you want to use and what you want to provide. In this sense
    there are four different degrees between control and ease of use and two
    different drawing APIs to provide for.

    So first of the easiest way to do font handling is by just providing a
    `NkUserFont` struct which only requires the height in pixel of the used
    font and a callback to calculate the width of a string. This way of handling
    fonts is best fitted for using the normal draw shape command API were you
    do all the text drawing yourself and the library does not require any kind
    of deeper knowledge about which font handling mechanism you use.

    While the first approach works fine if you don't want to use the optional
    vertex buffer output it is not enough if you do. To get font handling working
    for these cases you have to provide two additional parameters inside the
    `NkUserFont`. First a texture atlas handle used to draw text as subimages
    of a bigger font atlas texture and a callback to query a c_character's glyph
    information (offset, size, ...). So it is still possible to provide your own
    font and use the vertex buffer output.

    The final approach if you do not have a font handling functionality or don't
    want to use it in this library is by using the optional font baker. This API
    is divided into a high- and low-level API with different priorities between
    ease of use and control. Both API's can be used to create a font and 
    font atlas texture and can even be used with or without the vertex buffer
    output. So it still uses the `NkUserFont` struct and the two different
    approaches previously stated still work.
    Now to the difference between the low level API and the high level API. The low
    level API provides a lot of control over the baking process of the font and
    provides total control over memory. It consists of a number of functions that
    need to be called from begin to end and each step requires some additional
    configuration, so it is a lot more complex than the high-level API.
    If you don't want to do all the work required for using the low-level API
    you can use the font atlas API. It provides the same functionality as the
    low-level API but takes away some configuration and all of memory control and
    in term provides a easier to use API.
*/
pub type NkTextWidthF = Option<extern "C" fn(hnd: NkHandle, h: c_float, label: *const c_char, len: c_int) -> c_float>;
pub type NkQueryFontGlyphF = Option<extern "C" fn(hnd: NkHandle, font_height: c_float, glyph: *mut NkUserFontGlyph, codepoint: NkRune, next_codepoint: NkRune)>; 

//#ifdef NK_INCLUDE_VERTEX_BUFFER_OUTPUT
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkUserFontGlyph {
    uv:  [NkVec2; 2],
    /* texture coordinates */
    offset: NkVec2,
    /* offset between top left and glyph */
    width: c_float, 
    height: c_float,
    /* size of the glyph  */
    xadvance: c_float,
    /* offset to the next glyph */
}
//#endif

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkUserFont {
    userdata: NkHandle,
    /* user provided font handle */
    height: c_float,
    /* max height of the font */
    width: NkTextWidthF,
    /* font string width in pixel callback */
//#ifdef NK_INCLUDE_VERTEX_BUFFER_OUTPUT
    query: NkQueryFontGlyphF,
    /* font glyph callback to query drawing info */
    texture: NkHandle,
    /* texture handle to the used font atlas or texture */
//#endif
}

impl Default for NkUserFont {
	fn default() -> NkUserFont {
		NkUserFont {
		    userdata: NkHandle::default(),
		    /* user provided font handle */
		    height: 0.0,
		    /* max height of the font */
		    width: None,
		    /* font string width in pixel callback */
		//#ifdef NK_INCLUDE_VERTEX_BUFFER_OUTPUT
		    query: None,
		    /* font glyph callback to query drawing info */
		    texture: NkHandle::default(),
		    /* texture handle to the used font atlas or texture */
		//#endif
		}
	}
}

//#ifdef NK_INCLUDE_FONT_BAKING
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkFontCoordType {
    NK_COORD_UV,
    /* texture coordinates inside font glyphs are clamped between 0-1 */
    NK_COORD_PIXEL
    /* texture coordinates inside font glyphs are in absolute pixel */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_baked_font {
    height: c_float,
    /* height of the font  */
    ascent: c_float, 
    descent: c_float,
    /* font glyphs ascent and descent  */
    glyph_offset: NkRune,
    /* glyph array offset inside the font glyph baking output array  */
    glyph_count: NkRune,
    /* number of glyphs of this font inside the glyph baking array output */
    ranges: *const NkRune,
    /* font codepoint ranges as pairs of (from/to) and 0 as last element */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkFontConfig {
    next: *mut NkFontConfig,
    /* NOTE: only used internally */
    ttf_blob: *mut c_void,
    /* pointer to loaded TTF file memory block.
     * NOTE: not needed for NkFontAtlas_add_from_memory and NkFontAtlas_add_from_file. */
    ttf_size: NkSize,
    /* size of the loaded TTF file memory block
     * NOTE: not needed for NkFontAtlas_add_from_memory and NkFontAtlas_add_from_file. */

    ttf_data_owned_by_atlas: c_uchar,
    /* used inside font atlas: default to: 0*/
    merge_mode: c_uchar, 
    /* merges this font into the last font */
    pixel_snap: c_uchar,
    /* align very c_character to pixel boundary (if true set oversample (1,1)) */
    pub oversample_v: c_uchar, 
    pub oversample_h: c_uchar,
    /* rasterize at hight quality for sub-pixel position */
    padding: [c_uchar; 3],

    size: c_float,
    /* baked pixel height of the font */
    coord_type: NkFontCoordType,
    /* texture coordinate format with either pixel or UV coordinates */
    spacing: NkVec2,
    /* extra pixel spacing between glyphs  */
    range: *const NkRune,
    /* list of unicode ranges (2 values per range, zero terminated) */
    font: *mut nk_baked_font,
    /* font to setup in the baking process: NOTE: not needed for font atlas */
    fallback_glyph: NkRune, 
    /* fallback glyph to use if a given rune is not found */
}

impl NkFontConfig {
	pub fn new(font_height: f32) -> NkFontConfig {
		unsafe{
			nk_font_config(font_height)
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkFontGlyph {
    codepoint: NkRune,
    xadvance: c_float,
    x0: c_float, y0: c_float, x1: c_float, y1: c_float, w: c_float, h: c_float,
    u0: c_float, v0: c_float, u1: c_float, v1: c_float,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkFont {
    next: *mut NkFont,
    handle: NkUserFont,
    info: nk_baked_font,
    scale: c_float,
    glyphs: *mut NkFontGlyph,
    fallback: *const NkFontGlyph,
    fallback_codepoint: NkRune,
    texture: NkHandle, 
    config: *mut NkFontConfig,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkFontAtlas_format {
    NK_FONT_ATLAS_ALPHA8,
    NK_FONT_ATLAS_RGBA32
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkFontAtlas {
    pixel: *mut c_void,
    tex_width: c_int, 
    tex_height: c_int,
    permanent: NkAllocator,
    temporary: NkAllocator,
    custom: NkRecti,
    cursors: [NkCursor; NK_CURSOR_COUNT],

    glyph_count: c_int,
    glyphs: *mut NkFontGlyph,
    default_font: *mut NkFont,
    fonts: *mut NkFont,
    config: *mut NkFontConfig,
    font_num: c_int,
}

impl Default for NkFontAtlas {
	fn default() -> NkFontAtlas {
		NkFontAtlas {
		    pixel: ::std::ptr::null_mut(),
		    tex_width: 0, 
		    tex_height: 0,
		    permanent: NkAllocator::default(),
		    temporary: NkAllocator::default(),
		    custom: NkRecti::default(),
		    cursors: [NkCursor::default(); NK_CURSOR_COUNT],
		
		    glyph_count: 0,
		    glyphs: ::std::ptr::null_mut(),
		    default_font: ::std::ptr::null_mut(),
		    fonts: ::std::ptr::null_mut(),
		    config: ::std::ptr::null_mut(),
		    font_num: 0,
		}
	}
}

extern "C" {
	/* some language glyph codepoint ranges */
	pub fn nk_font_default_glyph_ranges() -> *const NkRune;
	pub fn nk_font_chinese_glyph_ranges() -> *const NkRune;
	pub fn nk_font_cyrillic_glyph_ranges() -> *const NkRune;
	pub fn nk_font_korean_glyph_ranges() -> *const NkRune;
}

#[link(name = "nuklear", kind = "static")]
extern "C" {
	//#ifdef NK_INCLUDE_DEFAULT_ALLOCATOR
	//pub fn nk_font_atlas_init_default(atlas: *mut NkFontAtlas);
//#endif
	pub fn nk_font_atlas_init(atlas: *mut NkFontAtlas, allocator: *mut NkAllocator);
	pub fn nk_font_atlas_init_custom(atlas: *mut NkFontAtlas, persistent: *mut NkAllocator, transient: *mut NkAllocator);
	pub fn nk_font_atlas_begin(atlas: *mut NkFontAtlas);
	pub fn nk_font_config(pixel_height: c_float) -> NkFontConfig;
	pub fn nk_font_atlas_add(atlas: *mut NkFontAtlas, cfg: *const NkFontConfig) -> *mut NkFont;
/*#ifdef NK_INCLUDE_DEFAULT_FONT
	pub fn struct NkFont* NkFontAtlas_add_default(atlas: *mut NkFontAtlas, float height, const struct NkFontConfig*);
#endif*/
	pub fn nk_font_atlas_add_from_memory(atlas: *mut NkFontAtlas, memory: *mut c_void, size: NkSize, height: c_float, config: *const NkFontConfig) -> *mut NkFont;
/*#ifdef NK_INCLUDE_STANDARD_IO
	pub fn struct NkFont* NkFontAtlas_add_from_file(struct NkFontAtlas *atlas, const c_char *file_path, float height, const struct NkFontConfig*);
#endif*/
	pub fn nk_font_atlas_add_compressed(atlas: *mut NkFontAtlas, memory: *mut c_void, size: NkSize, height: c_float, config: *const NkFontConfig) -> *mut NkFont;
	pub fn nk_font_atlas_add_compressed_base85(atlas: *mut NkFontAtlas, data: *const c_char, height: c_float, config: *const NkFontConfig) -> *mut NkFont;
	pub fn nk_font_atlas_bake(atlas: *mut NkFontAtlas, width: *mut c_int, height: *mut c_int, fmt: NkFontAtlas_format) -> *const c_void;
	pub fn nk_font_atlas_end(atlas: *mut NkFontAtlas, tex: NkHandle, nul: *mut NkDrawNullTexture);
	pub fn nk_font_atlas_clear(atlas: *mut NkFontAtlas);
	pub fn nk_font_find_glyph(font: *mut NkFont, unicode: NkRune) -> *const NkFontGlyph;
}

//#endif

/* ===============================================================
 *
 *                          DRAWING
 *
 * ===============================================================*/
/*  This library was designed to be render backend agnostic so it does
    not draw anything to screen. Instead all drawn shapes, widgets
    are made of, are buffered into memory and make up a command queue.
    Each frame therefore fills the command buffer with draw commands
    that then need to be executed by the user and his own render backend.
    After that the command buffer needs to be cleared and a new frame can be
    started. It is probably important to note that the command buffer is the main
    drawing API and the optional vertex buffer API only takes this format and
    converts it into a hardware accessible format.

    Draw commands are divided into filled shapes and shape outlines but only
    filled shapes as well as line, curves and scissor are required to be provided.
    All other shape drawing commands can be used but are not required. This was
    done to allow the maximum number of render backends to be able to use this
    library without you having to do additional work.
*/
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum nk_command_type {
    NK_COMMAND_NOP,
    NK_COMMAND_SCISSOR,
    NK_COMMAND_LINE,
    NK_COMMAND_CURVE,
    NK_COMMAND_RECT,
    NK_COMMAND_RECT_FILLED,
    NK_COMMAND_RECT_MULTI_COLOR,
    NK_COMMAND_CIRCLE,
    NK_COMMAND_CIRCLE_FILLED,
    NK_COMMAND_ARC,
    NK_COMMAND_ARC_FILLED,
    NK_COMMAND_TRIANGLE,
    NK_COMMAND_TRIANGLE_FILLED,
    NK_COMMAND_POLYGON,
    NK_COMMAND_POLYGON_FILLED,
    NK_COMMAND_POLYLINE,
    NK_COMMAND_TEXT,
    NK_COMMAND_IMAGE
}

/* command base and header of every command inside the buffer */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command {
    ty: nk_command_type,
    next: NkSize,
//#ifdef NK_INCLUDE_COMMAND_USERDATA
    userdata: NkHandle,
//#endif
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_scissor {
    header: nk_command,
    x: c_short, y: c_short,
    w: c_ushort, h: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_line {
    header: nk_command,
    line_thickness: c_ushort,
    begin: NkVec2i,
    end: NkVec2i,
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_curve {
    header: nk_command,
    line_thickness: c_ushort,
    begin: NkVec2i,
    end: NkVec2i,
    ctrl: [NkVec2i; 2],
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_rect {
    header: nk_command,
    rounding: c_ushort,
    line_thickness: c_ushort,
    x: c_short, y: c_short,
    w: c_ushort, h: c_ushort,
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_rect_filled {
    header: nk_command,
    rounding: c_ushort,
    x: c_short, y: c_short,
    w: c_ushort, h: c_ushort,
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_rect_multi_color {
    header: nk_command,
    x: c_short, y: c_short,
    w: c_ushort, h: c_ushort,
    left: NkColor,
    top: NkColor,
    bottom: NkColor,
    right: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_triangle {
    header: nk_command,
    line_thickness: c_ushort,
    a: NkVec2i,
    b: NkVec2i,
    c: NkVec2i,
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_triangle_filled {
    header: nk_command,
    a: NkVec2i,
    b: NkVec2i,
    c: NkVec2i,
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_circle {
    header: nk_command,
    x: c_short, y: c_short,
    line_thickness: c_ushort,
    w: c_ushort, h: c_ushort,
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_circle_filled {
    header: nk_command,
    x: c_short, y: c_short,
    w: c_ushort, h: c_ushort,
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_arc {
    header: nk_command,
    cx: c_short, cy: c_short,
    r: c_ushort,
    line_thickness: c_ushort,
    a: [c_float; 2],
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_arc_filled {
    header: nk_command,
    cx: c_short, cy: c_short,
    r: c_ushort,
    a: [c_float; 2],
    color: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_polygon {
    header: nk_command,
    color: NkColor,
    line_thickness: c_ushort,
    point_count: c_ushort,
    points: [NkVec2i; 1],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_polygon_filled {
    header: nk_command,
    color: NkColor,
    point_count: c_ushort,
    points: [NkVec2i; 1],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_polyline {
    header: nk_command,
    color: NkColor,
    line_thickness: c_ushort,
    point_count: c_ushort,
    points: [NkVec2i; 1]
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_image {
    header: nk_command,
    x: c_short, y: c_short,
    w: c_ushort, h: c_ushort,
    img: NkImage,
    col: NkColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_command_text {
    header: nk_command,
    font: *const NkUserFont,
    background: NkColor,
    foreground: NkColor,
    x: c_short, y: c_short,
    w: c_ushort, h: c_ushort,
    height: c_float,
    length: c_int,
    string: [c_char; 1],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum nk_command_clipping {
    NK_CLIPPING_OFF = NkFalse,
    NK_CLIPPING_ON = NkTrue
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkCommandBuffer {
    base: *mut NkBuffer,
    clip: NkRect,
    use_clipping: c_int, 
    userdata: NkHandle,
    begin: NkSize, end: NkSize, last: NkSize,
}

impl Default for NkCommandBuffer {
	fn default() -> NkCommandBuffer {
		NkCommandBuffer {
		    base: ::std::ptr::null_mut(),
		    clip: NkRect::default(),
		    use_clipping: 0, 
		    userdata: NkHandle::default(),
		    begin: 0, end: 0, last: 0,
		}
	}
}

extern "C" {
	/* shape outlines */
	pub fn nk_stroke_line(b: *mut NkCommandBuffer, x0: c_float, y0: c_float, x1: c_float, y1: c_float, line_thickness: c_float, color: NkColor);
	pub fn nk_stroke_curve(b: *mut NkCommandBuffer, x0: c_float, y0: c_float, x1: c_float, y1: c_float, x2: c_float, y2: c_float, x3: c_float, y3: c_float, line_thickness: c_float, color: NkColor);
	pub fn nk_stroke_rect(b: *mut NkCommandBuffer, rect: NkRect, rounding: c_float, line_thickness : c_float, color: NkColor);
	pub fn nk_stroke_circle(b: *mut NkCommandBuffer, rect: NkRect, line_thickness: c_float, color: NkColor);
	pub fn nk_stroke_arc(b: *mut NkCommandBuffer, cx: c_float, cy: c_float, radius: c_float, a_min: c_float, a_max: c_float, line_thickness: c_float, color: NkColor);
	pub fn nk_stroke_triangle(b: *mut NkCommandBuffer, x0: c_float, y0: c_float, x1: c_float, y1: c_float, x2: c_float, y2: c_float, line_thichness: c_float, color: NkColor);
	pub fn nk_stroke_polyline(b: *mut NkCommandBuffer, points: *mut c_float, point_count: c_int, line_thickness: c_float, color: NkColor);
	pub fn nk_stroke_polygon(b: *mut NkCommandBuffer, points: *mut c_float, point_count: c_int, line_thickness: c_float, color: NkColor);
}

extern "C" {
	/* filled shades */
	pub fn nk_fill_rect(b: *mut NkCommandBuffer, rect: NkRect, rounding: c_float , color: NkColor);
	pub fn nk_fill_rect_multi_color(b: *mut NkCommandBuffer, rect: NkRect, left: NkColor, top: NkColor, right: NkColor, bottom: NkColor);
	pub fn nk_fill_circle(b: *mut NkCommandBuffer, rect: NkRect, color: NkColor);
	pub fn nk_fill_arc(b: *mut NkCommandBuffer, cx: c_float , cy: c_float, radius: c_float, a_min: c_float, a_max: c_float, color: NkColor);
	pub fn nk_fill_triangle(b: *mut NkCommandBuffer, x0: c_float, y0: c_float, x1: c_float, y1: c_float, x2: c_float, y2: c_float, color: NkColor);
	pub fn nk_fill_polygon(b: *mut NkCommandBuffer, p: *mut c_float, point_count: c_int , color: NkColor);
}

extern "C" {
	/* misc */
	pub fn nk_push_scissor(b: *mut NkCommandBuffer, rect: NkRect);
	pub fn nk_draw_image(b: *mut NkCommandBuffer, rect: NkRect, img: *const NkImage, color: NkColor);
	pub fn nk_draw_text(b: *mut NkCommandBuffer, rect: NkRect, text: *const c_char, len: c_int, font: *const NkUserFont, color: NkColor, color: NkColor);
	pub fn nk__next(ctx: *mut NkContext, next: *const nk_command) -> *const nk_command;
	pub fn nk__begin(ctx: *mut NkContext) -> *const nk_command;
}

/* ===============================================================
 *
 *                          INPUT
 *
 * ===============================================================*/
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkMouseButton {
    down: c_int,
    clicked: c_uint,
    clicked_pos: NkVec2 ,
}

impl Default for NkMouseButton {
	fn default() -> NkMouseButton {
		NkMouseButton {down: 0, clicked: 0, clicked_pos: NkVec2::default()}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkMouse {
    buttons: [NkMouseButton; 3], //NK_BUTTON_MAX],
    pos: NkVec2,
    prev: NkVec2,
    delta: NkVec2,
    scroll_delta: c_float,
    grab: c_uchar,
    grabbed: c_uchar,
    ungrab: c_uchar,
}

impl Default for NkMouse {
	fn default() -> NkMouse {
		NkMouse {
		    buttons: [NkMouseButton::default(); NK_BUTTON_MAX as usize],
		    pos: NkVec2::default(),
		    prev: NkVec2::default(),
		    delta: NkVec2::default(),
		    scroll_delta: 0.0,
		    grab: 0,
		    grabbed: 0,
		    ungrab: 0,
	    }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkKey {
    down: c_int,
    clicked: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkKeyboard {
    keys: [NkKey; 29], //NK_KEY_MAX],
    text: [c_char; NK_INPUT_MAX],
    text_len: c_int,
}

impl Default for NkKeyboard {
	fn default() -> NkKeyboard {
		NkKeyboard {
			keys: [NkKey{down: 0, clicked: 0}; NK_KEY_MAX as usize],
		    text: [0; NK_INPUT_MAX as usize],
		    text_len: 0,
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkInput {
    keyboard: NkKeyboard,
    mouse: NkMouse,
}

impl Default for NkInput {
	fn default() -> NkInput {
		NkInput {
			keyboard: NkKeyboard::default(),
		    mouse: NkMouse::default(),
		}
	}
}

extern "C" {
	pub fn nk_input_has_mouse_click(i: *const NkInput, button: NkButtons) -> c_int;
	pub fn nk_input_has_mouse_click_in_rect(i: *const NkInput, button: NkButtons, rect: NkRect) -> c_int;
	pub fn nk_input_has_mouse_click_down_in_rect(i: *const NkInput, button: NkButtons, rect: NkRect, down: c_int) -> c_int;
	pub fn nk_input_is_mouse_click_in_rect(i: *const NkInput, button: NkButtons, rect: NkRect) -> c_int;
	pub fn nk_input_is_mouse_click_down_in_rect(i: *const NkInput, id: NkButtons, rect: NkRect, down: c_int) -> c_int;
	pub fn nk_input_any_mouse_click_in_rect(i: *const NkInput, rect: NkRect) -> c_int;
	pub fn nk_input_is_mouse_prev_hovering_rect(i: *const NkInput, rect: NkRect) -> c_int;
	pub fn nk_input_is_mouse_hovering_rect(i: *const NkInput, rect: NkRect) -> c_int;
	pub fn nk_input_mouse_clicked(i: *const NkInput, button: NkButtons, rect: NkRect) -> c_int;
	pub fn nk_input_is_mouse_down(i: *const NkInput, button: NkButtons) -> c_int;
	pub fn nk_input_is_mouse_pressed(i: *const NkInput, button: NkButtons) -> c_int;
	pub fn nk_input_is_mouse_released(i: *const NkInput, button: NkButtons) -> c_int;
	pub fn nk_input_is_key_pressed(i: *const NkInput, key: NkKeys) -> c_int;
	pub fn nk_input_is_key_released(i: *const NkInput, key: NkKeys) -> c_int;
	pub fn nk_input_is_key_down(i: *const NkInput, key: NkKeys) -> c_int;
}


/* ===============================================================
 *
 *                          DRAW LIST
 *
 * ===============================================================*/
//#ifdef NK_INCLUDE_VERTEX_BUFFER_OUTPUT
/*  The optional vertex buffer draw list provides a 2D drawing context
    with antialiasing functionality which takes basic filled or outlined shapes
    or a path and outputs vertexes, elements and draw commands.
    The actual draw list API is not required to be used directly while using this
    library since converting the default library draw command output is done by
    just calling `nk_convert` but I decided to still make this library accessible
    since it can be useful.

    The draw list is based on a path buffering and polygon and polyline
    rendering API which allows a lot of ways to draw 2D content to screen.
    In fact it is probably more powerful than needed but allows even more crazy
    things than this library provides by default.
*/
pub type NkDrawIndex = c_ushort;
pub type NkDrawVertexColor = NkUint;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkDrawList_stroke {
    NK_STROKE_OPEN = NkFalse,
    /* build up path has no connection back to the beginning */
    NK_STROKE_CLOSED = NkTrue
    /* build up path has a connection back to the beginning */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_draw_vertex {
    position: NkVec2, 
    uv: NkVec2,
    col: NkDrawVertexColor,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_draw_command {
    elem_count: c_uint,
    /* number of elements in the current draw batch */
    clip_rect: NkRect,
    /* current screen clipping rectangle */
    texture: NkHandle,
    /* current texture to set */
//#ifdef NK_INCLUDE_COMMAND_USERDATA
    userdata: NkHandle,
//#endif
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkDrawList {
    global_alpha: c_float,
    shape_AA: NkAntiAliasing,
    line_AA: NkAntiAliasing,
    null: NkDrawNullTexture,
    clip_rect: NkRect,
    buffer: *mut NkBuffer,
    vertices: *mut NkBuffer,
    elements: *mut NkBuffer,
    element_count: c_uint,
    vertex_count: c_uint,
    cmd_offset: NkSize,
    cmd_count: c_uint,
    path_count: c_uint,
    path_offset: c_uint,
    circle_vtx: [NkVec2; 12],
//#ifdef NK_INCLUDE_COMMAND_USERDATA
    userdata: NkHandle,
//#endif
}

impl Default for NkDrawList {
	fn default() -> NkDrawList {
		NkDrawList {
		    global_alpha: 0.0,
		    shape_AA: NkAntiAliasing::NK_ANTI_ALIASING_OFF,
		    line_AA: NkAntiAliasing::NK_ANTI_ALIASING_OFF,
		    null: NkDrawNullTexture::default(),
		    clip_rect: NkRect::default(),
		    buffer: ::std::ptr::null_mut(),
		    vertices: ::std::ptr::null_mut(),
		    elements: ::std::ptr::null_mut(),
		    element_count: 0,
		    vertex_count: 0,
		    cmd_offset: 0,
		    cmd_count: 0,
		    path_count: 0,
		    path_offset: 0,
		    circle_vtx: [NkVec2::default(); 12],
		//#ifdef NK_INCLUDE_COMMAND_USERDATA
		    userdata: NkHandle::default(),
		//#endif
		}
	}
}

extern "C" {
	/* draw list */
	pub fn nk_DrawList_init(list: *mut NkDrawList);
	pub fn nk_DrawList_setup(list: *mut NkDrawList, global_alpha: c_float, shape_aa: NkAntiAliasing, line_aa: NkAntiAliasing, null: NkDrawNullTexture, cmds: *mut NkBuffer, vert: *mut NkBuffer, elem: *mut NkBuffer);
	pub fn nk_DrawList_clear(list: *mut NkDrawList);

	/* drawing */
//#define NkDrawList_foreach(cmd, can, b) for((cmd)=nk__draw_list_begin(can, b); (cmd)!=0; (cmd)=nk__draw_list_next(cmd, b, can))
	pub fn nk__draw_list_begin(list: *const NkDrawList, buffer: *const NkBuffer) -> *const nk_draw_command;
	pub fn nk__draw_list_next(cmds: *const nk_draw_command, buffer: *const NkBuffer, draw_list: *const NkDrawList) -> *const nk_draw_command;
	pub fn nk__draw_begin(ctx: *const NkContext, buffer: *const NkBuffer) -> *const nk_draw_command;
	pub fn nk__draw_next(cmds: *const nk_draw_command, buffer: *const NkBuffer, ctx: *const NkContext) -> *const nk_draw_command;

	/* path */
	pub fn nk_DrawList_path_clear(list: *mut NkDrawList);
	pub fn nk_DrawList_path_line_to(list: *mut NkDrawList, pos: NkVec2);
	pub fn nk_DrawList_path_arc_to_fast(list: *mut NkDrawList, center: NkVec2, radius: c_float, a_min: c_int, a_max: c_int);
	pub fn nk_DrawList_path_arc_to(list: *mut NkDrawList, center: NkVec2, radius: c_float, a_min: c_float, a_max: c_float, segments: c_uint);
	pub fn nk_DrawList_path_rect_to(list: *mut NkDrawList, a: NkVec2, b: NkVec2, rounding: c_float);
	pub fn nk_DrawList_path_curve_to(list: *mut NkDrawList, p2: NkVec2, p3: NkVec2, p4: NkVec2, num_segments: c_uint);
	pub fn nk_DrawList_path_fill(list: *mut NkDrawList, color: NkColor);
	pub fn nk_DrawList_path_stroke(list: *mut NkDrawList, color: NkColor, closed: NkDrawList_stroke, thickness: c_float);

	/* stroke */
	pub fn nk_DrawList_stroke_line(list: *mut NkDrawList, a: NkVec2, b: NkVec2, color: NkColor, thickness: c_float);
	pub fn nk_DrawList_stroke_rect(list: *mut NkDrawList, rect: NkRect, color: NkColor, rounding: c_float, thickness: c_float);
	pub fn nk_DrawList_stroke_triangle(list: *mut NkDrawList, a: NkVec2, b: NkVec2, c: NkVec2, color: NkColor, thickness: c_float);
	pub fn nk_DrawList_stroke_circle(list: *mut NkDrawList, center: NkVec2, radius: c_float, color: NkColor, segs: c_uint, thickness: c_float);
	pub fn nk_DrawList_stroke_curve(list: *mut NkDrawList, p0: NkVec2, cp0: NkVec2, cp1: NkVec2, p1: NkVec2, color: NkColor, segments: c_uint, thickness: c_float);
	pub fn nk_DrawList_stroke_poly_line(list: *mut NkDrawList, pnts: *const NkVec2, cnt: c_uint, color: NkColor, stroke: NkDrawList_stroke, thickness: c_float , aa: NkAntiAliasing);

	/* fill */
	pub fn nk_DrawList_fill_rect(list: *mut NkDrawList, rect: NkRect, color: NkColor, rounding: c_float);
	pub fn nk_DrawList_fill_rect_multi_color(list: *mut NkDrawList, rect: NkRect, left: NkColor, top: NkColor, right: NkColor, bottom: NkColor);
	pub fn nk_DrawList_fill_triangle(list: *mut NkDrawList, a: NkVec2, b: NkVec2, c: NkVec2, color: NkColor);
	pub fn nk_DrawList_fill_circle(list: *mut NkDrawList, center: NkVec2, radius: c_float, color: NkColor, segs: c_uint);
	pub fn nk_DrawList_fill_poly_convex(list: *mut NkDrawList, points: *const NkVec2, count: c_uint, color: NkColor, aa: NkAntiAliasing);

	/* misc */
	pub fn nk_DrawList_add_image(list: *mut NkDrawList, texture: NkImage, rect: NkRect, color: NkColor);
	pub fn nk_DrawList_add_text(list: *mut NkDrawList, font: *const NkUserFont, rect: NkRect, text: *const c_char, len: c_int, font_height: c_float, color: NkColor);
//#ifdef NK_INCLUDE_COMMAND_USERDATA
	pub fn nk_DrawList_push_userdata(list: *mut NkDrawList, userdata: NkHandle);
}
//#endif

//#endif

/* ===============================================================
 *
 *                          GUI
 *
 * ===============================================================*/
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkStyleItemType {
    NK_STYLE_ITEM_COLOR,
    NK_STYLE_ITEM_IMAGE
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkStyleItemData {img: NkImage, color: NkColor}

impl Default for NkStyleItemData {
	fn default() -> NkStyleItemData {
		NkStyleItemData {
			img: NkImage::default(), 
			color: NkColor::default()
    	}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkStyleItem {
    ty: NkStyleItemType,
    data: NkStyleItemData,
}

impl Default for NkStyleItem {
	fn default() -> NkStyleItem {
		NkStyleItem {
    		ty: NkStyleItemType::NK_STYLE_ITEM_COLOR , 
    		data: NkStyleItemData::default(),
    	}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkStyleText {
    color: NkColor,
    padding: NkVec2,
}

impl Default for NkStyleText {
	fn default() -> NkStyleText {
		NkStyleText {
		    color: NkColor::default(),
		    padding: NkVec2::default(),
		}
	}
}

#[repr(C)]
pub struct NkStyleButton {
    /* background */
    normal: NkStyleItem,
    hover: NkStyleItem,
    active: NkStyleItem,
    border_color: NkColor,

    /* text */
    text_background: NkColor,
    text_normal: NkColor,
    text_hover: NkColor,
    text_active: NkColor,
    text_alignment: NkFlags,

    /* properties */
    border: c_float,
    rounding: c_float,
    padding: NkVec2,
    image_padding: NkVec2,
    touch_padding: NkVec2,

    /* optional user callbacks */
    userdata: NkHandle,
    draw_begin: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_begin)(b: *mut NkCommandBuffer, NkHandle userdata);
    draw_end: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_end)(b: *mut NkCommandBuffer, NkHandle userdata);
}

impl Default for NkStyleButton {
	fn default() -> NkStyleButton {
		NkStyleButton {
	    	normal: NkStyleItem::default(),
		    hover: NkStyleItem::default(),
		    active: NkStyleItem::default(),
		    border_color: NkColor::default(),
		
		    text_background: NkColor::default(),
		    text_normal: NkColor::default(),
		    text_hover: NkColor::default(),
		    text_active: NkColor::default(),
		    text_alignment: 0,
		
		    border: 0.0,
		    rounding: 0.0,
		    padding: NkVec2::default(),
		    image_padding: NkVec2::default(),
		    touch_padding: NkVec2::default(),
		
		    userdata: NkHandle::default(),
		    draw_begin: None,
		    draw_end: None
	    }
	}
}

#[repr(C)]
pub struct NkStyleToggle {
    /* background */
    normal: NkStyleItem,
    hover: NkStyleItem,
    active: NkStyleItem,
    border_color: NkColor,

    /* cursor */
    cursor_normal: NkStyleItem,
    cursor_hover: NkStyleItem,

    /* text */
    text_normal: NkColor,
    text_hover: NkColor,
    text_active: NkColor,
    text_background: NkColor,
    text_alignment: NkFlags,

    /* properties */
    padding: NkVec2,
    touch_padding: NkVec2,
    spacing: c_float,
    border: c_float,

    /* optional user callbacks */
    userdata: NkHandle,
	draw_begin: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_begin)(b: *mut NkCommandBuffer, NkHandle userdata);
    draw_end: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_end)(b: *mut NkCommandBuffer, NkHandle userdata);
}

impl Default for NkStyleToggle {
	fn default() -> NkStyleToggle {
		NkStyleToggle {
	    	normal: NkStyleItem::default(),
		    hover: NkStyleItem::default(),
		    active: NkStyleItem::default(),
		    border_color: NkColor::default(),
		    cursor_normal: NkStyleItem::default(),
		    cursor_hover: NkStyleItem::default(),
		    text_normal: NkColor::default(),
		    text_hover: NkColor::default(),
		    text_active: NkColor::default(),
		    text_background: NkColor::default(),
		    text_alignment: 0,
		    padding: NkVec2::default(),
		    touch_padding: NkVec2::default(),
		    spacing: 0.0,
		    border: 0.0,
		    userdata: NkHandle::default(),
			draw_begin: None,
		    draw_end: None,
	    }
	}
}

#[repr(C)]
pub struct NkStyleSelectable {
    /* background (inactive) */
    normal: NkStyleItem,
    hover: NkStyleItem,
    pressed: NkStyleItem,

    /* background (active) */
    normal_active: NkStyleItem,
    hover_active: NkStyleItem,
    pressed_active: NkStyleItem,

    /* text color (inactive) */
    text_normal: NkColor,
    text_hover: NkColor,
    text_pressed: NkColor,

    /* text color (active) */
    text_normal_active: NkColor,
    text_hover_active: NkColor,
    text_pressed_active: NkColor,
    text_background: NkColor,
    text_alignment: NkFlags,

    /* properties */
    rounding: c_float,
    padding: NkVec2,
    touch_padding: NkVec2,
    image_padding: NkVec2,

    /* optional user callbacks */
	userdata: NkHandle,
	draw_begin: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_begin)(b: *mut NkCommandBuffer, NkHandle userdata);
    draw_end: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_end)(b: *mut NkCommandBuffer, NkHandle userdata);
}

impl Default for NkStyleSelectable {
	fn default() -> NkStyleSelectable {
		NkStyleSelectable {
	    	/* background (inactive) */
		    normal: NkStyleItem::default(),
		    hover: NkStyleItem::default(),
		    pressed: NkStyleItem::default(),
		
		    /* background (active) */
		    normal_active: NkStyleItem::default(),
		    hover_active: NkStyleItem::default(),
		    pressed_active: NkStyleItem::default(),
		
		    /* text color (inactive) */
		    text_normal: NkColor::default(),
		    text_hover: NkColor::default(),
		    text_pressed: NkColor::default(),
		
		    /* text color (active) */
		    text_normal_active: NkColor::default(),
		    text_hover_active: NkColor::default(),
		    text_pressed_active: NkColor::default(),
		    text_background: NkColor::default(),
		    text_alignment: 0,
		
		    /* properties */
		    rounding: 0.0,
		    padding: NkVec2::default(),
		    touch_padding: NkVec2::default(),
		    image_padding: NkVec2::default(),
		
		    /* optional user callbacks */
			userdata: NkHandle::default(),
			draw_begin: None,
		    draw_end: None,
	    }
	}
}

#[repr(C)]
pub struct NkStyleSlider {
    /* background */
    normal: NkStyleItem,
    hover: NkStyleItem,
    active: NkStyleItem,
    border_color: NkColor,

    /* background bar */
    bar_normal: NkColor,
    bar_hover: NkColor,
    bar_active: NkColor,
    bar_filled: NkColor,

    /* cursor */
    cursor_normal: NkStyleItem,
    cursor_hover: NkStyleItem,
    cursor_active: NkStyleItem,

    /* properties */
    border: c_float,
    rounding: c_float,
    bar_height: c_float,
    padding: NkVec2,
    spacing: NkVec2,
    cursor_size: NkVec2,

    /* optional buttons */
    show_buttons: c_int,
    inc_button: NkStyleButton,
    dec_button: NkStyleButton,
    inc_symbol: NkSymbolType,
    dec_symbol: NkSymbolType,

    /* optional user callbacks */
	userdata: NkHandle,
	draw_begin: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_begin)(b: *mut NkCommandBuffer, NkHandle userdata);
    draw_end: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_end)(b: *mut NkCommandBuffer, NkHandle userdata);
}

impl Default for NkStyleSlider {
	fn default() -> NkStyleSlider {
		NkStyleSlider {
	    	/* background */
		    normal: NkStyleItem::default(),
		    hover: NkStyleItem::default(),
		    active: NkStyleItem::default(),
		    border_color: NkColor::default(),
		
		    /* background bar */
		    bar_normal: NkColor::default(),
		    bar_hover: NkColor::default(),
		    bar_active: NkColor::default(),
		    bar_filled: NkColor::default(),
		
		    /* cursor */
		    cursor_normal: NkStyleItem::default(),
		    cursor_hover: NkStyleItem::default(),
		    cursor_active: NkStyleItem::default(),
		
		    /* properties */
		    border: 0.0,
		    rounding: 0.0,
		    bar_height: 0.0,
		    padding: NkVec2::default(),
		    spacing: NkVec2::default(),
		    cursor_size: NkVec2::default(),
		
		    /* optional buttons */
		    show_buttons: 0,
		    inc_button: NkStyleButton::default(),
		    dec_button: NkStyleButton::default(),
		    inc_symbol: NkSymbolType::NK_SYMBOL_NONE,
		    dec_symbol: NkSymbolType::NK_SYMBOL_NONE,
		
		    /* optional user callbacks */
			userdata: NkHandle::default(),
			draw_begin: None,
		    draw_end: None
	    }
	}
}

#[repr(C)]
pub struct NkStyleProgress {
    /* background */
    normal: NkStyleItem,
    hover: NkStyleItem,
    active: NkStyleItem,
    border_color: NkColor,

    /* cursor */
    cursor_normal: NkStyleItem,
    cursor_hover: NkStyleItem,
    cursor_active: NkStyleItem,
    cursor_border_color: NkColor,

    /* properties */
    rounding: c_float,
    border: c_float,
    cursor_border: c_float,
    cursor_rounding: c_float,
    padding: NkVec2,

    /* optional user callbacks */
	userdata: NkHandle,
	draw_begin: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_begin)(b: *mut NkCommandBuffer, NkHandle userdata);
    draw_end: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_end)(b: *mut NkCommandBuffer, NkHandle userdata);
}

impl Default for NkStyleProgress {
	fn default() -> NkStyleProgress {
			NkStyleProgress {
		    	/* background */
			    normal: NkStyleItem::default(),
			    hover: NkStyleItem::default(),
			    active: NkStyleItem::default(),
			    border_color: NkColor::default(),
			
			    /* cursor */
			    cursor_normal: NkStyleItem::default(),
			    cursor_hover: NkStyleItem::default(),
			    cursor_active: NkStyleItem::default(),
			    cursor_border_color: NkColor::default(),
			
			    /* properties */
			    rounding: 0.0,
			    border: 0.0,
			    cursor_border: 0.0,
			    cursor_rounding: 0.0,
			    padding: NkVec2::default(),
			
			    /* optional user callbacks */
				userdata: NkHandle::default(),
				draw_begin: None,
			    draw_end: None,
		    }
	}
}

#[repr(C)]
pub struct NkStyleScrollbar {
    /* background */
    normal: NkStyleItem,
    hover: NkStyleItem,
    active: NkStyleItem,
    border_color: NkColor,

    /* cursor */
    cursor_normal: NkStyleItem,
    cursor_hover: NkStyleItem,
    cursor_active: NkStyleItem,
    cursor_border_color: NkColor,

    /* properties */
    border: c_float,
    rounding: c_float,
    border_cursor: c_float,
    rounding_cursor: c_float,
    padding: NkVec2,

    /* optional buttons */
    show_buttons: c_int, 
    inc_button: NkStyleButton,
    dec_button: NkStyleButton,
    inc_symbol: NkSymbolType,
    dec_symbol: NkSymbolType,

    /* optional user callbacks */
	userdata: NkHandle,
	draw_begin: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_begin)(b: *mut NkCommandBuffer, NkHandle userdata);
    draw_end: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_end)(b: *mut NkCommandBuffer, NkHandle userdata);
}

impl Default for NkStyleScrollbar {
	fn default() -> NkStyleScrollbar {
		NkStyleScrollbar {
		    /* background */
		    normal: Default::default(),
		    hover: Default::default(),
		    active: Default::default(),
		    border_color: Default::default(),
		
		    /* cursor */
		    cursor_normal: Default::default(),
		    cursor_hover: Default::default(),
		    cursor_active: Default::default(),
		    cursor_border_color: Default::default(),
		
		    /* properties */
		    border: 0.0,
		    rounding: 0.0,
		    border_cursor: 0.0,
		    rounding_cursor: 0.0,
		    padding: Default::default(),
		
		    /* optional buttons */
		    show_buttons: 0, 
		    inc_button: Default::default(),
		    dec_button: Default::default(),
		    inc_symbol: NkSymbolType::NK_SYMBOL_NONE,
		    dec_symbol: NkSymbolType::NK_SYMBOL_NONE,
		
		    /* optional user callbacks */
			userdata: Default::default(),
			draw_begin: None,
		    draw_end: None,
		}
	}
}

#[repr(C)]
pub struct NkStyleEdit {
    /* background */
    normal: NkStyleItem,
    hover: NkStyleItem,
    active: NkStyleItem,
    border_color: NkColor,
    scrollbar: NkStyleScrollbar,

    /* cursor  */
    cursor_normal: NkColor,
    cursor_hover: NkColor,
    cursor_text_normal: NkColor,
    cursor_text_hover: NkColor,

    /* text (unselected) */
    text_normal: NkColor,
    text_hover: NkColor,
    text_active: NkColor,

    /* text (selected) */
    selected_normal: NkColor,
    selected_hover: NkColor,
    selected_text_normal: NkColor,
    selected_text_hover: NkColor,

    /* properties */
    border : c_float,
    rounding: c_float,
    cursor_size: c_float,
    scrollbar_size: NkVec2,
    padding: NkVec2,
    row_padding: c_float,
}

impl Default for NkStyleEdit {
	fn default() -> NkStyleEdit {
		NkStyleEdit {
			/* background */
		    normal: Default::default(),
		    hover: Default::default(),
		    active: Default::default(),
		    border_color: Default::default(),
		    scrollbar: Default::default(),
		
		    /* cursor  */
		    cursor_normal: Default::default(),
		    cursor_hover: Default::default(),
		    cursor_text_normal: Default::default(),
		    cursor_text_hover: Default::default(),
		
		    /* text (unselected) */
		    text_normal: Default::default(),
		    text_hover: Default::default(),
		    text_active: Default::default(),
		
		    /* text (selected) */
		    selected_normal: Default::default(),
		    selected_hover: Default::default(),
		    selected_text_normal: Default::default(),
		    selected_text_hover: Default::default(),
		
		    /* properties */
		    border : 0.0,
		    rounding: 0.0,
		    cursor_size: 0.0,
		    scrollbar_size: Default::default(),
		    padding: Default::default(),
		    row_padding: 0.0,
		}
	}
}

#[repr(C)]
pub struct NkStyleProperty {
    /* background */
    normal: NkStyleItem,
    hover: NkStyleItem,
    active: NkStyleItem,
    border_color: NkColor,

    /* text */
    label_normal: NkColor,
    label_hover: NkColor,
    label_active: NkColor,

    /* symbols */
    sym_left: NkSymbolType,
    sym_right: NkSymbolType,

    /* properties */
    border: c_float,
    rounding: c_float,
    padding: NkVec2,

    edit: NkStyleEdit,
    inc_button: NkStyleButton,
    dec_button: NkStyleButton,

    /* optional user callbacks */
	userdata: NkHandle,
	draw_begin: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_begin)(b: *mut NkCommandBuffer, NkHandle userdata);
    draw_end: Option<extern "C" fn(b: *mut NkCommandBuffer, userdata: NkHandle)>,  //void(*draw_end)(b: *mut NkCommandBuffer, NkHandle userdata);
}

impl Default for NkStyleProperty {
	fn default() -> NkStyleProperty {
		NkStyleProperty {
	    	/* background */
		    normal: Default::default(),
		    hover: Default::default(),
		    active: Default::default(),
		    border_color: Default::default(),
		
		    /* text */
		    label_normal: Default::default(),
		    label_hover: Default::default(),
		    label_active: Default::default(),
		
		    /* symbols */
		    sym_left: NkSymbolType::NK_SYMBOL_NONE,
		    sym_right: NkSymbolType::NK_SYMBOL_NONE,
		
		    /* properties */
		    border: 0.0,
		    rounding: 0.0,
		    padding: Default::default(),
		
		    edit: Default::default(),
		    inc_button: Default::default(),
		    dec_button: Default::default(),
		
		    /* optional user callbacks */
			userdata: Default::default(),
			draw_begin: None,
		    draw_end: None,
	    }
	}
}

#[repr(C)]
pub struct NkStyleChart {
    /* colors */
    background: NkStyleItem,
    border_color: NkColor,
    selected_color: NkColor,
    color: NkColor,

    /* properties */
    border: c_float,
    rounding: c_float,
    padding: NkVec2,
}

impl Default for NkStyleChart {
	fn default() -> NkStyleChart {
		NkStyleChart {
	    	/* colors */
		    background: NkStyleItem::default(),
		    border_color: NkColor::default(),
		    selected_color: NkColor::default(),
		    color: NkColor::default(),
		
		    /* properties */
		    border: 0.0,
		    rounding: 0.0,
		    padding: NkVec2::default(),
	    }
	}
}

#[repr(C)]
pub struct NkStyleCombo {
    /* background */
    normal: NkStyleItem,
    hover: NkStyleItem,
    active: NkStyleItem,
    border_color: NkColor,

    /* label */
    label_normal: NkColor,
    label_hover: NkColor,
    label_active: NkColor,

    /* symbol */
    symbol_normal: NkColor,
    symbol_hover: NkColor,
    symbol_active: NkColor,

    /* button */
    button: NkStyleButton,
    sym_normal: NkSymbolType,
    sym_hover: NkSymbolType,
    sym_active: NkSymbolType,

    /* properties */
    border: c_float,
    rounding: c_float,
    content_padding: NkVec2,
    button_padding: NkVec2,
    spacing: NkVec2,
}

impl Default for NkStyleCombo {
	fn default() -> NkStyleCombo {
		NkStyleCombo {
	    	/* background */
		    normal: NkStyleItem::default(),
		    hover: NkStyleItem::default(),
		    active: NkStyleItem::default(),
		    border_color: NkColor::default(),
		
		    /* label */
		    label_normal: NkColor::default(),
		    label_hover: NkColor::default(),
		    label_active: NkColor::default(),
		
		    /* symbol */
		    symbol_normal: NkColor::default(),
		    symbol_hover: NkColor::default(),
		    symbol_active: NkColor::default(),
		
		    /* button */
		    button: NkStyleButton::default(),
		    sym_normal: NkSymbolType::NK_SYMBOL_NONE,
		    sym_hover: NkSymbolType::NK_SYMBOL_NONE,
		    sym_active: NkSymbolType::NK_SYMBOL_NONE,
		
		    /* properties */
		    border: 0.0,
		    rounding: 0.0,
		    content_padding: NkVec2::default(),
		    button_padding: NkVec2::default(),
		    spacing: NkVec2::default(),
	    }
	}
}

#[repr(C)]
pub struct NkStyleTab {
    /* background */
    background: NkStyleItem,
    border_color: NkColor,
    text: NkColor,

    /* button */
    tab_maximize_button: NkStyleButton,
    tab_minimize_button: NkStyleButton,
    node_maximize_button: NkStyleButton,
    node_minimize_button: NkStyleButton,
    sym_minimize: NkSymbolType,
    sym_maximize: NkSymbolType,

    /* properties */
    border: c_float,
    rounding: c_float,
    indent: c_float,
    padding: NkVec2,
    spacing: NkVec2,
}

impl Default for NkStyleTab {
	fn default() -> NkStyleTab {
		NkStyleTab {
	    	/* background */
		    background: NkStyleItem::default(),
		    border_color: NkColor::default(),
		    text: NkColor::default(),
		
		    /* button */
		    tab_maximize_button: NkStyleButton::default(),
		    tab_minimize_button: NkStyleButton::default(),
		    node_maximize_button: NkStyleButton::default(),
		    node_minimize_button: NkStyleButton::default(),
		    sym_minimize: NkSymbolType::NK_SYMBOL_NONE,
		    sym_maximize: NkSymbolType::NK_SYMBOL_NONE,
		
		    /* properties */
		    border: 0.0,
		    rounding: 0.0,
		    indent: 0.0,
		    padding: NkVec2::default(),
		    spacing: NkVec2::default(),
	    }
	}
}

#[repr(C)]
pub enum NkStyleHeaderAlign {
    NK_HEADER_LEFT,
    NK_HEADER_RIGHT
}

#[repr(C)]
pub struct NkStyleWindowHeader {
    /* background */
    normal: NkStyleItem,
    hover: NkStyleItem,
    active: NkStyleItem,

    /* button */
    close_button: NkStyleButton,
    minimize_button: NkStyleButton,
    close_symbol: NkSymbolType,
    minimize_symbol: NkSymbolType,
    maximize_symbol: NkSymbolType,

    /* title */
    label_normal: NkColor,
    label_hover: NkColor,
    label_active: NkColor,

    /* properties */
    align: NkStyleHeaderAlign,
    padding: NkVec2,
    label_padding: NkVec2,
    spacing: NkVec2,
}

impl Default for NkStyleWindowHeader {
	fn default() -> NkStyleWindowHeader {
		NkStyleWindowHeader {
		    /* background */
		    normal: NkStyleItem::default(),
		    hover: NkStyleItem::default(),
		    active: NkStyleItem::default(),
		
		    /* button */
		    close_button: NkStyleButton::default(),
		    minimize_button: NkStyleButton::default(),
		    close_symbol: NkSymbolType::NK_SYMBOL_NONE,
		    minimize_symbol: NkSymbolType::NK_SYMBOL_NONE,
		    maximize_symbol: NkSymbolType::NK_SYMBOL_NONE,
		
		    /* title */
		    label_normal: NkColor::default(),
		    label_hover: NkColor::default(),
		    label_active: NkColor::default(),
		
		    /* properties */
		    align: NkStyleHeaderAlign::NK_HEADER_LEFT,
		    padding: NkVec2::default(),
		    label_padding: NkVec2::default(),
		    spacing: NkVec2::default(),
		}
	}
}

#[repr(C)]
pub struct NkStyleWindow {
    header: NkStyleWindowHeader,
    fixed_background: NkStyleItem,
    background: NkColor,

    border_color: NkColor,
    combo_border_color: NkColor,
    contextual_border_color: NkColor,
    menu_border_color: NkColor,
    group_border_color: NkColor,
    tooltip_border_color: NkColor,

    scaler: NkStyleItem,
    footer_padding: NkVec2,

    border: c_float,
    combo_border: c_float,
    contextual_border: c_float,
    menu_border: c_float,
    group_border: c_float,
    tooltip_border: c_float,

    rounding: c_float,
    scaler_size: NkVec2,
    spacing: NkVec2,
    scrollbar_size: NkVec2,
    min_size: NkVec2,

    padding: NkVec2,
    group_padding: NkVec2,
    popup_padding: NkVec2,
    combo_padding: NkVec2,
    contextual_padding: NkVec2,
    menu_padding: NkVec2,
    tooltip_padding: NkVec2,
}

impl Default for NkStyleWindow {
	fn default() -> NkStyleWindow {
		NkStyleWindow {
    	    header: NkStyleWindowHeader::default(),
		    fixed_background: NkStyleItem::default(),
		    background: NkColor::default(),
		
		    border_color: NkColor::default(),
		    combo_border_color: NkColor::default(),
		    contextual_border_color: NkColor::default(),
		    menu_border_color: NkColor::default(),
		    group_border_color: NkColor::default(),
		    tooltip_border_color: NkColor::default(),
		
		    scaler: NkStyleItem::default(),
		    footer_padding: NkVec2::default(),
		
		    border: 0.0,
		    combo_border: 0.0,
		    contextual_border: 0.0,
		    menu_border: 0.0,
		    group_border: 0.0,
		    tooltip_border: 0.0,
		
		    rounding: 0.0,
		    scaler_size: NkVec2::default(),
		    spacing: NkVec2::default(),
		    scrollbar_size: NkVec2::default(),
		    min_size: NkVec2::default(),
		
		    padding: NkVec2::default(),
		    group_padding: NkVec2::default(),
		    popup_padding: NkVec2::default(),
		    combo_padding: NkVec2::default(),
		    contextual_padding: NkVec2::default(),
		    menu_padding: NkVec2::default(),
		    tooltip_padding: NkVec2::default(),
	    }
	}
}

#[repr(C)]
pub struct NkStyle {
    font: *const NkUserFont,
    cursors: *const [NkCursor; 7], //NK_CURSOR_COUNT],
    cursor_active: *const NkCursor,
    cursor_last: *mut NkCursor,
    cursor_visible: c_int,

    text: NkStyleText,
    button: NkStyleButton,
    contextual_button: NkStyleButton,
    menu_button: NkStyleButton,
    option: NkStyleToggle,
    checkbox: NkStyleToggle,
    selectable: NkStyleSelectable,
    slider: NkStyleSlider,
    progress: NkStyleProgress,
    property: NkStyleProperty,
    edit: NkStyleEdit,
    chart: NkStyleChart,
    scrollh: NkStyleScrollbar,
    scrollv: NkStyleScrollbar,
    tab: NkStyleTab,
    combo: NkStyleCombo,
    window: NkStyleWindow,
}

impl Default for NkStyle {
	fn default() -> NkStyle {
		NkStyle {
		    font: ::std::ptr::null_mut(),
		    cursors: ::std::ptr::null_mut(),
		    cursor_active: ::std::ptr::null_mut(),
		    cursor_last: ::std::ptr::null_mut(),
		    cursor_visible: 0,
		
		    text: NkStyleText::default(),
		    button: NkStyleButton::default(),
		    contextual_button: NkStyleButton::default(),
		    menu_button: NkStyleButton::default(),
		    option: NkStyleToggle::default(),
		    checkbox: NkStyleToggle::default(),
		    selectable: NkStyleSelectable::default(),
		    slider: NkStyleSlider::default(),
		    progress: NkStyleProgress::default(),
		    property: NkStyleProperty::default(),
		    edit: NkStyleEdit::default(),
		    chart: NkStyleChart::default(),
		    scrollh: NkStyleScrollbar::default(),
		    scrollv: NkStyleScrollbar::default(),
		    tab: NkStyleTab::default(),
		    combo: NkStyleCombo::default(),
		    window: NkStyleWindow::default(),
	    }
	}
}

extern "C" {
	pub fn nk_style_item_image(img: NkImage) -> NkStyleItem;
	pub fn nk_style_item_color(color: NkColor) -> NkStyleItem;
	pub fn nk_style_item_hide() -> NkStyleItem;
}

/*==============================================================
 *                          PANEL
 * =============================================================*/
//#ifndef NK_CHART_MAX_SLOT
pub const NK_CHART_MAX_SLOT: size_t = 4;
//#endif

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_chart_slot {
    ty: NkChartType,
    color: NkColor,
    highlight: NkColor,
    min: c_float, 
    max: c_float, 
    range: c_float,
    count: c_int,
    last: NkVec2,
    index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_chart {
    slots: [nk_chart_slot; NK_CHART_MAX_SLOT],
    slot: c_int,
    x: c_float, 
    y: c_float, 
    w: c_float, 
    h: c_float,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_row_layout {
    ty: c_int,
    index: c_int,
	height: c_float,
    columns: c_int,
    ratio: *const c_float,
    item_width: c_float, 
    item_height: c_float,
    item_offset: c_float,
    filled: c_float,
    item: NkRect,
    tree_depth: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_popup_buffer {
    begin: NkSize,
    parent: NkSize,
    last: NkSize,
    end: NkSize,
    active: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct nk_menu_state {
    x: c_float, 
    y: c_float, 
    w: c_float, 
    h: c_float,
    offset: NkScroll,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkPanel {
    flags: NkFlags,
    bounds: NkRect,
    offset: *mut NkScroll,
    at_x: c_float, 
    at_y: c_float, 
    max_x: c_float,
    width: c_float, 
    height: c_float,
    footer_h: c_float,
    header_h: c_float,
    border: c_float,
    has_scrolling: c_uint,
    clip: NkRect,
    menu: nk_menu_state,
    row: nk_row_layout,
    chart: nk_chart,
    popup_buffer: nk_popup_buffer,
    buffer: *mut NkCommandBuffer,
    parent: *mut NkPanel,
}

/*==============================================================
 *                          WINDOW
 * =============================================================*/

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NkWindowFlags {
    NK_WINDOW_PRIVATE       = 1 << 11, //NK_FLAG(11),
    /* dummy flag marks the beginning of the private window flag part */
    NK_WINDOW_ROM           = 1 << 12, //NK_FLAG(12),
    /* sets the window into a read only mode and does not allow input changes */
    NK_WINDOW_HIDDEN        = 1 << 13, //NK_FLAG(13),
    /* Hides the window and stops any window interaction and drawing */
    NK_WINDOW_CLOSED        = 1 << 14, //NK_FLAG(14),
    /* Directly closes and frees the window at the end of the frame */
    NK_WINDOW_MINIMIZED     = 1 << 15, //NK_FLAG(15),
    /* marks the window as minimized */
    NK_WINDOW_SUB           = 1 << 16, //NK_FLAG(16),
    /* Marks the window as subwindow of another window*/
    NK_WINDOW_GROUP         = 1 << 17, //NK_FLAG(17),
    /* Marks the window as window widget group */
    NK_WINDOW_POPUP         = 1 << 18, //NK_FLAG(18),
    /* Marks the window as a popup window */
    NK_WINDOW_NONBLOCK      = 1 << 19, //NK_FLAG(19),
    /* Marks the window as a nonblock popup window */
    NK_WINDOW_CONTEXTUAL    = 1 << 20, //NK_FLAG(20),
    /* Marks the window as a combo box or menu */
    NK_WINDOW_COMBO         = 1 << 21, //NK_FLAG(21),
    /* Marks the window as a combo box */
    NK_WINDOW_MENU          = 1 << 22, //NK_FLAG(22),
    /* Marks the window as a menu */
    NK_WINDOW_TOOLTIP       = 1 << 23, //NK_FLAG(23),
    /* Marks the window as a menu */
    NK_WINDOW_REMOVE_ROM    = 1 << 24, //NK_FLAG(24)
    /* Removes the read only mode at the end of the window */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkPopupState {
    win: *mut NkWindow,
    ty: c_uint,
    name: NkHash,
    active: c_int,
    combo_count: c_uint,
    con_count: c_uint, 
    con_old: c_uint,
    active_con: c_uint,
}

impl Default for NkPopupState {
	fn default() -> NkPopupState {
		NkPopupState {
		    win: ::std::ptr::null_mut(),
		    ty: 0,
		    name: 0,
		    active: 0,
		    combo_count: 0,
		    con_count: 0, 
		    con_old: 0,
		    active_con: 0,
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct NkEditState {
    name: NkHash,
    seq: c_uint,
    old: c_uint,
    active: c_int, 
    prev: c_int,
    cursor: c_int,
    sel_start: c_int,
    sel_end: c_int,
    scrollbar: NkScroll,
    mode: c_uchar,
    single_line: c_uchar,
}

#[repr(C)]
pub struct NkPropertyState {
    active: c_int, 
    prev: c_int,
    buffer: [c_char; NK_MAX_NUMBER_BUFFER],
    length: c_int,
    cursor: c_int,
    name: NkHash,
	seq: c_uint,
    old: c_uint,
    state: c_int,
}

impl Default for NkPropertyState {
	fn default() -> NkPropertyState {
		NkPropertyState {
		    active: 0, 
		    prev: 0,
		    buffer: [0; NK_MAX_NUMBER_BUFFER],
		    length: 0,
		    cursor: 0,
		    name: 0,
			seq: 0,
		    old: 0,
		    state: 0,
		}
	}
}

#[repr(C)]
pub struct NkWindow {
    seq: c_uint,
    name: NkHash,
    flags: NkFlags,
    bounds: NkRect,
    scrollbar: NkScroll,
    buffer: NkCommandBuffer,
    layout: *mut NkPanel,
    scrollbar_hiding_timer: c_float,

    /* persistent widget state */
    property: NkPropertyState,
    popup: NkPopupState,
    edit: NkEditState,
    scrolled: c_uint,

    tables: *mut NkTable,
    table_count: c_ushort,
    table_size: c_ushort,

    /* window list hooks */
    next: *mut NkWindow,
    prev: *mut NkWindow,
    parent: *mut NkWindow,
}

impl Default for NkWindow {
	fn default() -> NkWindow {
		NkWindow {
		    seq: 0,
		    name: 0,
		    flags: 0,
		    bounds: NkRect::default(),
		    scrollbar: NkScroll::default(),
		    buffer: NkCommandBuffer::default(),
		    layout: ::std::ptr::null_mut(),
		    scrollbar_hiding_timer: 0.0,
		
		    /* persistent widget state */
		    property: NkPropertyState::default(),
		    popup: NkPopupState::default(),
		    edit: NkEditState::default(),
		    scrolled: 0,
		
		    tables: ::std::ptr::null_mut(),
		    table_count: 0,
		    table_size: 0,
		
		    /* window list hooks */
		    next: ::std::ptr::null_mut(),
		    prev: ::std::ptr::null_mut(),
		    parent: ::std::ptr::null_mut(),
		}
	}
}

/*==============================================================
 *                          CONTEXT
 * =============================================================*/
pub const NK_VALUE_PAGE_CAPACITY: usize = 43; // -> c_int {((std::mem::size_of::<NkWindow>() / std::mem::size_of::<NkUint>()) / 2) as c_int} //((sizeof(struct NkWindow) / sizeof(NkUint)) / 2);

#[repr(C)]
pub struct NkTable {
    seq: c_uint,
    keys: [NkHash; NK_VALUE_PAGE_CAPACITY], 
    values: [NkUint; NK_VALUE_PAGE_CAPACITY], 
    next: *mut NkTable, 
    prev: *mut NkTable, 
}

impl Default for NkTable {
	fn default() -> NkTable {
		NkTable {
		    seq: 0,
		    keys: [0; NK_VALUE_PAGE_CAPACITY], 
		    values: [0; NK_VALUE_PAGE_CAPACITY], 
		    next: ::std::ptr::null_mut(), 
		    prev: ::std::ptr::null_mut(), 
		}
	}
}

#[repr(C)]
pub struct NkPageData {table: NkTable, wnd: NkWindow}

impl Default for NkPageData {
	fn default() -> NkPageData {
		NkPageData {table: NkTable::default(), wnd: NkWindow::default()}
	}
}

#[repr(C)]
pub struct NkPageElement {
    data: NkPageData,
    next: *mut NkPageElement,
    prev: *mut NkPageElement,
}

impl Default for NkPageElement {
	fn default() -> NkPageElement {
		NkPageElement {
		    data: NkPageData::default(),
		    next: ::std::ptr::null_mut(),
		    prev: ::std::ptr::null_mut(),
		}
	}
}

#[repr(C)]
pub struct NkPage {
    size: c_uint,
    next: *mut NkPage,
    win: [NkPageElement; 1],
}

impl Default for NkPage {
	fn default() -> NkPage {
		NkPage {
		    size: 0,
		    next: ::std::ptr::null_mut(),
		    win: [NkPageElement::default(); 1],
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NkPool {
    alloc: NkAllocator,
    ty: NkAllocationType,
    page_count: c_uint,
    pages: *mut NkPage,
    freelist: *mut NkPageElement,
    capacity: c_uint,
    size: NkSize,
    cap: NkSize,
}

impl Default for NkPool {
	fn default() -> NkPool {
		NkPool {
			alloc: NkAllocator::default(),
		    ty: NkAllocationType::NK_BUFFER_DYNAMIC,
		    page_count: 0,
		    pages: ::std::ptr::null_mut(),
		    freelist: ::std::ptr::null_mut(),
		    capacity: 0,
		    size: 0,
		    cap: 0,
		}
	}
}

#[repr(C)]
pub struct NkContext {
/* public: can be accessed freely */
    input: NkInput,
    style: NkStyle,
    memory: NkBuffer,
    clip: NkClipboard,
    last_widget_state: NkFlags,
    button_behavior: NkButtonBehavior,
    delta_time_seconds: c_float,

/* private:
    should only be accessed if you
    know what you are doing */
//#ifdef NK_INCLUDE_VERTEX_BUFFER_OUTPUT
    draw_list: NkDrawList,
//#endif
//#ifdef NK_INCLUDE_COMMAND_USERDATA
    userdata: NkHandle,
//#endif
    /* text editor objects are quite big because of an internal
     * undo/redo stack. Therefore does not make sense to have one for
     * each window for temporary use cases, so I only provide *one* instance
     * for all windows. This works because the content is cleared anyway */
    text_edit: NkTextEdit,
    /* draw buffer used for overlay drawing operation like cursor */
    overlay: NkCommandBuffer,

    /* windows */
    build: c_int,
    use_pool: c_int,
    pool: NkPool,
    begin: *mut NkWindow,
    end: *mut NkWindow,
    active: *mut NkWindow,
    current: *mut NkWindow,
    freelist: *mut NkPageElement,
    count: c_uint,
    seq: c_uint,
}

impl NkContext {
	pub fn init(&mut self, allocator: &mut NkAllocator, font: &NkUserFont) {
		unsafe {
			nk_init(self as *mut NkContext, allocator as *mut NkAllocator, font as *const NkUserFont); 
		}
	}
}

impl Default for NkContext {
	fn default() -> NkContext {
		NkContext {
			input: NkInput::default(),
		    style: NkStyle::default(),
		    memory: NkBuffer::default(),
		    clip: NkClipboard::default(),
		    last_widget_state: 0,
		    button_behavior: NkButtonBehavior::NK_BUTTON_DEFAULT,
		    delta_time_seconds: 0.0,
		
		/* private:
		    should only be accessed if you
		    know what you are doing */
		//#ifdef NK_INCLUDE_VERTEX_BUFFER_OUTPUT
		    draw_list: NkDrawList::default(),
		//#endif
		//#ifdef NK_INCLUDE_COMMAND_USERDATA
		    userdata: NkHandle::default(),
		//#endif
		    /* text editor objects are quite big because of an internal
		     * undo/redo stack. Therefore does not make sense to have one for
		     * each window for temporary use cases, so I only provide *one* instance
		     * for all windows. This works because the content is cleared anyway */
		    text_edit: NkTextEdit::default(),
		    /* draw buffer used for overlay drawing operation like cursor */
		    overlay: NkCommandBuffer::default(),
		
		    /* windows */
		    build: 0,
		    use_pool: 0,
		    pool: NkPool::default(),
		    begin: ::std::ptr::null_mut(),
		    end: ::std::ptr::null_mut(),
		    active: ::std::ptr::null_mut(),
		    current: ::std::ptr::null_mut(),
		    freelist: ::std::ptr::null_mut(),
		    count: 0,
		    seq: 0,
		}
	}
}

//=============================================================================================

const ALIGNMENT: usize = 16;
use alloc::heap;
use std::mem;

extern "C" fn alloc_rust(_: NkHandle, old: *mut c_void, size: NkSize) -> *mut c_void {
	unsafe {
		println!("allocating {} bytes", size);
		
        let size_size = mem::size_of::<NkSize>();
        let size = size + size_size;

        println!("allocating {} / {} bytes", size_size, size);
		let memory = if old.is_null() {
            heap::allocate(size, ALIGNMENT)
        } else {
            let old = old as *mut u8;
            let old = old.offset(-(size_size as isize));
            let old_size = *(old as *const usize);
            heap::reallocate(old, old_size, size, ALIGNMENT)
        };
		
		*(memory as *mut NkSize) = size;
        memory.offset(size_size as isize) as *mut c_void
    }
}

extern "C" fn free_rust(_: NkHandle, old: *mut c_void) {
    unsafe {
        let size_size = mem::size_of::<NkSize>();

        let old = old as *mut u8;
        let old = old.offset(-(size_size as isize));
        let old_size = *(old as *const NkSize);

        println!("deallocating {} bytes", old_size);
		
		heap::deallocate(old as *mut u8, old_size, ALIGNMENT);
    }
}

use std::slice;


extern "C" fn alloc_rust_hacky(hnd: NkHandle, old: *mut c_void, size: NkSize) -> *mut c_void {
	unsafe {
		if old.is_null() {
			free_rust_hacky(hnd, old);
		}
		
		println!("allocating {} bytes", size);
		let size_size = mem::size_of::<NkSize>();
        let size = size + size_size;

        println!("allocating {} / {} bytes", size_size, size);
		
		let mut v: Vec<u8> = Vec::with_capacity(size);
        //let array: [u8; usizelen] = unsafe { mem::transmute(size_size) };
        
        let ip: *const usize = &size_size;
		let bp: *const u8 = ip as *const _;
		let array: &[u8] = slice::from_raw_parts(
	        bp,
	        mem::size_of::<usize>()
	    );        
        
        for cv in array {
        	v.push(cv.clone());
        }
        
	    let ptr = v.as_mut_ptr();
	    std::mem::forget(v);
	    ptr.offset(size_size as isize) as *mut c_void
    }
}

extern "C" fn free_rust_hacky(_: NkHandle, old: *mut c_void) {
    unsafe {
        let size_size = mem::size_of::<NkSize>();

        let old = old as *mut u8;
        let old = old.offset(-(size_size as isize));
        let old_size = *(old as *const NkSize);

        println!("deallocating {} bytes", old_size);
		
		std::mem::drop(Vec::from_raw_parts(old, 0, old_size));
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
    fn test_alloc_dealloc() {
    	let mut h = NkHandle::default();
    	let mut pool = NkPool::default();
    	
    	println!("allocating 100500");
    	let mut mem = pool.alloc.alloc.unwrap()(h, ::std::ptr::null_mut(), 100500);
    	println!("freeing 100500");
    	pool.alloc.free.unwrap()(h, mem);
    }
}