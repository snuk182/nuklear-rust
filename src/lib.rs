#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ptr))] // TODO later
#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))] // TODO later
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))] // API requirement
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))] // API requirement
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))] // API requirement
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))] // required by allocator
#![cfg_attr(feature = "cargo-clippy", allow(non_upper_case_globals))]
#![allow(non_upper_case_globals)]
#![cfg_attr(feature = "rust_allocator", feature(allocator_api))]

#[macro_use]
extern crate log;

use std::borrow::Cow;
use std::default::Default;
use std::os::raw::*;

use nuklear_sys::*;
pub use nuklear_sys;
pub use nuklear_sys::nk_allocation_type as AllocationType;
pub use nuklear_sys::nk_chart_slot as ChartSlot;
pub use nuklear_sys::nk_color as Color;
pub use nuklear_sys::nk_colorf as ColorF;
pub use nuklear_sys::nk_draw_list_stroke as DrawListStroke;
pub use nuklear_sys::nk_flags as Flags;
//TODO
pub use nuklear_sys::nk_font_coord_type as FontCoordType;
pub use nuklear_sys::nk_glyph as Glyph;
pub use nuklear_sys::nk_menu_state as MenuState;
pub use nuklear_sys::nk_panel_row_layout_type as PanelRowLayoutType;
pub use nuklear_sys::nk_panel_type as PanelType;
pub use nuklear_sys::nk_plugin_copy as PluginCopy;
pub use nuklear_sys::nk_plugin_filter as PluginFilter;
pub use nuklear_sys::nk_plugin_paste as PluginPaste;
pub use nuklear_sys::nk_popup_buffer as PopupBuffer;
pub use nuklear_sys::nk_rect as Rect;
pub use nuklear_sys::nk_recti as Recti;
pub use nuklear_sys::nk_scroll as Scroll;
pub use nuklear_sys::nk_size as Size;
pub use nuklear_sys::nk_style_colors as StyleColor;
pub use nuklear_sys::nk_style_cursor as StyleCursor;
pub use nuklear_sys::nk_style_header_align as StyleHeaderAlign;
pub use nuklear_sys::nk_style_text as StyleText;
pub use nuklear_sys::nk_vec2 as Vec2;
pub use nuklear_sys::nk_vec2i as Vec2i;
pub use nuklear_sys::nk_widget_layout_states as WidgetLayoutState;

#[cfg(feature = "rust_allocator")]
mod alloc_heap;
mod alloc_vec;

pub const NK_FILTER_DEFAULT: PluginFilter = Some(nk_filter_default);
pub const NK_FILTER_ASCII: PluginFilter = Some(nk_filter_ascii);
pub const NK_FILTER_FLOAT: PluginFilter = Some(nk_filter_float);
pub const NK_FILTER_DECIMAL: PluginFilter = Some(nk_filter_decimal);
pub const NK_FILTER_HEX: PluginFilter = Some(nk_filter_hex);
pub const NK_FILTER_OCT: PluginFilter = Some(nk_filter_oct);
pub const NK_FILTER_BINARY: PluginFilter = Some(nk_filter_binary);

pub const ALIGNMENT: usize = 16;

macro_rules! wrapper_impls {
    ($name:ident, $typ:ty) => {
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
                unsafe { &*(self as *const $typ as *const $name) }
            }
        }
        impl AsMut<$name> for $typ {
            fn as_mut(&mut self) -> &mut $name {
                unsafe { &mut *(self as *mut $typ as *mut $name) }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name { internal: unsafe { ::std::mem::zeroed() } }
            }
        }
    };
}

macro_rules! wrapper_type {
    ($name:ident, $typ:ty) => {
        #[derive(Clone)]
        #[repr(C)]
        pub struct $name {
            internal: $typ,
        }

        wrapper_impls!($name, $typ);
    };
}

macro_rules! wrapper_type_no_clone {
    ($name:ident, $typ:ty) => {
        #[repr(C)]
        pub struct $name {
            internal: $typ,
        }

        wrapper_impls!($name, $typ);
    };
}

macro_rules! from_into_enum {
    ($name:ident, $typ:ty) => {
        impl From<$name> for $typ {
            fn from(a: $name) -> $typ {
                a as $typ
            }
        }
        impl From<$typ> for $name {
            fn from(a: $typ) -> $name {
                unsafe { ::std::mem::transmute(a) }
            }
        }
        impl<'a> From<&'a $typ> for &'a $name {
            fn from(a: &'a $typ) -> &'a $name {
                unsafe { ::std::mem::transmute(a) }
            }
        }
    };
}

// ==========================================================================================================

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    Nop = nk_command_type_NK_COMMAND_NOP as isize,
    Scissor = nk_command_type_NK_COMMAND_SCISSOR as isize,
    Line = nk_command_type_NK_COMMAND_LINE as isize,
    Curve = nk_command_type_NK_COMMAND_CURVE as isize,
    Rect = nk_command_type_NK_COMMAND_RECT as isize,
    RectFilled = nk_command_type_NK_COMMAND_RECT_FILLED as isize,
    RectMultiColor = nk_command_type_NK_COMMAND_RECT_MULTI_COLOR as isize,
    Circle = nk_command_type_NK_COMMAND_CIRCLE as isize,
    CircleFilled = nk_command_type_NK_COMMAND_CIRCLE_FILLED as isize,
    Arc = nk_command_type_NK_COMMAND_ARC as isize,
    ArcFilled = nk_command_type_NK_COMMAND_ARC_FILLED as isize,
    Triangle = nk_command_type_NK_COMMAND_TRIANGLE as isize,
    TriangleFilled = nk_command_type_NK_COMMAND_TRIANGLE_FILLED as isize,
    Polygon = nk_command_type_NK_COMMAND_POLYGON as isize,
    PolygonFilled = nk_command_type_NK_COMMAND_POLYGON_FILLED as isize,
    Polyline = nk_command_type_NK_COMMAND_POLYLINE as isize,
    Text = nk_command_type_NK_COMMAND_TEXT as isize,
    Image = nk_command_type_NK_COMMAND_IMAGE as isize,
    Custom = nk_command_type_NK_COMMAND_CUSTOM as isize,
}
from_into_enum!(CommandType, nk_command_type);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    None = nk_symbol_type_NK_SYMBOL_NONE as isize,
    X = nk_symbol_type_NK_SYMBOL_X as isize,
    Underscore = nk_symbol_type_NK_SYMBOL_UNDERSCORE as isize,
    CircleSolid = nk_symbol_type_NK_SYMBOL_CIRCLE_SOLID as isize,
    CircleOutline = nk_symbol_type_NK_SYMBOL_CIRCLE_OUTLINE as isize,
    RectSolid = nk_symbol_type_NK_SYMBOL_RECT_SOLID as isize,
    RectOutline = nk_symbol_type_NK_SYMBOL_RECT_OUTLINE as isize,
    TriangleUp = nk_symbol_type_NK_SYMBOL_TRIANGLE_UP as isize,
    TriangleDown = nk_symbol_type_NK_SYMBOL_TRIANGLE_DOWN as isize,
    TriangleLeft = nk_symbol_type_NK_SYMBOL_TRIANGLE_LEFT as isize,
    TriangleRight = nk_symbol_type_NK_SYMBOL_TRIANGLE_RIGHT as isize,
    Plus = nk_symbol_type_NK_SYMBOL_PLUS as isize,
    Minus = nk_symbol_type_NK_SYMBOL_MINUS as isize,
    Max = nk_symbol_type_NK_SYMBOL_MAX as isize,
}
from_into_enum!(SymbolType, nk_symbol_type);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditFlag {
    Default = nk_edit_flags_NK_EDIT_DEFAULT as isize,
    ReadOnly = nk_edit_flags_NK_EDIT_READ_ONLY as isize,
    AutoSelect = nk_edit_flags_NK_EDIT_AUTO_SELECT as isize,
    SigEnter = nk_edit_flags_NK_EDIT_SIG_ENTER as isize,
    AllowTab = nk_edit_flags_NK_EDIT_ALLOW_TAB as isize,
    NoCursor = nk_edit_flags_NK_EDIT_NO_CURSOR as isize,
    Selectable = nk_edit_flags_NK_EDIT_SELECTABLE as isize,
    Clipboard = nk_edit_flags_NK_EDIT_CLIPBOARD as isize,
    CtrlEnterNewline = nk_edit_flags_NK_EDIT_CTRL_ENTER_NEWLINE as isize,
    NoHorizontalScroll = nk_edit_flags_NK_EDIT_NO_HORIZONTAL_SCROLL as isize,
    AlwaysInsertMode = nk_edit_flags_NK_EDIT_ALWAYS_INSERT_MODE as isize,
    Multiline = nk_edit_flags_NK_EDIT_MULTILINE as isize,
    GoToEndOnActivate = nk_edit_flags_NK_EDIT_GOTO_END_ON_ACTIVATE as isize,
}
from_into_enum!(EditFlag, nk_edit_flags);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditType {
    Simple = nk_edit_types_NK_EDIT_SIMPLE as isize,
    Field = nk_edit_types_NK_EDIT_FIELD as isize,
    Box = nk_edit_types_NK_EDIT_BOX as isize,
    Editor = nk_edit_types_NK_EDIT_EDITOR as isize,
}
from_into_enum!(EditType, nk_edit_types);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditEvent {
    Active = nk_edit_events_NK_EDIT_ACTIVE as isize,
    Inactive = nk_edit_events_NK_EDIT_INACTIVE as isize,
    Activated = nk_edit_events_NK_EDIT_ACTIVATED as isize,
    Deactivated = nk_edit_events_NK_EDIT_DEACTIVATED as isize,
    Commited = nk_edit_events_NK_EDIT_COMMITED as isize,
}
from_into_enum!(EditEvent, nk_edit_events);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PanelFlags {
    Border = nk_panel_flags_NK_WINDOW_BORDER as isize,
    Movable = nk_panel_flags_NK_WINDOW_MOVABLE as isize,
    Scalable = nk_panel_flags_NK_WINDOW_SCALABLE as isize,
    Closable = nk_panel_flags_NK_WINDOW_CLOSABLE as isize,
    Minimizable = nk_panel_flags_NK_WINDOW_MINIMIZABLE as isize,
    NoScrollbar = nk_panel_flags_NK_WINDOW_NO_SCROLLBAR as isize,
    Title = nk_panel_flags_NK_WINDOW_TITLE as isize,
    ScrollAutoHide = nk_panel_flags_NK_WINDOW_SCROLL_AUTO_HIDE as isize,
    Background = nk_panel_flags_NK_WINDOW_BACKGROUND as isize,
    ScaleLeft = nk_panel_flags_NK_WINDOW_SCALE_LEFT as isize,
    NoInput = nk_panel_flags_NK_WINDOW_NO_INPUT as isize,
}
from_into_enum!(PanelFlags, nk_panel_flags);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Heading {
    Up = nk_heading_NK_UP as isize,
    Right = nk_heading_NK_RIGHT as isize,
    Down = nk_heading_NK_DOWN as isize,
    Left = nk_heading_NK_LEFT as isize,
}
from_into_enum!(Heading, nk_heading);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonBehavior {
    Default = nk_button_behavior_NK_BUTTON_DEFAULT as isize,
    Repeater = nk_button_behavior_NK_BUTTON_REPEATER as isize,
}
from_into_enum!(ButtonBehavior, nk_button_behavior);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Modify {
    Fixed = nk_modify_NK_FIXED as isize,
    Modifiable = nk_modify_NK_MODIFIABLE as isize,
}
from_into_enum!(Modify, nk_modify);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Vertical = nk_orientation_NK_VERTICAL as isize,
    Horizontal = nk_orientation_NK_HORIZONTAL as isize,
}
from_into_enum!(Orientation, nk_orientation);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CollapseState {
    Minimized = nk_collapse_states_NK_MINIMIZED as isize,
    Maximized = nk_collapse_states_NK_MAXIMIZED as isize,
}
from_into_enum!(CollapseState, nk_collapse_states);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShowState {
    Hidden = nk_show_states_NK_HIDDEN as isize,
    Shown = nk_show_states_NK_SHOWN as isize,
}
from_into_enum!(ShowState, nk_show_states);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChartType {
    Lines = nk_chart_type_NK_CHART_LINES as isize,
    Column = nk_chart_type_NK_CHART_COLUMN as isize,
    Max = nk_chart_type_NK_CHART_MAX as isize,
}
from_into_enum!(ChartType, nk_chart_type);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChartEvent {
    Hovering = nk_chart_event_NK_CHART_HOVERING as isize,
    Clicked = nk_chart_event_NK_CHART_CLICKED as isize,
}
from_into_enum!(ChartEvent, nk_chart_event);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorFormat {
    Rgb = nk_color_format_NK_RGB as isize,
    Rgba = nk_color_format_NK_RGBA as isize,
}
from_into_enum!(ColorFormat, nk_color_format);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PopupType {
    Static = nk_popup_type_NK_POPUP_STATIC as isize,
    Dynamic = nk_popup_type_NK_POPUP_DYNAMIC as isize,
}
from_into_enum!(PopupType, nk_popup_type);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LayoutFormat {
    Dynamic = nk_layout_format_NK_DYNAMIC as isize,
    Static = nk_layout_format_NK_STATIC as isize,
}
from_into_enum!(LayoutFormat, nk_layout_format);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TreeType {
    Node = nk_tree_type_NK_TREE_NODE as isize,
    Tab = nk_tree_type_NK_TREE_TAB as isize,
}
from_into_enum!(TreeType, nk_tree_type);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left = nk_text_align_NK_TEXT_ALIGN_LEFT as isize,
    Centered = nk_text_align_NK_TEXT_ALIGN_CENTERED as isize,
    Right = nk_text_align_NK_TEXT_ALIGN_RIGHT as isize,
    Top = nk_text_align_NK_TEXT_ALIGN_TOP as isize,
    Middle = nk_text_align_NK_TEXT_ALIGN_MIDDLE as isize,
    Bottom = nk_text_align_NK_TEXT_ALIGN_BOTTOM as isize,
}
from_into_enum!(TextAlign, nk_text_align);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left = nk_text_alignment_NK_TEXT_LEFT as isize,
    Centered = nk_text_alignment_NK_TEXT_CENTERED as isize,
    Right = nk_text_alignment_NK_TEXT_RIGHT as isize,
}
from_into_enum!(TextAlignment, nk_text_alignment);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Key {
    None = nk_keys_NK_KEY_NONE as isize,
    Shift = nk_keys_NK_KEY_SHIFT as isize,
    Ctrl = nk_keys_NK_KEY_CTRL as isize,
    Del = nk_keys_NK_KEY_DEL as isize,
    Enter = nk_keys_NK_KEY_ENTER as isize,
    Tab = nk_keys_NK_KEY_TAB as isize,
    Backspace = nk_keys_NK_KEY_BACKSPACE as isize,
    Copy = nk_keys_NK_KEY_COPY as isize,
    Cut = nk_keys_NK_KEY_CUT as isize,
    Paste = nk_keys_NK_KEY_PASTE as isize,
    Up = nk_keys_NK_KEY_UP as isize,
    Down = nk_keys_NK_KEY_DOWN as isize,
    Left = nk_keys_NK_KEY_LEFT as isize,
    Right = nk_keys_NK_KEY_RIGHT as isize,
    InsertMode = nk_keys_NK_KEY_TEXT_INSERT_MODE as isize,
    ReplaceMode = nk_keys_NK_KEY_TEXT_REPLACE_MODE as isize,
    ResetMode = nk_keys_NK_KEY_TEXT_RESET_MODE as isize,
    LineStart = nk_keys_NK_KEY_TEXT_LINE_START as isize,
    LineEnd = nk_keys_NK_KEY_TEXT_LINE_END as isize,
    TextStart = nk_keys_NK_KEY_TEXT_START as isize,
    TextEnd = nk_keys_NK_KEY_TEXT_END as isize,
    TextUndo = nk_keys_NK_KEY_TEXT_UNDO as isize,
    TextRedo = nk_keys_NK_KEY_TEXT_REDO as isize,
    TextSelectAll = nk_keys_NK_KEY_TEXT_SELECT_ALL as isize,
    TextWordLeft = nk_keys_NK_KEY_TEXT_WORD_LEFT as isize,
    TextWordRight = nk_keys_NK_KEY_TEXT_WORD_RIGHT as isize,
    ScrollStart = nk_keys_NK_KEY_SCROLL_START as isize,
    ScrollEnd = nk_keys_NK_KEY_SCROLL_END as isize,
    ScrollDown = nk_keys_NK_KEY_SCROLL_DOWN as isize,
    ScrollUp = nk_keys_NK_KEY_SCROLL_UP as isize,
}
from_into_enum!(Key, nk_keys);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left = nk_buttons_NK_BUTTON_LEFT as isize,
    Middle = nk_buttons_NK_BUTTON_MIDDLE as isize,
    Right = nk_buttons_NK_BUTTON_RIGHT as isize,
    Double = nk_buttons_NK_BUTTON_DOUBLE as isize,
    Max = nk_buttons_NK_BUTTON_MAX as isize,
}
from_into_enum!(Button, nk_buttons);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AntiAliasing {
    Off = nk_anti_aliasing_NK_ANTI_ALIASING_OFF as isize,
    On = nk_anti_aliasing_NK_ANTI_ALIASING_ON as isize,
}
from_into_enum!(AntiAliasing, nk_anti_aliasing);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DrawVertexLayoutFormat {
    Char = nk_draw_vertex_layout_format_NK_FORMAT_SCHAR as isize,
    Short = nk_draw_vertex_layout_format_NK_FORMAT_SSHORT as isize,
    Int = nk_draw_vertex_layout_format_NK_FORMAT_SINT as isize,
    Uchar = nk_draw_vertex_layout_format_NK_FORMAT_UCHAR as isize,
    Ushort = nk_draw_vertex_layout_format_NK_FORMAT_USHORT as isize,
    Uint = nk_draw_vertex_layout_format_NK_FORMAT_UINT as isize,
    Float = nk_draw_vertex_layout_format_NK_FORMAT_FLOAT as isize,
    Double = nk_draw_vertex_layout_format_NK_FORMAT_DOUBLE as isize,
    R8G8B8 = nk_draw_vertex_layout_format_NK_FORMAT_R8G8B8 as isize,
    R16G16B16 = nk_draw_vertex_layout_format_NK_FORMAT_R16G15B16 as isize,
    R32G32B32 = nk_draw_vertex_layout_format_NK_FORMAT_R32G32B32 as isize,
    R8G8B8A8 = nk_draw_vertex_layout_format_NK_FORMAT_R8G8B8A8 as isize,
    B8G8R8A8 = nk_draw_vertex_layout_format_NK_FORMAT_B8G8R8A8 as isize,
    R16G15B16A16 = nk_draw_vertex_layout_format_NK_FORMAT_R16G15B16A16 as isize,
    R32G32B32A32 = nk_draw_vertex_layout_format_NK_FORMAT_R32G32B32A32 as isize,
    R32G32B32A32Float = nk_draw_vertex_layout_format_NK_FORMAT_R32G32B32A32_FLOAT as isize,
    R32G32B32A32Double = nk_draw_vertex_layout_format_NK_FORMAT_R32G32B32A32_DOUBLE as isize,
    Rgb32 = nk_draw_vertex_layout_format_NK_FORMAT_RGB32 as isize,
    Rgba32 = nk_draw_vertex_layout_format_NK_FORMAT_RGBA32 as isize,
    Count = nk_draw_vertex_layout_format_NK_FORMAT_COUNT as isize,
}
from_into_enum!(DrawVertexLayoutFormat, nk_draw_vertex_layout_format);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DrawVertexLayoutAttribute {
    Position = nk_draw_vertex_layout_attribute_NK_VERTEX_POSITION as isize,
    Color = nk_draw_vertex_layout_attribute_NK_VERTEX_COLOR as isize,
    TexCoord = nk_draw_vertex_layout_attribute_NK_VERTEX_TEXCOORD as isize,
    AttributeCount = nk_draw_vertex_layout_attribute_NK_VERTEX_ATTRIBUTE_COUNT as isize,
}
from_into_enum!(DrawVertexLayoutAttribute, nk_draw_vertex_layout_attribute);

// ==========================================================================================================

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FontAtlasFormat {
    Alpha8 = nk_font_atlas_format_NK_FONT_ATLAS_ALPHA8 as isize,
    Rgba32 = nk_font_atlas_format_NK_FONT_ATLAS_RGBA32 as isize,
}
from_into_enum!(FontAtlasFormat, nk_font_atlas_format);

// ==========================================================================================================

unsafe extern "C" fn nk_filter_custom(arg1: *const nk_text_edit, unicode: nk_rune) -> ::std::os::raw::c_int {
    if let Some(f) = CUSTOM_EDIT_FILTER {
        if f(&*(arg1 as *const TextEdit), ::std::char::from_u32_unchecked(unicode)) {
            1
        } else {
            0
        }
    } else {
        1
    }
}

static mut CUSTOM_EDIT_FILTER: Option<fn(&TextEdit, char) -> bool> = None;

// ===========================================================================================================

// unsafe extern "C" fn nk_plot_value_getter_custom(user: *mut ::std::os::raw::c_void, index: ::std::os::raw::c_int) -> f32 {
// let f = user as *const _ as &[f32];
// f[index as usize]
// }

// ===========================================================================================================

#[derive(Clone)]
pub struct String<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> String<'a> {
    pub unsafe fn from_bytes_unchecked(bytes: &'a [u8]) -> String<'a> {
        String { bytes: Cow::Borrowed(bytes) }
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

impl<'a> From<&'a str> for String<'a> {
    fn from(value: &'a str) -> String<'a> {
        let mut bytes: Vec<u8> = value.bytes().collect();
        bytes.push(0);
        String { bytes: Cow::Owned(bytes) }
    }
}

impl<'a> From<::std::string::String> for String<'a> {
    fn from(mut value: ::std::string::String) -> String<'a> {
        value.push('\0');
        String { bytes: Cow::Owned(value.into_bytes()) }
    }
}

#[macro_export]
macro_rules! nk_string {
    ($e:tt) => ({
        let value = concat!($e, "\0");
        unsafe { $crate::String::from_bytes_unchecked(value.as_bytes()) }
    });
    ($e:tt, $($arg:tt)*) => ({
        $crate::String::from(format!($e, $($arg)*))
    })
}

// ======================================================================================

#[derive(Clone)]
pub struct StringArray<'a> {
    arr: Vec<String<'a>>,
    ptrs: Vec<*const c_char>,
}

impl<'a> StringArray<'a> {
    pub fn as_ptr(&self) -> *const *const c_char {
        self.ptrs.as_slice() as *const _ as *const *const c_char
    }
    pub fn as_mut(&mut self) -> *mut *const c_char {
        self.ptrs.as_mut_slice() as *mut _ as *mut *const c_char
    }
    pub fn len(&self) -> usize {
        self.ptrs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() < 1
    }
}

impl<'a> From<&'a [&'a str]> for StringArray<'a> {
    fn from(value: &[&'a str]) -> StringArray<'a> {
        let mut r = StringArray {
            arr: Vec::with_capacity(value.len()),
            ptrs: Vec::with_capacity(value.len()),
        };

        for s in value {
            r.arr.push(String::from(*s));
            r.ptrs.push(r.arr[r.arr.len() - 1].as_ptr());
        }

        r
    }
}

// ======================================================================================

#[derive(Debug, Clone, PartialEq, Copy)]
enum HandleKind {
    Empty,
    Ptr,
    Id,
    Unknown,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Handle {
    internal: nk_handle,
    kind: HandleKind,
}

impl Default for Handle {
    fn default() -> Self {
        Handle {
            kind: HandleKind::Empty,
            internal: nk_handle::default(),
        }
    }
}

impl Handle {
    pub fn id(&mut self) -> Option<i32> {
        match self.kind {
            HandleKind::Id | HandleKind::Unknown => Some(unsafe { self.internal.id }),
            _ => None,
        }
    }

    pub fn ptr(&mut self) -> Option<*mut c_void> {
        match self.kind {
            HandleKind::Ptr | HandleKind::Unknown => Some(unsafe { self.internal.ptr }),
            _ => None,
        }
    }

    pub fn from_id(value: i32) -> Handle {
        Handle {
            kind: HandleKind::Id,
            internal: unsafe { nk_handle_id(value) },
        }
    }

    pub unsafe fn from_ptr(value: *mut c_void) -> Handle {
        Handle { kind: HandleKind::Ptr, internal: nk_handle_ptr(value) }
    }
}

// ==================================================================================

/*
wrapper_type!(ConfigurationStacks, nk_configuration_stacks);

impl ConfigurationStacks {
    pub style_items: nk_config_stack_style_item,
    pub floats: nk_config_stack_float,
    pub vectors: nk_config_stack_vec2,
    pub flags: nk_config_stack_flags,
    pub colors: nk_config_stack_color,
    pub fonts: nk_config_stack_user_font,
    pub button_behaviors: nk_config_stack_button_behavior,

}
*/

// ==================================================================================

wrapper_type!(Clipboard, nk_clipboard);

impl Clipboard {
    pub unsafe fn userdata_ptr(&self) -> Handle {
        Handle::from_ptr(self.internal.userdata.ptr)
    }
    pub unsafe fn userdata_id(&self) -> Handle {
        Handle::from_id(self.internal.userdata.id)
    }

    pub fn paste(&self) -> PluginPaste {
        self.internal.paste
    }
    pub fn set_paste(&mut self, plug: PluginPaste) {
        self.internal.paste = plug;
    }
    pub fn copy(&self) -> PluginCopy {
        self.internal.copy
    }
    pub fn set_copy(&mut self, plug: PluginCopy) {
        self.internal.copy = plug;
    }
}

// ==================================================================================

wrapper_type!(Input, nk_input);

impl Input {
    pub fn mouse(&self) -> Mouse {
        Mouse { internal: self.internal.mouse }
    }

    pub fn has_mouse_click(&self, b: Button) -> bool {
        unsafe { nk_input_has_mouse_click(&self.internal, b.into()) != 0 }
    }

    pub fn has_mouse_click_in_rect(&self, b: Button, rect: Rect) -> bool {
        unsafe { nk_input_has_mouse_click_in_rect(&self.internal, b.into(), rect) != 0 }
    }

    pub fn has_mouse_click_down_in_rect(&self, b: Button, rect: Rect, down: bool) -> bool {
        unsafe { nk_input_has_mouse_click_down_in_rect(&self.internal, b.into(), rect, if down { 1 } else { 0 }) != 0 }
    }

    pub fn is_mouse_click_in_rect(&self, b: Button, rect: Rect) -> bool {
        unsafe { nk_input_is_mouse_click_in_rect(&self.internal, b.into(), rect) != 0 }
    }

    pub fn is_mouse_click_down_in_rect(&self, b: Button, rect: Rect, down: bool) -> bool {
        unsafe { nk_input_is_mouse_click_down_in_rect(&self.internal, b.into(), rect, down as ::std::os::raw::c_int) != 0 }
    }

    pub fn any_mouse_click_in_rect(&self, rect: Rect) -> bool {
        unsafe { nk_input_any_mouse_click_in_rect(&self.internal, rect) != 0 }
    }

    pub fn is_mouse_prev_hovering_rect(&self, rect: Rect) -> bool {
        unsafe { nk_input_is_mouse_prev_hovering_rect(&self.internal, rect) != 0 }
    }

    pub fn is_mouse_hovering_rect(&self, rect: Rect) -> bool {
        unsafe { nk_input_is_mouse_hovering_rect(&self.internal, rect) != 0 }
    }

    pub fn is_mouse_clicked(&self, b: Button, rect: Rect) -> bool {
        unsafe { nk_input_mouse_clicked(&self.internal, b.into(), rect) != 0 }
    }

    pub fn is_mouse_down(&self, b: Button) -> bool {
        unsafe { nk_input_is_mouse_down(&self.internal, b.into()) != 0 }
    }

    pub fn is_mouse_pressed(&self, b: Button) -> bool {
        unsafe { nk_input_is_mouse_pressed(&self.internal, b.into()) != 0 }
    }

    pub fn is_mouse_released(&self, b: Button) -> bool {
        unsafe { nk_input_is_mouse_released(&self.internal, b.into()) != 0 }
    }

    pub fn is_key_pressed(&self, k: Key) -> bool {
        unsafe { nk_input_is_key_pressed(&self.internal, k.into()) != 0 }
    }

    pub fn is_key_released(&self, k: Key) -> bool {
        unsafe { nk_input_is_key_released(&self.internal, k.into()) != 0 }
    }

    pub fn is_key_down(&self, k: Key) -> bool {
        unsafe { nk_input_is_key_down(&self.internal, k.into()) != 0 }
    }
}

// =====================================================================

wrapper_type!(DrawCommand, nk_draw_command);

impl DrawCommand {
    pub fn clip_rect(&self) -> &Rect {
        &self.internal.clip_rect
    }

    pub fn elem_count(&self) -> u32 {
        self.internal.elem_count
    }

    pub fn texture(&self) -> Handle {
        Handle {
            kind: HandleKind::Unknown,
            internal: self.internal.texture,
        }
    }
}

// =====================================================================

#[derive(Copy, Clone, Debug)]
pub struct MouseButton {
    pub down: bool,
    pub clicked: bool,
    pub clicked_pos: Vec2,
}

impl MouseButton {
    fn from_native(n: nk_mouse_button) -> MouseButton {
        MouseButton {
            down: n.down > 0,
            clicked: n.clicked > 0,
            clicked_pos: n.clicked_pos,
        }
    }
}

wrapper_type!(Mouse, nk_mouse);

impl Mouse {
    pub fn pos(&self) -> &Vec2 {
        &self.internal.pos
    }

    pub fn prev(&self) -> &Vec2 {
        &self.internal.prev
    }

    pub fn delta(&self) -> &Vec2 {
        &self.internal.delta
    }

    pub fn scroll_delta(&self) -> &Vec2 {
        &self.internal.scroll_delta
    }

    pub fn buttons(&self) -> [MouseButton; 3] {
        [MouseButton::from_native(self.internal.buttons[0]), MouseButton::from_native(self.internal.buttons[1]), MouseButton::from_native(self.internal.buttons[2])]
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

wrapper_type!(Style, nk_style);

impl Style {
    // ===== mut getters =====

    pub fn window_mut(&mut self) -> &mut StyleWindow {
        unsafe { ::std::mem::transmute(&mut self.internal.window) }
    }

    pub fn font_mut(&mut self) -> &mut UserFont {
        unsafe { ::std::mem::transmute(self.internal.font) }
    }

    pub fn cursors_mut(&mut self) -> &mut CursorMap {
        unsafe { ::std::mem::transmute(&mut self.internal.cursors) }
    }

    pub fn cursor_active_mut(&mut self) -> &mut Cursor {
        unsafe { ::std::mem::transmute(&mut self.internal.cursor_active) }
    }

    pub fn set_cursor_visible(&mut self, value: bool) {
        self.internal.cursor_visible = if value { 1 } else { 0 }
    }

    pub fn text_mut(&mut self) -> &mut StyleText {
        &mut self.internal.text
    }

    pub fn button_mut(&mut self) -> &mut StyleButton {
        unsafe { ::std::mem::transmute(&mut self.internal.button) }
    }

    pub fn contextual_button_mut(&mut self) -> &mut StyleButton {
        unsafe { ::std::mem::transmute(&mut self.internal.contextual_button) }
    }

    pub fn menu_button_mut(&mut self) -> &mut StyleButton {
        unsafe { ::std::mem::transmute(&mut self.internal.menu_button) }
    }

    pub fn option_mut(&mut self) -> &mut StyleToggle {
        unsafe { ::std::mem::transmute(&mut self.internal.option) }
    }

    pub fn checkbox_mut(&mut self) -> &mut StyleToggle {
        unsafe { ::std::mem::transmute(&mut self.internal.checkbox) }
    }

    pub fn selectable_mut(&mut self) -> &mut StyleSelectable {
        unsafe { ::std::mem::transmute(&mut self.internal.selectable) }
    }

    pub fn slider_mut(&mut self) -> &mut StyleSlider {
        unsafe { ::std::mem::transmute(&mut self.internal.slider) }
    }

    pub fn progress_mut(&mut self) -> &mut StyleProgress {
        unsafe { ::std::mem::transmute(&mut self.internal.progress) }
    }

    pub fn property_mut(&mut self) -> &mut StyleProperty {
        unsafe { ::std::mem::transmute(&mut self.internal.property) }
    }

    pub fn edit_mut(&mut self) -> &mut StyleEdit {
        unsafe { ::std::mem::transmute(&mut self.internal.edit) }
    }

    pub fn chart_mut(&mut self) -> &mut StyleChart {
        unsafe { ::std::mem::transmute(&mut self.internal.chart) }
    }

    pub fn scroll_h_mut(&mut self) -> &mut StyleScrollbar {
        unsafe { ::std::mem::transmute(&mut self.internal.scrollh) }
    }

    pub fn scroll_v_mut(&mut self) -> &mut StyleScrollbar {
        unsafe { ::std::mem::transmute(&mut self.internal.scrollv) }
    }

    pub fn tab_mut(&mut self) -> &mut StyleTab {
        unsafe { ::std::mem::transmute(&mut self.internal.tab) }
    }

    pub fn combo_mut(&mut self) -> &mut StyleCombo {
        unsafe { ::std::mem::transmute(&mut self.internal.combo) }
    }

    // ===== getters =====

    pub fn window(&self) -> &StyleWindow {
        unsafe { ::std::mem::transmute(&self.internal.window) }
    }

    pub fn font(&self) -> &UserFont {
        unsafe { ::std::mem::transmute(self.internal.font) }
    }

    pub fn cursors(&self) -> &CursorMap {
        unsafe { ::std::mem::transmute(&self.internal.cursors) }
    }

    pub fn cursor_active(&self) -> &Cursor {
        unsafe { ::std::mem::transmute(&self.internal.cursor_active) }
    }

    pub fn cursor_visible(&self) -> bool {
        self.internal.cursor_visible > 0
    }

    pub fn text(&self) -> &StyleText {
        &self.internal.text
    }

    pub fn button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.button) }
    }

    pub fn contextual_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.contextual_button) }
    }

    pub fn menu_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.menu_button) }
    }

    pub fn option(&self) -> &StyleToggle {
        unsafe { ::std::mem::transmute(&self.internal.option) }
    }

    pub fn checkbox(&self) -> &StyleToggle {
        unsafe { ::std::mem::transmute(&self.internal.checkbox) }
    }

    pub fn selectable(&self) -> &StyleSelectable {
        unsafe { ::std::mem::transmute(&self.internal.selectable) }
    }

    pub fn slider(&self) -> &StyleSlider {
        unsafe { ::std::mem::transmute(&self.internal.slider) }
    }

    pub fn progress(&self) -> &StyleProgress {
        unsafe { ::std::mem::transmute(&self.internal.progress) }
    }

    pub fn property(&self) -> &StyleProperty {
        unsafe { ::std::mem::transmute(&self.internal.property) }
    }

    pub fn edit(&self) -> &StyleEdit {
        unsafe { ::std::mem::transmute(&self.internal.edit) }
    }

    pub fn chart(&self) -> &StyleChart {
        unsafe { ::std::mem::transmute(&self.internal.chart) }
    }

    pub fn scroll_h(&self) -> &StyleScrollbar {
        unsafe { ::std::mem::transmute(&self.internal.scrollh) }
    }

    pub fn scroll_v(&self) -> &StyleScrollbar {
        unsafe { ::std::mem::transmute(&self.internal.scrollv) }
    }

    pub fn tab(&self) -> &StyleTab {
        unsafe { ::std::mem::transmute(&self.internal.tab) }
    }

    pub fn combo(&self) -> &StyleCombo {
        unsafe { ::std::mem::transmute(&self.internal.combo) }
    }
}

// =====================================================================

wrapper_type!(StyleCombo, nk_style_combo);

impl StyleCombo {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn label_normal(&self) -> &Color {
        &self.internal.label_normal
    }

    pub fn label_hover(&self) -> &Color {
        &self.internal.label_hover
    }

    pub fn label_active(&self) -> &Color {
        &self.internal.label_active
    }

    pub fn symbol_normal(&self) -> &Color {
        &self.internal.symbol_normal
    }

    pub fn symbol_hover(&self) -> &Color {
        &self.internal.symbol_hover
    }

    pub fn symbol_active(&self) -> &Color {
        &self.internal.symbol_active
    }

    pub fn button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.button) }
    }

    pub fn sym_normal(&self) -> &SymbolType {
        (&self.internal.sym_normal).into()
    }

    pub fn sym_hover(&self) -> &SymbolType {
        (&self.internal.sym_hover).into()
    }

    pub fn sym_active(&self) -> &SymbolType {
        (&self.internal.sym_active).into()
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn content_padding(&self) -> &Vec2 {
        &self.internal.content_padding
    }

    pub fn button_padding(&self) -> &Vec2 {
        &self.internal.button_padding
    }

    pub fn spacing(&self) -> &Vec2 {
        &self.internal.spacing
    }

    // ===== setters ======

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: StyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_label_normal(&mut self, c: Color) {
        self.internal.label_normal = c
    }

    pub fn set_label_hover(&mut self, c: Color) {
        self.internal.label_hover = c
    }

    pub fn set_label_active(&mut self, c: Color) {
        self.internal.label_active = c
    }

    pub fn set_symbol_normal(&mut self, c: Color) {
        self.internal.symbol_normal = c
    }

    pub fn set_symbol_hover(&mut self, c: Color) {
        self.internal.symbol_hover = c
    }

    pub fn set_symbol_active(&mut self, c: Color) {
        self.internal.symbol_active = c
    }

    pub fn set_button(&mut self, s: StyleButton) {
        self.internal.button = s.internal
    }

    pub fn set_sym_normal(&mut self, t: SymbolType) {
        self.internal.sym_normal = t.into()
    }

    pub fn set_sym_hover(&mut self, t: SymbolType) {
        self.internal.sym_hover = t.into()
    }

    pub fn set_sym_active(&mut self, t: SymbolType) {
        self.internal.sym_active = t.into()
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_content_padding(&mut self, v: Vec2) {
        self.internal.content_padding = v
    }

    pub fn set_button_padding(&mut self, v: Vec2) {
        self.internal.button_padding = v
    }

    pub fn set_spacing(&mut self, v: Vec2) {
        self.internal.spacing = v
    }
}

// =====================================================================

wrapper_type!(StyleTab, nk_style_tab);

impl StyleTab {
    // ===== getters =====

    pub fn background(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.background) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn text(&self) -> &Color {
        &self.internal.text
    }

    pub fn tab_maximize_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.tab_maximize_button) }
    }

    pub fn tab_minimize_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.tab_minimize_button) }
    }

    pub fn node_maximize_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.node_maximize_button) }
    }

    pub fn node_minimize_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.node_minimize_button) }
    }

    pub fn sym_minimize(&self) -> &SymbolType {
        (&self.internal.sym_minimize).into()
    }

    pub fn sym_maximize(&self) -> &SymbolType {
        (&self.internal.sym_maximize).into()
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

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn spacing(&self) -> &Vec2 {
        &self.internal.spacing
    }

    // ===== setters =====

    pub fn set_background(&mut self, i: StyleItem) {
        self.internal.background = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_text(&mut self, c: Color) {
        self.internal.text = c
    }

    pub fn set_tab_maximize_button(&mut self, s: StyleButton) {
        self.internal.tab_maximize_button = s.internal
    }

    pub fn set_tab_minimize_button(&mut self, s: StyleButton) {
        self.internal.tab_minimize_button = s.internal
    }

    pub fn set_node_maximize_button(&mut self, s: StyleButton) {
        self.internal.node_maximize_button = s.internal
    }

    pub fn set_node_minimize_button(&mut self, s: StyleButton) {
        self.internal.node_minimize_button = s.internal
    }

    pub fn set_sym_minimize(&mut self, t: SymbolType) {
        self.internal.sym_minimize = t.into()
    }

    pub fn set_sym_maximize(&mut self, t: SymbolType) {
        self.internal.sym_maximize = t.into()
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

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }

    pub fn set_spacing(&mut self, v: Vec2) {
        self.internal.spacing = v
    }
}

// =====================================================================

wrapper_type!(StyleScrollbar, nk_style_scrollbar);

impl StyleScrollbar {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn cursor_normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_normal) }
    }

    pub fn cursor_hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_hover) }
    }

    pub fn cursor_active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_active) }
    }

    pub fn cursor_border_color(&self) -> &Color {
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

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn show_buttons(&self) -> bool {
        self.internal.show_buttons > 0
    }

    pub fn inc_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.inc_button) }
    }

    pub fn dec_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.dec_button) }
    }

    pub fn inc_symbol(&self) -> &SymbolType {
        (&self.internal.inc_symbol).into()
    }

    pub fn dec_symbol(&self) -> &SymbolType {
        (&self.internal.dec_symbol).into()
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: StyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_cursor_normal(&mut self, i: StyleItem) {
        self.internal.cursor_normal = i.internal;
    }

    pub fn set_cursor_hover(&mut self, i: StyleItem) {
        self.internal.cursor_hover = i.internal;
    }

    pub fn set_cursor_active(&mut self, i: StyleItem) {
        self.internal.cursor_active = i.internal;
    }

    pub fn set_cursor_border_color(&mut self, c: Color) {
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

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }

    pub fn set_show_buttons(&mut self, show: bool) {
        self.internal.show_buttons = if show { 1 } else { 0 }
    }

    pub fn set_inc_button(&mut self, s: StyleButton) {
        self.internal.inc_button = s.internal
    }

    pub fn set_dec_button(&mut self, s: StyleButton) {
        self.internal.dec_button = s.internal
    }

    pub fn set_inc_symbol(&mut self, t: SymbolType) {
        self.internal.inc_symbol = t.into()
    }

    pub fn set_dec_symbol(&mut self, t: SymbolType) {
        self.internal.dec_symbol = t.into()
    }
}

// =====================================================================

wrapper_type!(StyleChart, nk_style_chart);

impl StyleChart {
    // ===== getters =====

    pub fn background(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.background) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn selected_color(&self) -> &Color {
        &self.internal.selected_color
    }

    pub fn color(&self) -> &Color {
        &self.internal.color
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    // ===== setters =====

    pub fn set_background(&mut self, i: StyleItem) {
        self.internal.background = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_selected_color(&mut self, c: Color) {
        self.internal.selected_color = c
    }

    pub fn set_color(&mut self, c: Color) {
        self.internal.color = c
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }
}

// =====================================================================

wrapper_type!(StyleEdit, nk_style_edit);

impl StyleEdit {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn scrollbar(&self) -> &StyleScrollbar {
        unsafe { ::std::mem::transmute(&self.internal.scrollbar) }
    }

    pub fn cursor_normal(&self) -> &Color {
        &self.internal.cursor_normal
    }

    pub fn cursor_hover(&self) -> &Color {
        &self.internal.cursor_hover
    }

    pub fn cursor_text_normal(&self) -> &Color {
        &self.internal.cursor_text_normal
    }

    pub fn cursor_text_hover(&self) -> &Color {
        &self.internal.cursor_text_hover
    }

    pub fn text_normal(&self) -> &Color {
        &self.internal.text_normal
    }

    pub fn text_hover(&self) -> &Color {
        &self.internal.text_hover
    }

    pub fn text_active(&self) -> &Color {
        &self.internal.text_active
    }

    pub fn selected_normal(&self) -> &Color {
        &self.internal.selected_normal
    }

    pub fn selected_hover(&self) -> &Color {
        &self.internal.selected_hover
    }

    pub fn selected_text_normal(&self) -> &Color {
        &self.internal.selected_text_normal
    }

    pub fn selected_text_hover(&self) -> &Color {
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

    pub fn scrollbar_size(&self) -> &Vec2 {
        &self.internal.scrollbar_size
    }

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn row_padding(&self) -> f32 {
        self.internal.row_padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: StyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_cursor_normal(&mut self, i: Color) {
        self.internal.cursor_normal = i;
    }

    pub fn set_cursor_hover(&mut self, i: Color) {
        self.internal.cursor_hover = i;
    }

    pub fn set_cursor_text_normal(&mut self, i: Color) {
        self.internal.cursor_text_normal = i;
    }

    pub fn set_cursor_text_hover(&mut self, i: Color) {
        self.internal.cursor_text_hover = i;
    }

    pub fn set_text_normal(&mut self, i: Color) {
        self.internal.text_normal = i;
    }

    pub fn set_text_hover(&mut self, i: Color) {
        self.internal.text_hover = i;
    }

    pub fn set_text_active(&mut self, i: Color) {
        self.internal.text_active = i;
    }

    pub fn set_selected_normal(&mut self, i: Color) {
        self.internal.selected_normal = i;
    }

    pub fn set_selected_hover(&mut self, i: Color) {
        self.internal.selected_hover = i;
    }

    pub fn set_selected_text_normal(&mut self, i: Color) {
        self.internal.selected_text_normal = i;
    }

    pub fn set_selected_text_hover(&mut self, i: Color) {
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

    pub fn set_scrollbar_size(&mut self, v: Vec2) {
        self.internal.scrollbar_size = v
    }

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }

    pub fn set_row_padding(&mut self, v: f32) {
        self.internal.row_padding = v
    }
}

// =====================================================================

wrapper_type!(StyleProperty, nk_style_property);

impl StyleProperty {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn label_normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.label_normal) }
    }

    pub fn label_hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.label_hover) }
    }

    pub fn label_active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.label_active) }
    }

    pub fn sym_left(&self) -> &SymbolType {
        (&self.internal.sym_left).into()
    }

    pub fn sym_right(&self) -> &SymbolType {
        (&self.internal.sym_right).into()
    }

    pub fn border(&self) -> f32 {
        self.internal.border
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn edit(&self) -> &StyleEdit {
        unsafe { ::std::mem::transmute(&self.internal.edit) }
    }

    pub fn inc_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.inc_button) }
    }

    pub fn dec_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.dec_button) }
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: StyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_label_normal(&mut self, c: Color) {
        self.internal.label_normal = c
    }

    pub fn set_label_hover(&mut self, c: Color) {
        self.internal.label_hover = c
    }

    pub fn set_label_active(&mut self, c: Color) {
        self.internal.label_active = c
    }

    pub fn set_sym_left(&mut self, t: SymbolType) {
        self.internal.sym_left = t.into()
    }

    pub fn set_sym_right(&mut self, t: SymbolType) {
        self.internal.sym_right = t.into()
    }

    pub fn set_border(&mut self, v: f32) {
        self.internal.border = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }

    pub fn set_inc_button(&mut self, s: StyleButton) {
        self.internal.inc_button = s.internal
    }

    pub fn set_dec_button(&mut self, s: StyleButton) {
        self.internal.dec_button = s.internal
    }
}

// =====================================================================

wrapper_type!(StyleProgress, nk_style_progress);

impl StyleProgress {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn cursor_normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_normal) }
    }

    pub fn cursor_hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_hover) }
    }

    pub fn cursor_active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_active) }
    }

    pub fn cursor_border_color(&self) -> &Color {
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

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: StyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_cursor_normal(&mut self, i: StyleItem) {
        self.internal.cursor_normal = i.internal;
    }

    pub fn set_cursor_hover(&mut self, i: StyleItem) {
        self.internal.cursor_hover = i.internal;
    }

    pub fn set_cursor_active(&mut self, i: StyleItem) {
        self.internal.cursor_active = i.internal;
    }

    pub fn set_cursor_border_color(&mut self, c: Color) {
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

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }
}

// =====================================================================

wrapper_type!(StyleSlider, nk_style_slider);

impl StyleSlider {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn bar_normal(&self) -> &Color {
        &self.internal.bar_normal
    }

    pub fn bar_hover(&self) -> &Color {
        &self.internal.bar_hover
    }

    pub fn bar_active(&self) -> &Color {
        &self.internal.bar_active
    }

    pub fn bar_filled(&self) -> &Color {
        &self.internal.bar_filled
    }

    pub fn cursor_normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_normal) }
    }

    pub fn cursor_hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_hover) }
    }

    pub fn cursor_active(&self) -> &StyleItem {
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

    pub fn spacing(&self) -> &Vec2 {
        &self.internal.spacing
    }

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn cursor_size(&self) -> &Vec2 {
        &self.internal.cursor_size
    }

    pub fn show_buttons(&self) -> bool {
        self.internal.show_buttons > 0
    }

    pub fn inc_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.inc_button) }
    }

    pub fn dec_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.dec_button) }
    }

    pub fn inc_symbol(&self) -> &SymbolType {
        (&self.internal.inc_symbol).into()
    }

    pub fn dec_symbol(&self) -> &SymbolType {
        (&self.internal.dec_symbol).into()
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: StyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_bar_normal(&mut self, c: Color) {
        self.internal.bar_normal = c
    }

    pub fn set_bar_hover(&mut self, c: Color) {
        self.internal.bar_hover = c
    }

    pub fn set_bar_active(&mut self, c: Color) {
        self.internal.bar_active = c
    }

    pub fn set_bar_filled(&mut self, c: Color) {
        self.internal.bar_filled = c
    }

    pub fn set_cursor_normal(&mut self, i: StyleItem) {
        self.internal.cursor_normal = i.internal;
    }

    pub fn set_cursor_hover(&mut self, i: StyleItem) {
        self.internal.cursor_hover = i.internal;
    }

    pub fn set_cursor_active(&mut self, i: StyleItem) {
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

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }

    pub fn set_spacing(&mut self, v: Vec2) {
        self.internal.spacing = v
    }

    pub fn set_cursor_size(&mut self, v: Vec2) {
        self.internal.cursor_size = v
    }

    pub fn set_show_buttons(&mut self, show: bool) {
        self.internal.show_buttons = if show { 1 } else { 0 }
    }

    pub fn set_inc_button(&mut self, s: StyleButton) {
        self.internal.inc_button = s.internal
    }

    pub fn set_dec_button(&mut self, s: StyleButton) {
        self.internal.dec_button = s.internal
    }

    pub fn set_inc_symbol(&mut self, t: SymbolType) {
        self.internal.inc_symbol = t.into()
    }

    pub fn set_dec_symbol(&mut self, t: SymbolType) {
        self.internal.dec_symbol = t.into()
    }
}

// =====================================================================

wrapper_type!(StyleSelectable, nk_style_selectable);

impl StyleSelectable {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn pressed(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.pressed) }
    }

    pub fn normal_active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal_active) }
    }

    pub fn hover_active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover_active) }
    }

    pub fn pressed_active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.pressed_active) }
    }

    pub fn text_normal(&self) -> &Color {
        &self.internal.text_normal
    }

    pub fn text_hover(&self) -> &Color {
        &self.internal.text_hover
    }

    pub fn text_pressed(&self) -> &Color {
        &self.internal.text_pressed
    }

    pub fn text_normal_active(&self) -> &Color {
        &self.internal.text_normal_active
    }

    pub fn text_hover_active(&self) -> &Color {
        &self.internal.text_hover_active
    }

    pub fn text_pressed_active(&self) -> &Color {
        &self.internal.text_pressed_active
    }

    pub fn text_background(&self) -> &Color {
        &self.internal.text_background
    }

    pub fn text_alignment(&self) -> u32 {
        self.internal.text_alignment
    }

    pub fn rounding(&self) -> f32 {
        self.internal.rounding
    }

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn touch_padding(&self) -> &Vec2 {
        &self.internal.touch_padding
    }

    pub fn image_padding(&self) -> &Vec2 {
        &self.internal.image_padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_pressed(&mut self, i: StyleItem) {
        self.internal.pressed = i.internal;
    }

    pub fn set_normal_active(&mut self, i: StyleItem) {
        self.internal.normal_active = i.internal;
    }

    pub fn set_hover_active(&mut self, i: StyleItem) {
        self.internal.hover_active = i.internal;
    }

    pub fn set_pressed_active(&mut self, i: StyleItem) {
        self.internal.pressed_active = i.internal;
    }

    pub fn set_text_normal(&mut self, c: Color) {
        self.internal.text_normal = c
    }

    pub fn set_text_hover(&mut self, c: Color) {
        self.internal.text_hover = c
    }

    pub fn set_text_pressed(&mut self, c: Color) {
        self.internal.text_pressed = c
    }

    pub fn set_text_normal_active(&mut self, c: Color) {
        self.internal.text_normal_active = c
    }

    pub fn set_text_hover_active(&mut self, c: Color) {
        self.internal.text_hover_active = c
    }

    pub fn set_text_pressed_active(&mut self, c: Color) {
        self.internal.text_pressed_active = c
    }

    pub fn set_text_background(&mut self, c: Color) {
        self.internal.text_background = c
    }

    pub fn set_text_alignment(&mut self, v: u32) {
        self.internal.text_alignment = v
    }

    pub fn set_rounding(&mut self, v: f32) {
        self.internal.rounding = v
    }

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }

    pub fn set_touch_padding(&mut self, v: Vec2) {
        self.internal.touch_padding = v
    }

    pub fn set_image_padding(&mut self, v: Vec2) {
        self.internal.image_padding = v
    }
}

// =====================================================================

wrapper_type!(StyleButton, nk_style_button);

impl StyleButton {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn text_background(&self) -> &Color {
        &self.internal.text_background
    }

    pub fn text_normal(&self) -> &Color {
        &self.internal.text_normal
    }

    pub fn text_hover(&self) -> &Color {
        &self.internal.text_hover
    }

    pub fn text_active(&self) -> &Color {
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

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn touch_padding(&self) -> &Vec2 {
        &self.internal.touch_padding
    }

    pub fn image_padding(&self) -> &Vec2 {
        &self.internal.image_padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: StyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_text_background(&mut self, c: Color) {
        self.internal.text_background = c
    }

    pub fn set_text_normal(&mut self, c: Color) {
        self.internal.text_normal = c
    }

    pub fn set_text_hover(&mut self, c: Color) {
        self.internal.text_hover = c
    }

    pub fn set_text_active(&mut self, c: Color) {
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

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }

    pub fn set_touch_padding(&mut self, v: Vec2) {
        self.internal.touch_padding = v
    }

    pub fn set_image_padding(&mut self, v: Vec2) {
        self.internal.image_padding = v
    }
}

// =====================================================================

wrapper_type!(StyleToggle, nk_style_toggle);

impl StyleToggle {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn cursor_normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_normal) }
    }

    pub fn cursor_hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.cursor_hover) }
    }

    pub fn text_normal(&self) -> &Color {
        &self.internal.text_normal
    }

    pub fn text_hover(&self) -> &Color {
        &self.internal.text_hover
    }

    pub fn text_active(&self) -> &Color {
        &self.internal.text_active
    }

    pub fn text_background(&self) -> &Color {
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

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn touch_padding(&self) -> &Vec2 {
        &self.internal.touch_padding
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: StyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_border_color(&mut self, c: Color) {
        self.internal.border_color = c
    }

    pub fn set_cursor_normal(&mut self, i: StyleItem) {
        self.internal.cursor_normal = i.internal;
    }

    pub fn set_cursor_hover(&mut self, i: StyleItem) {
        self.internal.cursor_hover = i.internal;
    }

    pub fn set_text_background(&mut self, c: Color) {
        self.internal.text_background = c
    }

    pub fn set_text_normal(&mut self, c: Color) {
        self.internal.text_normal = c
    }

    pub fn set_text_hover(&mut self, c: Color) {
        self.internal.text_hover = c
    }

    pub fn set_text_active(&mut self, c: Color) {
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

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }

    pub fn set_touch_padding(&mut self, v: Vec2) {
        self.internal.touch_padding = v
    }
}

// =====================================================================

wrapper_type!(StyleWindowHeader, nk_style_window_header);

impl StyleWindowHeader {
    // ===== getters =====

    pub fn normal(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.normal) }
    }

    pub fn hover(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.hover) }
    }

    pub fn active(&self) -> &StyleItem {
        unsafe { ::std::mem::transmute(&self.internal.active) }
    }

    pub fn close_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.close_button) }
    }

    pub fn minimize_button(&self) -> &StyleButton {
        unsafe { ::std::mem::transmute(&self.internal.minimize_button) }
    }

    pub fn close_symbol(&self) -> &SymbolType {
        (&self.internal.close_symbol).into()
    }

    pub fn minimize_symbol(&self) -> &SymbolType {
        (&self.internal.minimize_symbol).into()
    }

    pub fn maximize_symbol(&self) -> &SymbolType {
        (&self.internal.maximize_symbol).into()
    }

    pub fn label_normal(&self) -> &Color {
        &self.internal.label_normal
    }

    pub fn label_hover(&self) -> &Color {
        &self.internal.label_hover
    }

    pub fn label_active(&self) -> &Color {
        &self.internal.label_active
    }

    pub fn align(&self) -> &StyleHeaderAlign {
        &self.internal.align
    }

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn label_padding(&self) -> &Vec2 {
        &self.internal.label_padding
    }

    pub fn spacing(&self) -> &Vec2 {
        &self.internal.spacing
    }

    // ===== setters =====

    pub fn set_normal(&mut self, i: StyleItem) {
        self.internal.normal = i.internal;
    }

    pub fn set_hover(&mut self, i: StyleItem) {
        self.internal.hover = i.internal;
    }

    pub fn set_active(&mut self, i: StyleItem) {
        self.internal.active = i.internal;
    }

    pub fn set_close_symbol(&mut self, t: SymbolType) {
        self.internal.close_symbol = t.into()
    }

    pub fn set_minimize_symbol(&mut self, t: SymbolType) {
        self.internal.minimize_symbol = t.into()
    }

    pub fn set_maximize_symbol(&mut self, t: SymbolType) {
        self.internal.maximize_symbol = t.into()
    }

    pub fn set_label_normal(&mut self, c: Color) {
        self.internal.label_normal = c
    }

    pub fn set_label_hover(&mut self, c: Color) {
        self.internal.label_hover = c
    }

    pub fn set_label_active(&mut self, c: Color) {
        self.internal.label_active = c
    }

    pub fn set_align(&mut self, c: StyleHeaderAlign) {
        self.internal.align = c
    }

    pub fn set_padding(&mut self, v: Vec2) {
        self.internal.padding = v
    }

    pub fn set_label_padding(&mut self, v: Vec2) {
        self.internal.label_padding = v
    }

    pub fn set_spacing(&mut self, v: Vec2) {
        self.internal.spacing = v
    }
}

// =====================================================================

wrapper_type!(StyleWindow, nk_style_window);

impl StyleWindow {
    // ===== getters =====

    pub fn header(&self) -> &StyleWindowHeader {
        unsafe { ::std::mem::transmute(&self.internal.header) }
    }

    pub fn fixed_background(&self) -> StyleItem {
        StyleItem { internal: self.internal.fixed_background }
    }

    pub fn background(&self) -> &Color {
        &self.internal.background
    }

    pub fn border_color(&self) -> &Color {
        &self.internal.border_color
    }

    pub fn popup_border_color(&self) -> &Color {
        &self.internal.popup_border_color
    }

    pub fn combo_border_color(&self) -> &Color {
        &self.internal.combo_border_color
    }

    pub fn contextual_border_color(&self) -> &Color {
        &self.internal.contextual_border_color
    }

    pub fn menu_border_color(&self) -> &Color {
        &self.internal.menu_border_color
    }

    pub fn group_border_color(&self) -> &Color {
        &self.internal.group_border_color
    }

    pub fn tooltip_border_color(&self) -> &Color {
        &self.internal.tooltip_border_color
    }

    pub fn scaler(&self) -> &StyleItem {
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

    pub fn spacing(&self) -> &Vec2 {
        &self.internal.spacing
    }

    pub fn scrollbar_size(&self) -> &Vec2 {
        &self.internal.scrollbar_size
    }

    pub fn min_size(&self) -> &Vec2 {
        &self.internal.min_size
    }

    pub fn padding(&self) -> &Vec2 {
        &self.internal.padding
    }

    pub fn group_padding(&self) -> &Vec2 {
        &self.internal.group_padding
    }

    pub fn popup_padding(&self) -> &Vec2 {
        &self.internal.popup_padding
    }

    pub fn combo_padding(&self) -> &Vec2 {
        &self.internal.combo_padding
    }

    pub fn contextual_padding(&self) -> &Vec2 {
        &self.internal.contextual_padding
    }

    pub fn menu_padding(&self) -> &Vec2 {
        &self.internal.menu_padding
    }

    pub fn tooltip_padding(&self) -> &Vec2 {
        &self.internal.tooltip_padding
    }

    // ===== setters =====

    pub fn set_fixed_background(&mut self, item: StyleItem) {
        self.internal.fixed_background = item.internal;
    }

    pub fn set_background(&mut self, color: Color) {
        self.internal.background = color;
    }

    pub fn set_border_color(&mut self, color: Color) {
        self.internal.border_color = color;
    }

    pub fn set_popup_border_color(&mut self, color: Color) {
        self.internal.popup_border_color = color;
    }

    pub fn set_combo_border_color(&mut self, color: Color) {
        self.internal.combo_border_color = color;
    }

    pub fn set_contextual_border_color(&mut self, color: Color) {
        self.internal.contextual_border_color = color;
    }

    pub fn set_menu_border_color(&mut self, color: Color) {
        self.internal.menu_border_color = color;
    }

    pub fn set_group_border_color(&mut self, color: Color) {
        self.internal.group_border_color = color;
    }

    pub fn set_tooltip_border_color(&mut self, color: Color) {
        self.internal.tooltip_border_color = color;
    }

    pub fn set_scaler(&mut self, i: StyleItem) {
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

    pub fn set_spacing(&mut self, spacing: Vec2) {
        self.internal.spacing = spacing;
    }

    pub fn set_scrollbar_size(&mut self, s: Vec2) {
        self.internal.scrollbar_size = s;
    }

    pub fn set_min_size(&mut self, s: Vec2) {
        self.internal.min_size = s;
    }

    pub fn set_padding(&mut self, padding: Vec2) {
        self.internal.padding = padding;
    }

    pub fn set_group_padding(&mut self, padding: Vec2) {
        self.internal.group_padding = padding;
    }

    pub fn set_popup_padding(&mut self, padding: Vec2) {
        self.internal.popup_padding = padding;
    }

    pub fn set_combo_padding(&mut self, padding: Vec2) {
        self.internal.combo_padding = padding;
    }

    pub fn set_contextual_padding(&mut self, padding: Vec2) {
        self.internal.contextual_padding = padding;
    }

    pub fn set_menu_padding(&mut self, padding: Vec2) {
        self.internal.menu_padding = padding;
    }

    pub fn set_tooltip_padding(&mut self, padding: Vec2) {
        self.internal.tooltip_padding = padding;
    }
}

// =====================================================================

wrapper_type!(DrawList, nk_draw_list);

impl DrawList {
    pub fn init(&mut self) {
        unsafe {
            nk_draw_list_init(&mut self.internal);
        }
    }

    pub fn setup(&mut self, config: &ConvertConfig, cmds: &mut Buffer, vertices: &mut Buffer, elements: &mut Buffer, line_aa: AntiAliasing, shape_aa: AntiAliasing) {
        unsafe {
            nk_draw_list_setup(
                &mut self.internal,
                &config.internal as *const nk_convert_config,
                &mut cmds.internal as *mut nk_buffer,
                &mut vertices.internal as *mut nk_buffer,
                &mut elements.internal as *mut nk_buffer,
                line_aa.into(),
                shape_aa.into(),
            )
        }
    }

    pub fn begin(&self, buf: &Buffer) -> &DrawCommand {
        unsafe { ::std::mem::transmute(nk__draw_list_begin(&self.internal, &buf.internal)) }
    }

    pub fn next(&self, buf: &Buffer, prev: &DrawCommand) -> &DrawCommand {
        unsafe { ::std::mem::transmute(nk__draw_list_next(&prev.internal, &buf.internal, &self.internal)) }
    }

    pub fn path_clear(&mut self) {
        unsafe {
            nk_draw_list_path_clear(&mut self.internal);
        }
    }

    pub fn path_line_to(&mut self, pos: Vec2) {
        unsafe {
            nk_draw_list_path_line_to(&mut self.internal, pos);
        }
    }

    pub fn path_arc_to_fast(&mut self, center: Vec2, radius: f32, a_min: i32, a_max: i32) {
        unsafe {
            nk_draw_list_path_arc_to_fast(&mut self.internal, center, radius, a_min, a_max);
        }
    }

    pub fn path_arc_to(&mut self, center: Vec2, radius: f32, a_min: f32, a_max: f32, segments: u32) {
        unsafe {
            nk_draw_list_path_arc_to(&mut self.internal, center, radius, a_min, a_max, segments);
        }
    }

    pub fn path_rect_to(&mut self, a: Vec2, b: Vec2, rounding: f32) {
        unsafe {
            nk_draw_list_path_rect_to(&mut self.internal, a, b, rounding);
        }
    }

    pub fn path_curve_to(&mut self, p2: Vec2, p3: Vec2, p4: Vec2, num_segments: u32) {
        unsafe { nk_draw_list_path_curve_to(&mut self.internal, p2, p3, p4, num_segments) }
    }

    pub fn path_fill(&mut self, col: Color) {
        unsafe {
            nk_draw_list_path_fill(&mut self.internal, col);
        }
    }

    pub fn path_stroke(&mut self, arg2: Color, closed: DrawListStroke, thickness: f32) {
        unsafe {
            nk_draw_list_path_stroke(&mut self.internal, arg2, closed, thickness);
        }
    }

    pub fn stroke_line(&mut self, a: Vec2, b: Vec2, arg2: Color, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_line(&mut self.internal, a, b, arg2, thickness);
        }
    }

    pub fn stroke_rect(&mut self, rect: Rect, arg2: Color, rounding: f32, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_rect(&mut self.internal, rect, arg2, rounding, thickness);
        }
    }

    pub fn stroke_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, arg2: Color, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_triangle(&mut self.internal, a, b, c, arg2, thickness);
        }
    }

    pub fn stroke_circle(&mut self, center: Vec2, radius: f32, arg2: Color, segs: u32, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_circle(&mut self.internal, center, radius, arg2, segs, thickness);
        }
    }

    pub fn stroke_curve(&mut self, p0: Vec2, cp0: Vec2, cp1: Vec2, p1: Vec2, arg2: Color, segments: u32, thickness: f32) {
        unsafe {
            nk_draw_list_stroke_curve(&mut self.internal, p0, cp0, cp1, p1, arg2, segments, thickness);
        }
    }

    pub fn stroke_poly_line(&mut self, points: &[Vec2], arg2: Color, arg3: DrawListStroke, thickness: f32, aa: AntiAliasing) {
        unsafe {
            nk_draw_list_stroke_poly_line(&mut self.internal, points.as_ptr(), points.len() as u32, arg2, arg3, thickness, aa.into());
        }
    }

    pub fn fill_rect(&mut self, rect: Rect, arg2: Color, rounding: f32) {
        unsafe {
            nk_draw_list_fill_rect(&mut self.internal, rect, arg2, rounding);
        }
    }

    pub fn fill_rect_multi_color(&mut self, rect: Rect, left: Color, top: Color, right: Color, bottom: Color) {
        unsafe {
            nk_draw_list_fill_rect_multi_color(&mut self.internal, rect, left, top, right, bottom);
        }
    }

    pub fn fill_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, arg2: Color) {
        unsafe {
            nk_draw_list_fill_triangle(&mut self.internal, a, b, c, arg2);
        }
    }

    pub fn fill_circle(&mut self, center: Vec2, radius: f32, col: Color, segs: u32) {
        unsafe {
            nk_draw_list_fill_circle(&mut self.internal, center, radius, col, segs);
        }
    }

    pub fn fill_poly_convex(&mut self, points: &[Vec2], arg2: Color, arg3: AntiAliasing) {
        unsafe {
            nk_draw_list_fill_poly_convex(&mut self.internal, points.as_ptr(), points.len() as u32, arg2, arg3.into());
        }
    }

    pub fn add_image(&mut self, texture: Image, rect: Rect, arg2: Color) {
        unsafe {
            nk_draw_list_add_image(&mut self.internal, texture.internal, rect, arg2);
        }
    }

    pub fn add_text(&mut self, arg2: &UserFont, arg3: Rect, text: String, font_height: f32, arg4: Color) {
        unsafe {
            nk_draw_list_add_text(&mut self.internal, &arg2.internal, arg3, text.as_ptr(), text.bytes.len() as i32, font_height, arg4);
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
pub struct ColorMap {
    internal: [nk_color; 28],
}

impl Default for ColorMap {
    fn default() -> Self {
        ColorMap { internal: [nk_color::default(); 28] }
    }
}

impl ColorMap {
    pub fn set(&mut self, target: StyleColor, color: Color) {
        self.internal[target as usize] = color;
    }
}

// ==================================================================================

pub struct CursorMap<'a> {
    internal: [Option<&'a Cursor>; 7],
}

impl<'a> Default for CursorMap<'a> {
    fn default() -> Self {
        unsafe {
            let mut map = CursorMap { internal: [::std::mem::zeroed(); 7] };

            for i in &mut map.internal {
                ::std::ptr::write(i, None);
            }

            map
        }
    }
}

impl<'a> CursorMap<'a> {
    pub fn set(&mut self, target: StyleCursor, res: Option<&'a Cursor>) {
        self.internal[target as usize] = res;
    }
}

// ==================================================================================

wrapper_type!(Cursor, nk_cursor);

impl Cursor {
    pub fn img(&self) -> &Image {
        unsafe { ::std::mem::transmute(&self.internal.img) }
    }
    pub fn size(&self) -> &Vec2 {
        &self.internal.size
    }
    pub fn offset(&self) -> &Vec2 {
        &self.internal.offset
    }
}

// ==================================================================================

wrapper_type!(Allocator, nk_allocator);

impl Allocator {
    #[cfg(feature = "rust_allocator")]
    pub fn new_heap() -> Allocator {
        let mut a = Allocator::default();

        a.internal.alloc = Some(alloc_heap::alloc);
        a.internal.free = Some(alloc_heap::free);
        a.internal.userdata = nk_handle::default();
        a.internal.userdata.ptr = ::std::ptr::null_mut();
        a
    }

    pub fn new_vec() -> Allocator {
        let mut a = Allocator::default();

        a.internal.alloc = Some(alloc_vec::alloc);
        a.internal.free = Some(alloc_vec::free);
        a.internal.userdata = nk_handle::default();
        a.internal.userdata.ptr = ::std::ptr::null_mut();
        a
    }
}

// ============================================================================================

wrapper_type!(ConvertConfig, nk_convert_config);

impl ConvertConfig {
    pub fn set_global_alpha(&mut self, val: f32) {
        self.internal.global_alpha = val;
    }
    pub fn set_line_aa(&mut self, val: AntiAliasing) {
        self.internal.line_AA = val.into();
    }
    pub fn set_shape_aa(&mut self, val: AntiAliasing) {
        self.internal.shape_AA = val.into();
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
    pub fn set_null(&mut self, val: DrawNullTexture) {
        self.internal.null = val.internal;
    }
    pub fn set_vertex_layout(&mut self, val: &DrawVertexLayoutElements) {
        self.internal.vertex_layout = &val.arr.as_slice()[0];
    }
    pub fn set_vertex_size(&mut self, val: Size) {
        self.internal.vertex_size = val;
    }
    // pub fn set_vertex_alignment(&mut self, val: usize) {
    // self.internal.vertex_alignment = val;
    // }
}

// ============================================================================================

#[derive(Debug, Clone)]
pub struct DrawVertexLayoutElements {
    arr: Vec<nk_draw_vertex_layout_element>,
}

impl DrawVertexLayoutElements {
    pub fn new(var: &[(DrawVertexLayoutAttribute, DrawVertexLayoutFormat, Size)]) -> DrawVertexLayoutElements {
        DrawVertexLayoutElements {
            arr: var.iter().map(|&(a, f, o)| nk_draw_vertex_layout_element { attribute: a.into(), format: f.into(), offset: o }).collect::<Vec<_>>(),
        }
    }
}

// =============================================================================================

wrapper_type!(StyleItem, nk_style_item);

impl StyleItem {
    pub fn image(img: Image) -> StyleItem {
        unsafe { StyleItem { internal: nk_style_item_image(img.internal) } }
    }

    pub fn color(col: Color) -> StyleItem {
        unsafe { StyleItem { internal: nk_style_item_color(col) } }
    }

    pub fn hide() -> StyleItem {
        unsafe { StyleItem { internal: nk_style_item_hide() } }
    }
}

// =============================================================================================

wrapper_type_no_clone!(TextEdit, nk_text_edit);

impl Drop for TextEdit {
    fn drop(&mut self) {
        unsafe {
            nk_textedit_free(&mut self.internal);
        }
    }
}

impl TextEdit {
    pub fn init(&mut self, arg2: &mut Allocator, size: Size) {
        unsafe {
            nk_textedit_init(&mut self.internal, &mut arg2.internal as *mut nk_allocator, size);
        }
    }

    pub fn text(&mut self, arg2: &str) {
        unsafe {
            nk_textedit_text(&mut self.internal, arg2.as_ptr() as *const i8, arg2.as_bytes().len() as ::std::os::raw::c_int);
        }
    }

    pub fn delete(&mut self, where_: u32, len: u32) {
        unsafe {
            nk_textedit_delete(&mut self.internal, where_ as ::std::os::raw::c_int, len as ::std::os::raw::c_int);
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
        unsafe { nk_textedit_paste(&mut self.internal, arg2.as_ptr() as *const i8, arg2.as_bytes().len() as ::std::os::raw::c_int) != 0 }
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

wrapper_type!(FontConfig, nk_font_config);

impl FontConfig {
    pub fn with_size(pixel_height: f32) -> FontConfig {
        unsafe { FontConfig { internal: nk_font_config(pixel_height) } }
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

    pub fn glyph_range(&self) -> Option<&[(u32, u32)]> {
        if self.internal.range.is_null() {
            None
        } else {
            Some(raw_glyph_ranges_to_safe(self.internal.range))
        }
    }

    // pub fn set_next<'a>(&'a mut self, next_cfg: &mut FontConfig) {
    // self.internal.next = &mut next_cfg.internal;
    // }

    pub fn padding(&self) -> [u8; 3] {
        self.internal.padding
    }

    pub fn fallback_glyph(&self) -> char {
        unsafe { ::std::char::from_u32_unchecked(self.internal.fallback_glyph) }
    }

    pub fn spacing(&self) -> &Vec2 {
        &self.internal.spacing
    }

    pub fn coord_type(&self) -> &FontCoordType {
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

    // pub fn set_next<'a>(&'a mut self, next_cfg: &mut FontConfig) {
    // self.internal.next = &mut next_cfg.internal;
    // }

    pub fn set_ttf<'a>(&'a mut self, font_bytes: &'a [u8]) {
        self.internal.ttf_size = font_bytes.len() as Size;
        self.internal.ttf_blob = font_bytes as *const _ as *mut c_void;
    }

    pub fn set_padding(&mut self, p: [u8; 3]) {
        self.internal.padding = p;
    }

    pub fn set_fallback_glyph(&mut self, g: char) {
        self.internal.fallback_glyph = g as u32;
    }

    pub fn set_spacing(&mut self, s: Vec2) {
        self.internal.spacing = s;
    }

    pub fn set_coord_type(&mut self, t: FontCoordType) {
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

wrapper_type!(FontAtlas, nk_font_atlas);
pub type FontID = usize;

impl Drop for FontAtlas {
    fn drop(&mut self) {
        self.clear();
    }
}

impl FontAtlas {
    pub fn new(alloc: &mut Allocator) -> FontAtlas {
        let mut a = FontAtlas::default();
        a.init(alloc);
        a
    }

    pub fn add_font_with_config(&mut self, cfg: &FontConfig) -> Option<FontID> {
        unsafe {
            if self.internal.font_num < 1 {
                nk_font_atlas_begin(&mut self.internal as *mut nk_font_atlas);
            }
            let current = self.internal.font_num;

            let ret = nk_font_atlas_add(&mut self.internal as *mut nk_font_atlas, &cfg.internal as *const nk_font_config);

            if !ret.is_null() && (self.internal.font_num - current) == 1 {
                Some(current as FontID)
            } else {
                None
            }
        }
    }

    pub fn add_font_with_bytes(&mut self, font_bytes: &[u8], font_size: f32) -> Option<FontID> {
        let mut cfg = FontConfig::with_size(font_size);

        cfg.internal.ttf_size = font_bytes.len() as Size;
        cfg.internal.ttf_blob = font_bytes as *const _ as *mut c_void;
        cfg.internal.size = font_size;
        cfg.internal.ttf_data_owned_by_atlas = 1;

        self.add_font_with_config(&cfg)
    }

    pub fn bake(&mut self, format: FontAtlasFormat) -> (&[u8], u32, u32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        let image = unsafe { nk_font_atlas_bake(&mut self.internal as *mut nk_font_atlas, &mut width as *mut c_int, &mut height as *mut c_int, format.into()) };

        if width < 1 || height < 1 {
            return (&[], width as u32, height as u32);
        }

        let size = (match format {
            FontAtlasFormat::Alpha8 => 1,
            FontAtlasFormat::Rgba32 => 4,
        } * width
            * height) as usize;

        (unsafe { ::std::slice::from_raw_parts(image as *const u8, size) }, width as u32, height as u32)
    }

    pub fn end(&mut self, hnd: Handle, null_texture: Option<&mut DrawNullTexture>) {
        let nullt = match null_texture {
            Some(n) => &mut n.internal as *mut nk_draw_null_texture,
            None => ::std::ptr::null_mut(),
        };
        unsafe {
            nk_font_atlas_end(&mut self.internal as *mut nk_font_atlas, hnd.internal, nullt);
        }
    }

    pub fn cleanup(&mut self) {
        unsafe {
            nk_font_atlas_cleanup(&mut self.internal as *mut nk_font_atlas);
        }
    }

    fn clear(&mut self) {
        unsafe {
            nk_font_atlas_clear(&mut self.internal as *mut nk_font_atlas);
        }
    }

    fn init(&mut self, arg2: &mut Allocator) {
        unsafe {
            nk_font_atlas_init(&mut self.internal as *mut nk_font_atlas, &mut arg2.internal as *mut nk_allocator);
        }
    }

    #[allow(dead_code)]
    fn init_custom(&mut self, persistent: &mut Allocator, transient: &mut Allocator) {
        unsafe {
            nk_font_atlas_init_custom(&mut self.internal as *mut nk_font_atlas, &mut persistent.internal as *mut nk_allocator, &mut transient.internal as *mut nk_allocator);
        }
    }

    pub fn begin(&mut self) {
        unsafe {
            nk_font_atlas_begin(&mut self.internal as *mut nk_font_atlas);
        }
    }

    pub fn pixels(&self) -> &[u8] {
        unsafe { ::std::slice::from_raw_parts(self.internal.pixel as *const _ as *const u8, (self.internal.tex_width * self.internal.tex_height * 4) as usize) }
    }

    pub fn tex_width(&self) -> u16 {
        self.internal.tex_width as u16
    }

    pub fn tex_height(&self) -> u16 {
        self.internal.tex_height as u16
    }

    pub fn custom(&self) -> Recti {
        self.internal.custom
    }

    pub fn cursors(&self) -> &[Cursor] {
        unsafe { ::std::slice::from_raw_parts(self.internal.cursors.as_ptr() as *const Cursor, self.internal.cursors.len()) }
    }

    pub fn glyphs(&self) -> &[FontGlyph] {
        unsafe { ::std::slice::from_raw_parts(self.internal.glyphs as *const _ as *const FontGlyph, self.internal.glyph_count as usize) }
    }

    pub fn fonts_iterator(&self) -> FontIterator {
        FontIterator { ctx: self }
    }

    pub fn font(&self, id: FontID) -> Option<&Font> {
        let id = self.internal.font_num as usize - id - 1;
        self.fonts_iterator().into_iter().nth(id)
    }
}

pub struct FontIterator<'a> {
    ctx: &'a FontAtlas,
}

impl<'a> IntoIterator for FontIterator<'a> {
    type Item = &'a Font;
    type IntoIter = FontIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let font = if self.ctx.internal.fonts.is_null() { None } else { Some(unsafe { ::std::mem::transmute(self.ctx.internal.fonts) }) };
        FontIntoIter { font }
    }
}

pub struct FontIntoIter<'a> {
    font: Option<&'a Font>,
}

impl<'a> Iterator for FontIntoIter<'a> {
    type Item = &'a Font;
    fn next(&mut self) -> Option<&'a Font> {
        let r = self.font;

        self.font = if let Some(p) = self.font {
            if p.internal.next.is_null() {
                None
            } else {
                Some(unsafe { ::std::mem::transmute(p.internal.next) })
            }
        } else {
            None
        };

        r
    }
}

// =============================================================================================

wrapper_type!(DrawNullTexture, nk_draw_null_texture);

// =============================================================================================

const DEFAULT_BUFFER_SIZE: usize = 8096;

wrapper_type!(Buffer, nk_buffer);

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            nk_buffer_free(&mut self.internal);
        }
    }
}

impl Buffer {
    pub fn new(alloc: &mut Allocator) -> Buffer {
        Buffer::with_size(alloc, DEFAULT_BUFFER_SIZE)
    }

    pub fn with_size(alloc: &mut Allocator, buffer_size: usize) -> Buffer {
        let mut a = Buffer::default();
        unsafe {
            nk_buffer_init(&mut a.internal as *mut nk_buffer, &mut alloc.internal as *const nk_allocator, buffer_size as Size);
        }
        a
    }

    pub fn with_fixed(memory: &mut [u8]) -> Buffer {
        let mut a = Buffer::default();
        unsafe {
            nk_buffer_init_fixed(&mut a.internal as *mut nk_buffer, memory as *mut _ as *mut ::std::os::raw::c_void, memory.len() as Size);
        }
        a
    }

    pub fn total(&mut self) -> usize {
        unsafe { nk_buffer_total(&mut self.internal as *mut nk_buffer) as usize }
    }

    pub fn info(&mut self) -> (usize, usize, usize, usize) /*size, allocated, needed, calls*/ {
        let mut s = nk_memory_status::default();
        unsafe {
            nk_buffer_info(&mut s, &mut self.internal as *mut nk_buffer);
        }
        (s.size as usize, s.allocated as usize, s.needed as usize, s.calls as usize)
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
    // pub fn nk_buffer_total(arg1: *mut nk_buffer) -> &_size;
    //
    // pub fn nk_buffer_init(arg1: *mut nk_buffer, arg2: *const nk_allocator,
    // size: nk_size);
    // pub fn nk_buffer_init_fixed(arg1: *mut nk_buffer,
    // memory: *mut ::std::os::raw::c_void,
    // size: nk_size);
    //
}

// =============================================================================================

pub struct Context {
    internal: nk_context,
}

impl Default for Context {
    fn default() -> Self {
        Context { internal: nk_context::default() }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            nk_free(&mut self.internal as *mut nk_context);
        }
    }
}

impl Context {
    pub fn new(alloc: &mut Allocator, font: &UserFont) -> Context {
        let mut a = Context::default();

        unsafe {
            nk_init(&mut a.internal as *mut nk_context, &mut alloc.internal, &font.internal);
        }

        a
    }

    pub fn clip_mut(&mut self) -> &mut Clipboard {
        unsafe { ::std::mem::transmute(&mut self.internal.clip) }
    }

    pub fn clip(&self) -> &Clipboard {
        unsafe { ::std::mem::transmute(&self.internal.clip) }
    }

    pub fn last_widget_state(&self) -> Flags {
        self.internal.last_widget_state
    }

    pub fn delta_time_seconds(&self) -> f32 {
        self.internal.delta_time_seconds
    }

    pub fn button_behavior(&self) -> ButtonBehavior {
        self.internal.button_behavior.into()
    }

    pub fn set_button_behavior(&mut self, bb: ButtonBehavior) {
        self.internal.button_behavior = bb.into()
    }

    pub fn input_mut(&mut self) -> &mut Input {
        unsafe { ::std::mem::transmute(&mut self.internal.input) }
    }

    pub fn style_mut(&mut self) -> &mut Style {
        unsafe { ::std::mem::transmute(&mut self.internal.style) }
    }

    pub fn draw_list_mut(&mut self) -> &mut DrawList {
        unsafe { ::std::mem::transmute(&mut self.internal.draw_list) }
    }

    pub fn input(&self) -> &Input {
        unsafe { ::std::mem::transmute(&self.internal.input) }
    }

    pub fn style(&self) -> &Style {
        unsafe { ::std::mem::transmute(&self.internal.style) }
    }

    pub fn draw_list(&self) -> &DrawList {
        unsafe { ::std::mem::transmute(&self.internal.draw_list) }
    }

    pub fn clear(&mut self) {
        unsafe {
            nk_clear(&mut self.internal as *mut nk_context);
        }
    }

    pub fn begin(&mut self, title: String, bounds: Rect, flags: Flags) -> bool {
        unsafe { nk_begin(&mut self.internal as *mut nk_context, title.as_ptr(), bounds, flags) != 0 }
    }

    pub fn begin_titled(&mut self, name: String, title: String, bounds: Rect, flags: Flags) -> i32 {
        unsafe { nk_begin_titled(&mut self.internal as *mut nk_context, name.as_ptr(), title.as_ptr(), bounds, flags) }
    }

    pub fn end(&mut self) {
        unsafe {
            nk_end(&mut self.internal as *mut nk_context);
        }
    }

    pub fn window_find<S: AsRef<str>>(&self, name: S) -> Option<&Window> {
        let w = unsafe { nk_window_find(&self.internal as *const _ as *mut nk_context, name.as_ref().as_ptr() as *const i8) };

        unsafe {
            if w.is_null() {
                None
            } else {
                Some(::std::mem::transmute(w))
            }
        }
    }
    pub fn window_find_mut(&mut self, name: String) -> Option<&mut Window> {
        let w = unsafe { nk_window_find(&mut self.internal as *mut nk_context, name.as_ptr()) };

        unsafe {
            if w.is_null() {
                None
            } else {
                Some(::std::mem::transmute(w))
            }
        }
    }

    pub fn window_get_bounds(&self) -> Rect {
        unsafe { nk_window_get_bounds(&self.internal as *const nk_context) }
    }

    pub fn window_get_scroll(&mut self) -> Vector2<u32> {
        unsafe {
            let mut x: nk_uint = 0;
            let mut y: nk_uint = 0;
            nk_window_get_scroll(&mut self.internal as *mut nk_context, &mut x, &mut y);

            Vector2 { x: x as u32, y: y as u32 }
        }
    }

    pub fn window_get_size(&self) -> Vec2 {
        unsafe { nk_window_get_size(&self.internal as *const nk_context) }
    }

    pub fn window_get_position(&self) -> Vec2 {
        unsafe { nk_window_get_position(&self.internal as *const nk_context) }
    }

    pub fn window_get_width(&self) -> f32 {
        unsafe { nk_window_get_width(&self.internal as *const nk_context) }
    }

    pub fn window_get_height(&self) -> f32 {
        unsafe { nk_window_get_height(&self.internal as *const nk_context) }
    }

    pub fn window_get_panel_mut(&mut self) -> Option<&mut Panel> {
        let p = unsafe { nk_window_get_panel(&mut self.internal as *mut nk_context) };

        unsafe {
            if p.is_null() {
                None
            } else {
                Some(::std::mem::transmute(p))
            }
        }
    }
    pub fn window_get_panel(&self) -> Option<&Panel> {
        let p = unsafe { nk_window_get_panel(&self.internal as *const _ as *mut nk_context) };

        unsafe {
            if p.is_null() {
                None
            } else {
                Some(::std::mem::transmute(p))
            }
        }
    }

    pub fn window_get_content_region(&self) -> Rect {
        unsafe { nk_window_get_content_region(&self.internal as *const _ as *mut nk_context) }
    }

    pub fn window_get_content_region_min(&self) -> Vec2 {
        unsafe { nk_window_get_content_region_min(&self.internal as *const _ as *mut nk_context) }
    }

    pub fn window_get_content_region_max(&self) -> Vec2 {
        unsafe { nk_window_get_content_region_max(&self.internal as *const _ as *mut nk_context) }
    }

    pub fn window_get_content_region_size(&self) -> Vec2 {
        unsafe { nk_window_get_content_region_size(&self.internal as *const _ as *mut nk_context) }
    }

    pub fn window_get_canvas_mut(&mut self) -> Option<&mut CommandBuffer> {
        let b = unsafe { nk_window_get_canvas(&mut self.internal as *mut nk_context) };
        unsafe {
            if b.is_null() {
                None
            } else {
                Some(::std::mem::transmute(b))
            }
        }
    }
    pub fn window_get_canvas(&self) -> Option<&CommandBuffer> {
        let b = unsafe { nk_window_get_canvas(&self.internal as *const _ as *mut nk_context) };
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

    pub fn window_is_collapsed(&self, name: String) -> bool {
        unsafe { nk_window_is_collapsed(&self.internal as *const _ as *mut nk_context, name.as_ptr()) > 0 }
    }

    pub fn window_is_closed(&self, name: String) -> bool {
        unsafe { nk_window_is_closed(&self.internal as *const _ as *mut nk_context, name.as_ptr()) > 0 }
    }

    pub fn window_is_hidden(&self, name: String) -> bool {
        unsafe { nk_window_is_hidden(&self.internal as *const _ as *mut nk_context, name.as_ptr()) > 0 }
    }

    pub fn window_is_active(&self, name: String) -> bool {
        unsafe { nk_window_is_active(&self.internal as *const _ as *mut nk_context, name.as_ptr()) > 0 }
    }

    pub fn window_is_hovered(&self) -> bool {
        unsafe { nk_window_is_hovered(&self.internal as *const _ as *mut nk_context) > 0 }
    }

    pub fn window_is_any_hovered(&self) -> bool {
        unsafe { nk_window_is_any_hovered(&self.internal as *const _ as *mut nk_context) > 0 }
    }

    pub fn item_is_any_active(&self) -> bool {
        unsafe { nk_item_is_any_active(&self.internal as *const _ as *mut nk_context) > 0 }
    }

    pub fn window_set_bounds<S: AsRef<str>>(&mut self, name: S, bounds: Rect) {
        unsafe {
            nk_window_set_bounds(&mut self.internal as *mut nk_context, name.as_ref().as_ptr() as *const i8, bounds);
        }
    }

    pub fn window_set_position<S: AsRef<str>>(&mut self, name: S, pos: Vec2) {
        unsafe {
            nk_window_set_position(&mut self.internal as *mut nk_context, name.as_ref().as_ptr() as *const i8, pos);
        }
    }

    pub fn window_set_scroll(&mut self, scroll: Vector2<u32>) {
        unsafe {
            nk_window_set_scroll(&mut self.internal as *mut nk_context, scroll.x, scroll.y);
        }
    }

    pub fn window_set_size<S: AsRef<str>>(&mut self, name: S, size: Vec2) {
        unsafe {
            nk_window_set_size(&mut self.internal as *mut nk_context, name.as_ref().as_ptr() as *const i8, size);
        }
    }

    pub fn window_set_focus(&mut self, name: String) {
        unsafe {
            nk_window_set_focus(&mut self.internal as *mut nk_context, name.as_ptr());
        }
    }

    pub fn window_close(&mut self, name: String) {
        unsafe {
            nk_window_close(&mut self.internal as *mut nk_context, name.as_ptr());
        }
    }

    pub fn window_collapse(&mut self, name: String, state: CollapseState) {
        unsafe {
            nk_window_collapse(&mut self.internal as *mut nk_context, name.as_ptr(), state.into());
        }
    }

    pub fn window_collapse_if(&mut self, name: String, state: CollapseState, cond: bool) {
        unsafe {
            nk_window_collapse_if(&mut self.internal as *mut nk_context, name.as_ptr(), state.into(), if cond { 1 } else { 0 });
        }
    }

    pub fn window_show(&mut self, name: String, state: ShowState) {
        unsafe {
            nk_window_show(&mut self.internal as *mut nk_context, name.as_ptr(), state.into());
        }
    }

    pub fn window_show_if(&mut self, name: String, state: ShowState, cond: bool) {
        unsafe {
            nk_window_show_if(&mut self.internal as *mut nk_context, name.as_ptr(), state.into(), if cond { 1 } else { 0 });
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

    pub fn layout_row_begin(&mut self, fmt: LayoutFormat, row_height: f32, cols: i32) {
        unsafe {
            nk_layout_row_begin(&mut self.internal as *mut nk_context, fmt.into(), row_height, cols);
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

    pub fn layout_row(&mut self, fmt: LayoutFormat, height: f32, cols_ratio: &[f32]) {
        unsafe {
            nk_layout_row(&mut self.internal as *mut nk_context, fmt.into(), height, cols_ratio.len() as i32, cols_ratio.as_ptr());
        }
    }

    pub fn layout_space_begin(&mut self, fmt: LayoutFormat, height: f32, widget_count: i32) {
        unsafe {
            nk_layout_space_begin(&mut self.internal as *mut nk_context, fmt.into(), height, widget_count);
        }
    }

    pub fn layout_space_push(&mut self, space: Rect) {
        unsafe {
            nk_layout_space_push(&mut self.internal as *mut nk_context, space);
        }
    }

    pub fn layout_space_end(&mut self) {
        unsafe {
            nk_layout_space_end(&mut self.internal as *mut nk_context);
        }
    }

    pub fn layout_space_bounds(&mut self) -> Rect {
        unsafe { nk_layout_space_bounds(&mut self.internal as *mut nk_context) }
    }

    pub fn layout_space_to_screen(&mut self, space: Vec2) -> Vec2 {
        unsafe { nk_layout_space_to_screen(&mut self.internal as *mut nk_context, space) }
    }

    pub fn layout_space_to_local(&mut self, space: Vec2) -> Vec2 {
        unsafe { nk_layout_space_to_local(&mut self.internal as *mut nk_context, space) }
    }

    pub fn layout_space_rect_to_screen(&mut self, space: Rect) -> Rect {
        unsafe { nk_layout_space_rect_to_screen(&mut self.internal as *mut nk_context, space) }
    }

    pub fn layout_space_rect_to_local(&mut self, space: Rect) -> Rect {
        unsafe { nk_layout_space_rect_to_local(&mut self.internal as *mut nk_context, space) }
    }

    pub fn layout_ratio_from_pixel(&mut self, pixel_width: f32) -> f32 {
        unsafe { nk_layout_ratio_from_pixel(&mut self.internal as *mut nk_context, pixel_width) }
    }

    pub fn group_begin(&mut self, title: String, flags: Flags) -> i32 {
        unsafe { nk_group_begin(&mut self.internal as *mut nk_context, title.as_ptr(), flags) }
    }

    pub fn group_get_scroll<S: AsRef<str>>(&mut self, id: S) -> Vector2<u32> {
        unsafe {
            let mut x: nk_uint = 0;
            let mut y: nk_uint = 0;
            nk_group_get_scroll(&mut self.internal as *mut nk_context, id.as_ref().as_ptr() as *const ::std::os::raw::c_char, &mut x, &mut y);

            Vector2 { x: x as u32, y: y as u32 }
        }
    }

    pub fn group_set_scroll<S: AsRef<str>>(&mut self, id: S, scroll: Vector2<u32>) {
        unsafe {
            nk_group_set_scroll(&mut self.internal as *mut nk_context, id.as_ref().as_ptr() as *const ::std::os::raw::c_char, scroll.x, scroll.y);
        }
    }

    pub fn group_end(&mut self) {
        unsafe {
            nk_group_end(&mut self.internal as *mut nk_context);
        }
    }

    pub fn tree_push_hashed(&mut self, ty: TreeType, title: String, initial_state: CollapseState, hash: String, len: i32, seed: i32) -> i32 {
        unsafe { nk_tree_push_hashed(&mut self.internal as *mut nk_context, ty.into(), title.as_ptr(), initial_state.into(), hash.as_ptr(), len, seed) }
    }

    pub fn tree_image_push_hashed(&mut self, ty: TreeType, i: Image, title: String, initial_state: CollapseState, hash: String, len: i32, seed: i32) -> i32 {
        unsafe { nk_tree_image_push_hashed(&mut self.internal as *mut nk_context, ty.into(), i.internal, title.as_ptr(), initial_state.into(), hash.as_ptr(), len, seed) }
    }

    pub fn tree_pop(&mut self) {
        unsafe {
            nk_tree_pop(&mut self.internal as *mut nk_context);
        }
    }

    pub fn text(&mut self, text: &str, flags: Flags) {
        unsafe {
            nk_text(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32, flags);
        }
    }

    pub fn text_colored(&mut self, text: &str, flags: Flags, color: Color) {
        unsafe {
            nk_text_colored(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32, flags, color);
        }
    }

    pub fn text_wrap(&mut self, text: &str) {
        unsafe {
            nk_text_wrap(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32);
        }
    }

    pub fn text_wrap_colored(&mut self, text: &str, color: Color) {
        unsafe {
            nk_text_wrap_colored(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32, color);
        }
    }

    pub fn label(&mut self, text: String, flags: Flags) {
        unsafe {
            nk_label(&mut self.internal as *mut nk_context, text.as_ptr(), flags);
        }
    }

    pub fn label_colored(&mut self, text: String, flags: Flags, color: Color) {
        unsafe {
            nk_label_colored(&mut self.internal as *mut nk_context, text.as_ptr(), flags, color);
        }
    }

    pub fn label_wrap(&mut self, text: String) {
        unsafe {
            nk_label_wrap(&mut self.internal as *mut nk_context, text.as_ptr());
        }
    }

    pub fn label_colored_wrap(&mut self, text: String, color: Color) {
        unsafe {
            nk_label_colored_wrap(&mut self.internal as *mut nk_context, text.as_ptr(), color);
        }
    }

    pub fn image(&mut self, img: Image) {
        unsafe {
            nk_image(&mut self.internal as *mut nk_context, img.internal);
        }
    }

    pub fn button_text(&mut self, text: &str) -> bool {
        unsafe { nk_button_text(&mut self.internal as *mut nk_context, text.as_ptr() as *const i8, text.as_bytes().len() as i32) != 0 }
    }

    pub fn button_label(&mut self, title: String) -> bool {
        unsafe { nk_button_label(&mut self.internal as *mut nk_context, title.as_ptr()) != 0 }
    }

    pub fn button_color(&mut self, color: Color) -> bool {
        unsafe { nk_button_color(&mut self.internal as *mut nk_context, color) != 0 }
    }

    pub fn button_symbol(&mut self, ty: SymbolType) -> bool {
        unsafe { nk_button_symbol(&mut self.internal as *mut nk_context, ty.into()) != 0 }
    }

    pub fn button_image(&mut self, img: Image) -> bool {
        unsafe { nk_button_image(&mut self.internal as *mut nk_context, img.internal) != 0 }
    }

    pub fn button_symbol_label(&mut self, ty: SymbolType, title: String, text_alignment: Flags) -> bool {
        unsafe { nk_button_symbol_label(&mut self.internal as *mut nk_context, ty.into(), title.as_ptr(), text_alignment) != 0 }
    }

    pub fn button_symbol_text(&mut self, ty: SymbolType, title: &str, text_alignment: Flags) -> bool {
        unsafe { nk_button_symbol_text(&mut self.internal as *mut nk_context, ty.into(), title.as_ptr() as *const i8, title.as_bytes().len() as i32, text_alignment) != 0 }
    }

    pub fn button_image_label(&mut self, img: Image, title: String, text_alignment: Flags) -> bool {
        unsafe { nk_button_image_label(&mut self.internal as *mut nk_context, img.internal, title.as_ptr(), text_alignment) != 0 }
    }

    pub fn button_image_text(&mut self, img: Image, title: &str, text_alignment: Flags) -> bool {
        unsafe { nk_button_image_text(&mut self.internal as *mut nk_context, img.internal, title.as_ptr() as *const i8, title.as_bytes().len() as i32, text_alignment) != 0 }
    }

    pub fn button_set_behavior(&mut self, b: ButtonBehavior) {
        unsafe {
            nk_button_set_behavior(&mut self.internal as *mut nk_context, b.into());
        }
    }

    pub fn button_push_behavior(&mut self, b: ButtonBehavior) -> i32 {
        unsafe { nk_button_push_behavior(&mut self.internal as *mut nk_context, b.into()) }
    }

    pub fn button_pop_behavior(&mut self) -> i32 {
        unsafe { nk_button_pop_behavior(&mut self.internal as *mut nk_context) }
    }

    pub fn check_label(&mut self, title: String, active: bool) -> i32 {
        unsafe { nk_check_label(&mut self.internal as *mut nk_context, title.as_ptr(), if active { 1 } else { 0 }) }
    }

    pub fn check_text(&mut self, title: &str, active: bool) -> i32 {
        unsafe { nk_check_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.as_bytes().len() as i32, if active { 1 } else { 0 }) }
    }

    pub fn check_flags_label(&mut self, title: String, flags: u32, value: u32) -> u32 {
        unsafe { nk_check_flags_label(&mut self.internal as *mut nk_context, title.as_ptr(), flags, value) }
    }

    pub fn check_flags_text(&mut self, title: &str, flags: u32, value: u32) -> u32 {
        unsafe { nk_check_flags_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.as_bytes().len() as i32, flags, value) }
    }

    pub fn checkbox_label(&mut self, title: String, active: &mut bool) -> bool {
        let mut i = if *active { 1 } else { 0 };
        let r = unsafe { nk_checkbox_label(&mut self.internal as *mut nk_context, title.as_ptr(), &mut i as *mut i32) != 0 };

        *active = i != 0;
        r
    }

    pub fn checkbox_text(&mut self, title: &str, active: &mut bool) -> bool {
        let mut i = if *active { 1 } else { 0 };
        let r = unsafe { nk_checkbox_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.as_bytes().len() as i32, &mut i as *mut i32) != 0 };

        *active = i != 0;
        r
    }

    pub fn checkbox_flags_label(&mut self, title: String, flags: &mut u32, value: u32) -> bool {
        unsafe { nk_checkbox_flags_label(&mut self.internal as *mut nk_context, title.as_ptr(), flags, value) != 0 }
    }

    pub fn checkbox_flags_text(&mut self, title: &str, flags: &mut u32, value: u32) -> bool {
        unsafe { nk_checkbox_flags_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.as_bytes().len() as i32, flags, value) != 0 }
    }

    pub fn radio_label(&mut self, title: String, active: &mut bool) -> bool {
        let mut i = if *active { 1 } else { 0 };
        let r = unsafe { nk_radio_label(&mut self.internal as *mut nk_context, title.as_ptr(), &mut i as *mut i32) != 0 };

        *active = i != 0;
        r
    }

    pub fn radio_text(&mut self, title: &str, active: &mut bool) -> bool {
        let mut i = if *active { 1 } else { 0 };
        let r = unsafe { nk_radio_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.as_bytes().len() as i32, &mut i as *mut i32) != 0 };

        *active = i != 0;
        r
    }

    pub fn option_label(&mut self, title: String, active: bool) -> bool {
        unsafe { nk_option_label(&mut self.internal as *mut nk_context, title.as_ptr(), if active { 1 } else { 0 }) > 0 }
    }

    pub fn option_text(&mut self, title: &str, active: bool) -> bool {
        unsafe { nk_option_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.as_bytes().len() as i32, if active { 1 } else { 0 }) > 0 }
    }

    pub fn selectable_label(&mut self, title: String, align: Flags, value: &mut i32) -> bool {
        unsafe { nk_selectable_label(&mut self.internal as *mut nk_context, title.as_ptr(), align, value) != 0 }
    }

    pub fn selectable_text(&mut self, title: &str, align: Flags, value: &mut i32) -> bool {
        unsafe { nk_selectable_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.as_bytes().len() as i32, align, value) != 0 }
    }

    pub fn selectable_image_label(&mut self, img: Image, title: String, align: Flags, value: &mut i32) -> bool {
        unsafe { nk_selectable_image_label(&mut self.internal as *mut nk_context, img.internal, title.as_ptr(), align, value) != 0 }
    }

    pub fn selectable_image_text(&mut self, img: Image, title: &str, align: Flags, value: &mut i32) -> bool {
        unsafe { nk_selectable_image_text(&mut self.internal as *mut nk_context, img.internal, title.as_ptr() as *const i8, title.as_bytes().len() as i32, align, value) != 0 }
    }

    pub fn select_label(&mut self, title: String, align: Flags, value: i32) -> i32 {
        unsafe { nk_select_label(&mut self.internal as *mut nk_context, title.as_ptr(), align, value) }
    }

    pub fn select_text(&mut self, title: &str, align: Flags, value: i32) -> i32 {
        unsafe { nk_select_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.as_bytes().len() as i32, align, value) }
    }

    pub fn select_image_label(&mut self, img: Image, title: String, align: Flags, value: i32) -> i32 {
        unsafe { nk_select_image_label(&mut self.internal as *mut nk_context, img.internal, title.as_ptr(), align, value) }
    }

    pub fn select_image_text(&mut self, img: Image, title: &str, align: Flags, value: i32) -> i32 {
        unsafe { nk_select_image_text(&mut self.internal as *mut nk_context, img.internal, title.as_ptr() as *const i8, title.as_bytes().len() as i32, align, value) }
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

    pub fn progress(&mut self, cur: &mut Size, max: Size, is_modifyable: bool) -> bool {
        unsafe { nk_progress(&mut self.internal as *mut nk_context, cur, max, if is_modifyable { 1 } else { 0 }) != 0 }
    }

    pub fn prog(&mut self, cur: Size, max: Size, is_modifyable: bool) -> usize {
        unsafe { nk_prog(&mut self.internal as *mut nk_context, cur, max, if is_modifyable { 1 } else { 0 }) as usize }
    }

    pub fn color_picker(&mut self, color: ColorF, fmt: ColorFormat) -> ColorF {
        unsafe { nk_color_picker(&mut self.internal as *mut nk_context, color, fmt.into()) }
    }

    pub fn color_pick(&mut self, fmt: ColorFormat) -> (bool, ColorF) {
        let mut c = ColorF::default();
        let changed = unsafe { nk_color_pick(&mut self.internal as *mut nk_context, &mut c as *mut nk_colorf, fmt.into()) };
        (changed != 0, c)
    }

    pub fn property_int(&mut self, name: String, min: i32, val: &mut i32, max: i32, step: i32, inc_per_pixel: f32) {
        unsafe {
            nk_property_int(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel);
        }
    }

    pub fn property_float(&mut self, name: String, min: f32, val: &mut f32, max: f32, step: f32, inc_per_pixel: f32) {
        unsafe { nk_property_float(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel) }
    }

    pub fn property_double(&mut self, name: String, min: f64, val: &mut f64, max: f64, step: f64, inc_per_pixel: f32) {
        unsafe { nk_property_double(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel) }
    }

    pub fn propertyi(&mut self, name: String, min: i32, val: i32, max: i32, step: i32, inc_per_pixel: f32) -> i32 {
        unsafe { nk_propertyi(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel) }
    }

    pub fn propertyf(&mut self, name: String, min: f32, val: f32, max: f32, step: f32, inc_per_pixel: f32) -> f32 {
        unsafe { nk_propertyf(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel) }
    }

    pub fn propertyd(&mut self, name: String, min: f64, val: f64, max: f64, step: f64, inc_per_pixel: f32) -> f64 {
        unsafe { nk_propertyd(&mut self.internal as *mut nk_context, name.as_ptr(), min, val, max, step, inc_per_pixel) }
    }

    pub fn edit_string_custom_filter(&mut self, flags: Flags, buffer: &mut [u8], len: &mut i32, filter: fn(&TextEdit, char) -> bool) -> Flags {
        unsafe {
            CUSTOM_EDIT_FILTER = Some(filter);
            nk_edit_string(&mut self.internal as *mut nk_context, flags, &mut buffer[0] as *mut _ as *mut i8, len, buffer.len() as i32, Some(nk_filter_custom))
        }
    }

    pub fn edit_string(&mut self, flags: Flags, buffer: &mut [u8], len: &mut i32, filter: PluginFilter) -> Flags {
        unsafe { nk_edit_string(&mut self.internal as *mut nk_context, flags, &mut buffer[0] as *mut _ as *mut i8, len, buffer.len() as i32, filter) }
    }

    pub fn edit_buffer(&mut self, flags: Flags, editor: &mut TextEdit, filter: PluginFilter) -> Flags {
        unsafe { nk_edit_buffer(&mut self.internal as *mut nk_context, flags, &mut editor.internal, filter) }
    }

    pub fn chart_begin(&mut self, ty: ChartType, num: i32, min: f32, max: f32) -> bool {
        unsafe { nk_chart_begin(&mut self.internal as *mut nk_context, ty.into(), num, min, max) > 0 }
    }

    pub fn chart_begin_colored(&mut self, ty: ChartType, color: Color, active: Color, num: i32, min: f32, max: f32) -> bool {
        unsafe { nk_chart_begin_colored(&mut self.internal as *mut nk_context, ty.into(), color, active, num, min, max) > 0 }
    }

    pub fn chart_add_slot(&mut self, ty: ChartType, count: i32, min_value: f32, max_value: f32) {
        unsafe {
            nk_chart_add_slot(&mut self.internal as *mut nk_context, ty.into(), count, min_value, max_value);
        }
    }

    pub fn chart_add_slot_colored(&mut self, ty: ChartType, color: Color, active: Color, count: i32, min_value: f32, max_value: f32) {
        unsafe {
            nk_chart_add_slot_colored(&mut self.internal as *mut nk_context, ty.into(), color, active, count, min_value, max_value);
        }
    }

    pub fn chart_push(&mut self, value: f32) -> Flags {
        unsafe { nk_chart_push(&mut self.internal as *mut nk_context, value) }
    }

    pub fn chart_push_slot(&mut self, value: f32, count: i32) -> Flags {
        unsafe { nk_chart_push_slot(&mut self.internal as *mut nk_context, value, count) }
    }

    pub fn chart_end(&mut self) {
        unsafe {
            nk_chart_end(&mut self.internal as *mut nk_context);
        }
    }

    pub fn plot(&mut self, ty: ChartType, values: &[f32]) {
        unsafe {
            nk_plot(&mut self.internal as *mut nk_context, ty.into(), values.as_ptr(), values.len() as i32, 0);
        }
    }

    // pub fn plot_function(&mut self, ty: ChartType, userdata: &[f32], offset: i32) {
    // unsafe {
    // nk_plot_function(&mut self.internal as *mut nk_context, ty, userdata as *const _ as *mut ::std::os::raw::c_void, Some(nk_plot_value_getter_custom), userdata.len() as i32, offset);
    // }
    // }

    pub fn popup_begin(&mut self, ty: PopupType, title: String, flags: Flags, bounds: Rect) -> bool {
        unsafe { nk_popup_begin(&mut self.internal as *mut nk_context, ty.into(), title.as_ptr(), flags, bounds) > 0 }
    }

    pub fn popup_get_scroll(&mut self) -> Vector2<u32> {
        unsafe {
            let mut x: nk_uint = 0;
            let mut y: nk_uint = 0;
            nk_popup_get_scroll(&mut self.internal as *mut nk_context, &mut x, &mut y);

            Vector2 { x: x as u32, y: y as u32 }
        }
    }

    pub fn popup_set_scroll(&mut self, scroll: Vector2<u32>) {
        unsafe {
            nk_popup_set_scroll(&mut self.internal as *mut nk_context, scroll.x, scroll.y);
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

    pub fn combo(&mut self, items: &mut StringArray, selected: i32, item_height: i32, size: Vec2) -> i32 {
        unsafe { nk_combo(&mut self.internal as *mut nk_context, items.as_mut(), items.len() as i32, selected, item_height, size) }
    }

    pub fn combo_separator(&mut self, items_separated_by_separator: String, separator: char, selected: i32, item_height: i32, size: Vec2) -> i32 {
        let len = ::std::string::String::from_utf8_lossy(items_separated_by_separator.bytes.as_ref()).as_ref().split(separator).count();
        unsafe { nk_combo_separator(&mut self.internal as *mut nk_context, items_separated_by_separator.as_ptr(), separator as i32, selected, len as i32, item_height, size) }
    }

    pub fn combo_begin_label(&mut self, selected: String, size: Vec2) -> bool {
        unsafe { nk_combo_begin_label(&mut self.internal as *mut nk_context, selected.as_ptr(), size) > 0 }
    }

    pub fn combo_begin_text(&mut self, selected: &str, size: Vec2) -> bool {
        unsafe { nk_combo_begin_text(&mut self.internal as *mut nk_context, selected.as_ptr() as *const i8, selected.as_bytes().len() as i32, size) > 0 }
    }

    pub fn combo_begin_color(&mut self, color: Color, size: Vec2) -> bool {
        unsafe { nk_combo_begin_color(&mut self.internal as *mut nk_context, color, size) > 0 }
    }

    pub fn combo_begin_symbol(&mut self, sym: SymbolType, size: Vec2) -> bool {
        unsafe { nk_combo_begin_symbol(&mut self.internal as *mut nk_context, sym.into(), size) > 0 }
    }

    pub fn combo_begin_symbol_label(&mut self, label: String, sym: SymbolType, size: Vec2) -> bool {
        unsafe { nk_combo_begin_symbol_label(&mut self.internal as *mut nk_context, label.as_ptr(), sym.into(), size) > 0 }
    }

    pub fn combo_begin_symbol_text(&mut self, label: &str, sym: SymbolType, size: Vec2) -> bool {
        unsafe { nk_combo_begin_symbol_text(&mut self.internal as *mut nk_context, label.as_ptr() as *const i8, label.as_bytes().len() as i32, sym.into(), size) > 0 }
    }

    pub fn combo_begin_image(&mut self, img: Image, size: Vec2) -> bool {
        unsafe { nk_combo_begin_image(&mut self.internal as *mut nk_context, img.internal, size) > 0 }
    }

    pub fn combo_begin_image_label(&mut self, label: String, img: Image, size: Vec2) -> bool {
        unsafe { nk_combo_begin_image_label(&mut self.internal as *mut nk_context, label.as_ptr(), img.internal, size) > 0 }
    }

    pub fn combo_begin_image_text(&mut self, label: &str, img: Image, size: Vec2) -> bool {
        unsafe { nk_combo_begin_image_text(&mut self.internal as *mut nk_context, label.as_ptr() as *const i8, label.as_bytes().len() as i32, img.internal, size) > 0 }
    }

    pub fn combo_item_label(&mut self, label: String, alignment: Flags) -> bool {
        unsafe { nk_combo_item_label(&mut self.internal as *mut nk_context, label.as_ptr(), alignment) > 0 }
    }

    pub fn combo_item_text(&mut self, label: &str, alignment: Flags) -> bool {
        unsafe { nk_combo_item_text(&mut self.internal as *mut nk_context, label.as_ptr() as *const i8, label.as_bytes().len() as i32, alignment) > 0 }
    }

    pub fn combo_item_image_label(&mut self, img: Image, label: String, alignment: Flags) -> bool {
        unsafe { nk_combo_item_image_label(&mut self.internal as *mut nk_context, img.internal, label.as_ptr(), alignment) > 0 }
    }

    pub fn combo_item_image_text(&mut self, img: Image, label: &str, alignment: Flags) -> bool {
        unsafe { nk_combo_item_image_text(&mut self.internal as *mut nk_context, img.internal, label.as_ptr() as *const i8, label.as_bytes().len() as i32, alignment) > 0 }
    }

    pub fn combo_item_symbol_label(&mut self, sym: SymbolType, label: String, alignment: Flags) -> bool {
        unsafe { nk_combo_item_symbol_label(&mut self.internal as *mut nk_context, sym.into(), label.as_ptr(), alignment) > 0 }
    }

    pub fn combo_item_symbol_text(&mut self, sym: SymbolType, label: &str, alignment: Flags) -> bool {
        unsafe { nk_combo_item_symbol_text(&mut self.internal as *mut nk_context, sym.into(), label.as_ptr() as *const i8, label.as_bytes().len() as i32, alignment) > 0 }
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

    pub fn contextual_begin(&mut self, flags: Flags, bounds: Vec2, trigger_bounds: Rect) -> bool {
        unsafe { nk_contextual_begin(&mut self.internal as *mut nk_context, flags, bounds, trigger_bounds) > 0 }
    }

    pub fn contextual_item_label(&mut self, label: String, align: Flags) -> bool {
        unsafe { nk_contextual_item_label(&mut self.internal as *mut nk_context, label.as_ptr(), align) > 0 }
    }

    pub fn contextual_item_text(&mut self, label: &str, align: Flags) -> bool {
        unsafe { nk_contextual_item_text(&mut self.internal as *mut nk_context, label.as_ptr() as *const i8, label.as_bytes().len() as i32, align) > 0 }
    }

    pub fn contextual_item_image_label(&mut self, img: Image, label: String, align: Flags) -> bool {
        unsafe { nk_contextual_item_image_label(&mut self.internal as *mut nk_context, img.internal, label.as_ptr(), align) > 0 }
    }

    pub fn contextual_item_image_text(&mut self, img: Image, label: &str, align: Flags) -> bool {
        unsafe { nk_contextual_item_image_text(&mut self.internal as *mut nk_context, img.internal, label.as_ptr() as *const i8, label.as_bytes().len() as i32, align) > 0 }
    }

    pub fn contextual_item_symbol_label(&mut self, sym: SymbolType, label: String, align: Flags) -> bool {
        unsafe { nk_contextual_item_symbol_label(&mut self.internal as *mut nk_context, sym.into(), label.as_ptr(), align) > 0 }
    }

    pub fn contextual_item_symbol_text(&mut self, sym: SymbolType, label: &str, align: Flags) -> bool {
        unsafe { nk_contextual_item_symbol_text(&mut self.internal as *mut nk_context, sym.into(), label.as_ptr() as *const i8, label.as_bytes().len() as i32, align) > 0 }
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

    pub fn tooltip(&mut self, text: String) {
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

    pub fn menu_begin_label(&mut self, title: String, align: Flags, size: Vec2) -> bool {
        unsafe { nk_menu_begin_label(&mut self.internal as *mut nk_context, title.as_ptr(), align, size) > 0 }
    }

    pub fn menu_begin_text(&mut self, title: &str, align: Flags, size: Vec2) -> bool {
        unsafe { nk_menu_begin_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.len() as i32, align, size) > 0 }
    }

    pub fn menu_begin_image(&mut self, title: String, img: Image, size: Vec2) -> bool {
        unsafe { nk_menu_begin_image(&mut self.internal as *mut nk_context, title.as_ptr(), img.internal, size) > 0 }
    }

    pub fn menu_begin_image_label(&mut self, title: String, align: Flags, img: Image, size: Vec2) -> bool {
        unsafe { nk_menu_begin_image_label(&mut self.internal as *mut nk_context, title.as_ptr(), align, img.internal, size) > 0 }
    }

    pub fn menu_begin_image_text(&mut self, title: &str, align: Flags, img: Image, size: Vec2) -> bool {
        unsafe { nk_menu_begin_image_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.len() as i32, align, img.internal, size) > 0 }
    }

    pub fn menu_begin_symbol(&mut self, title: String, sym: SymbolType, size: Vec2) -> bool {
        unsafe { nk_menu_begin_symbol(&mut self.internal as *mut nk_context, title.as_ptr(), sym.into(), size) > 0 }
    }

    pub fn menu_begin_symbol_label(&mut self, title: String, align: Flags, sym: SymbolType, size: Vec2) -> bool {
        unsafe { nk_menu_begin_symbol_label(&mut self.internal as *mut nk_context, title.as_ptr(), align, sym.into(), size) > 0 }
    }

    pub fn menu_begin_symbol_text(&mut self, title: &str, align: Flags, sym: SymbolType, size: Vec2) -> bool {
        unsafe { nk_menu_begin_symbol_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.len() as i32, align, sym.into(), size) > 0 }
    }

    pub fn menu_item_label(&mut self, title: String, align: Flags) -> bool {
        unsafe { nk_menu_item_label(&mut self.internal as *mut nk_context, title.as_ptr(), align) > 0 }
    }

    pub fn menu_item_text(&mut self, title: &str, align: Flags) -> bool {
        unsafe { nk_menu_item_text(&mut self.internal as *mut nk_context, title.as_ptr() as *const i8, title.len() as i32, align) > 0 }
    }

    pub fn menu_item_image_label(&mut self, img: Image, title: String, align: Flags) -> bool {
        unsafe { nk_menu_item_image_label(&mut self.internal as *mut nk_context, img.internal, title.as_ptr(), align) > 0 }
    }

    pub fn menu_item_image_text(&mut self, img: Image, title: &str, align: Flags) -> bool {
        unsafe { nk_menu_item_image_text(&mut self.internal as *mut nk_context, img.internal, title.as_ptr() as *const i8, title.len() as i32, align) > 0 }
    }

    pub fn menu_item_symbol_label(&mut self, sym: SymbolType, title: String, align: Flags) -> bool {
        unsafe { nk_menu_item_symbol_label(&mut self.internal as *mut nk_context, sym.into(), title.as_ptr(), align) > 0 }
    }

    pub fn menu_item_symbol_text(&mut self, sym: SymbolType, title: &str, align: Flags) -> bool {
        unsafe { nk_menu_item_symbol_text(&mut self.internal as *mut nk_context, sym.into(), title.as_ptr() as *const i8, title.len() as i32, align) > 0 }
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

    pub fn convert(&mut self, cmds: &mut Buffer, vertices: &mut Buffer, elements: &mut Buffer, config: &ConvertConfig) {
        unsafe {
            nk_convert(
                &mut self.internal as *mut nk_context,
                &mut cmds.internal as *mut nk_buffer,
                &mut vertices.internal as *mut nk_buffer,
                &mut elements.internal as *mut nk_buffer,
                &config.internal as *const nk_convert_config,
            );
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

    pub fn input_key(&mut self, key: Key, down: bool) {
        unsafe {
            nk_input_key(&mut self.internal as *mut nk_context, key.into(), if down { 1 } else { 0 });
        }
    }

    pub fn input_button(&mut self, b: Button, x: i32, y: i32, down: bool) {
        unsafe {
            nk_input_button(&mut self.internal as *mut nk_context, b.into(), x, y, if down { 1 } else { 0 });
        }
    }

    pub fn input_scroll(&mut self, y: Vec2) {
        unsafe {
            nk_input_scroll(&mut self.internal as *mut nk_context, y);
        }
    }

    pub fn input_char(&mut self, c: u8) {
        unsafe {
            nk_input_char(&mut self.internal as *mut nk_context, c as i8);
        }
    }

    pub fn input_glyph(&mut self, g: Glyph) {
        unsafe {
            nk_input_glyph(&mut self.internal as *mut nk_context, &g[0] as *const _ as *mut i8);
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

    pub fn style_from_table(&mut self, table: &ColorMap) {
        unsafe {
            nk_style_from_table(&mut self.internal as *mut nk_context, &table.internal[0] as *const nk_color);
        }
    }

    pub fn style_load_cursor(&mut self, cur: StyleCursor, res: &Cursor) {
        unsafe {
            nk_style_load_cursor(&mut self.internal as *mut nk_context, cur, &res.internal);
        }
    }

    pub fn style_load_all_cursors(&mut self, table: &mut CursorMap) {
        unsafe {
            nk_style_load_all_cursors(&mut self.internal as *mut nk_context, table.internal.as_mut_ptr() as *mut nk_cursor);
        }
    }

    pub fn style_set_font(&mut self, font: &UserFont) {
        unsafe {
            nk_style_set_font(&mut self.internal as *mut nk_context, &font.internal);
        }
    }

    pub fn style_set_cursor(&mut self, cur: StyleCursor) -> bool {
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

    pub fn style_push_font(&mut self, font: &mut UserFont) -> bool {
        unsafe { nk_style_push_font(&mut self.internal as *mut nk_context, &mut font.internal) > 0 }
    }

    pub fn style_push_float(&mut self, addr: &mut f32, val: f32) -> bool {
        unsafe { nk_style_push_float(&mut self.internal as *mut nk_context, addr as *mut f32, val) > 0 }
    }

    pub fn style_push_vec2(&mut self, addr: &mut Vec2, val: Vec2) -> bool {
        unsafe { nk_style_push_vec2(&mut self.internal as *mut nk_context, addr as *mut nk_vec2, val) > 0 }
    }

    pub fn style_push_style_item(&mut self, addr: &mut StyleItem, val: StyleItem) -> bool {
        unsafe { nk_style_push_style_item(&mut self.internal as *mut nk_context, &mut addr.internal as *mut nk_style_item, val.internal) > 0 }
    }

    pub fn style_push_flags(&mut self, addr: &mut Flags, val: Flags) -> bool {
        unsafe { nk_style_push_flags(&mut self.internal as *mut nk_context, addr as *mut nk_flags, val) > 0 }
    }

    pub fn style_push_color(&mut self, addr: &mut Color, val: Color) -> bool {
        unsafe { nk_style_push_color(&mut self.internal as *mut nk_context, addr as *mut nk_color, val) > 0 }
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

    pub fn widget_bounds(&mut self) -> Rect {
        unsafe { nk_widget_bounds(&mut self.internal as *mut nk_context) }
    }

    pub fn widget_position(&mut self) -> Vec2 {
        unsafe { nk_widget_position(&mut self.internal as *mut nk_context) }
    }

    pub fn widget_size(&mut self) -> Vec2 {
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

    pub fn widget_is_mouse_clicked(&mut self, b: Button) -> bool {
        unsafe { nk_widget_is_mouse_clicked(&mut self.internal as *mut nk_context, b.into()) > 0 }
    }

    pub fn widget_has_mouse_click_down(&mut self, b: Button, down: bool) -> bool {
        unsafe { nk_widget_has_mouse_click_down(&mut self.internal as *mut nk_context, b.into(), if down { 1 } else { 0 }) > 0 }
    }

    pub fn widget(&self, arg1: &mut Rect) -> WidgetLayoutState {
        unsafe { nk_widget(arg1, &self.internal as *const nk_context) }
    }

    pub fn spacing(&mut self, cols: i32) {
        unsafe {
            nk_spacing(&mut self.internal as *mut nk_context, cols);
        }
    }

    pub fn draw_begin(&self, buf: &Buffer) -> Option<&DrawCommand> {
        let n = unsafe { nk__draw_begin(&self.internal, &buf.internal) };

        unsafe {
            if n.is_null() {
                None
            } else {
                Some(::std::mem::transmute(n))
            }
        }
    }
    pub fn draw_next<'a>(&self, prev: &DrawCommand, buf: &Buffer) -> Option<&'a DrawCommand> {
        let n = unsafe { nk__draw_next(&prev.internal, &buf.internal, &self.internal) };

        unsafe {
            if n.is_null() {
                None
            } else {
                Some(::std::mem::transmute(n))
            }
        }
    }

    pub fn next_cmd<'a, 'b>(&self, arg2: &'b Command) -> Option<&'a Command> {
        let r = unsafe { nk__next(&self.internal as *const _ as *mut nk_context, &arg2.internal) };
        unsafe {
            if r.is_null() {
                None
            } else {
                Some(::std::mem::transmute(r))
            }
        }
    }

    pub fn begin_cmd(&self) -> Option<&Command> {
        let r = unsafe { nk__begin(&self.internal as *const _ as *mut nk_context) };
        unsafe {
            if r.is_null() {
                None
            } else {
                Some(::std::mem::transmute(r))
            }
        }
    }

    pub fn draw_command_iterator<'a>(&'a mut self, buf: &'a Buffer) -> DrawCommandIterator<'a> {
        DrawCommandIterator { ctx: self, buf }
    }

    pub fn command_iterator(&mut self) -> CommandIterator {
        CommandIterator { ctx: self }
    }
}

// ============================================================================================

pub struct CommandIterator<'a> {
    ctx: &'a Context,
}

impl<'a> IntoIterator for CommandIterator<'a> {
    type Item = &'a Command;
    type IntoIter = CommandIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let cmd = self.ctx.begin_cmd();
        CommandIntoIter { ctx: self.ctx, cmd }
    }
}

pub struct CommandIntoIter<'a> {
    ctx: &'a Context,
    cmd: Option<&'a Command>,
}

impl<'a> Iterator for CommandIntoIter<'a> {
    type Item = &'a Command;
    fn next(&mut self) -> Option<&'a Command> {
        let r = self.cmd;

        self.cmd = if let Some(p) = self.cmd { self.ctx.next_cmd(p) } else { None };

        r
    }
}

// ============================================================================================

pub struct DrawCommandIterator<'a> {
    ctx: &'a mut Context,
    buf: &'a Buffer,
}

impl<'a> IntoIterator for DrawCommandIterator<'a> {
    type Item = &'a DrawCommand;
    type IntoIter = DrawCommandIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let cmd = self.ctx.draw_begin(self.buf);
        DrawCommandIntoIter { ctx: self.ctx, buf: self.buf, cmd }
    }
}

pub struct DrawCommandIntoIter<'a> {
    ctx: &'a Context,
    buf: &'a Buffer,
    cmd: Option<&'a DrawCommand>,
}

impl<'a> Iterator for DrawCommandIntoIter<'a> {
    type Item = &'a DrawCommand;
    fn next(&mut self) -> Option<&'a DrawCommand> {
        let r = self.cmd;

        self.cmd = if let Some(ref p) = self.cmd { self.ctx.draw_next(p, self.buf) } else { None };

        r
    }
}

// =============================================================================================

wrapper_type_no_clone!(Window, nk_window);

impl Window {
    pub fn seq(&self) -> u32 {
        self.internal.seq
    }
    pub fn name(&self) -> &str {
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
    pub fn flags(&self) -> &Flags {
        &self.internal.flags
    }
    pub fn bounds(&self) -> &Rect {
        &self.internal.bounds
    }
    pub fn scrollbar(&self) -> &Scroll {
        &self.internal.scrollbar
    }
    pub fn scrollbar_hiding_timer(&self) -> f32 {
        self.internal.scrollbar_hiding_timer
    }
    pub fn buffer(&self) -> &CommandBuffer {
        unsafe { ::std::mem::transmute(&self.internal.buffer) }
    }
    pub fn layout(&self) -> &Panel {
        unsafe { ::std::mem::transmute(self.internal.layout) }
    }
    pub fn layout_mut(&mut self) -> &mut Panel {
        unsafe { ::std::mem::transmute(self.internal.layout) }
    }
    pub fn property(&self) -> &PropertyState {
        unsafe { ::std::mem::transmute(&self.internal.property) }
    }
    pub fn popup(&self) -> &PopupState {
        unsafe { ::std::mem::transmute(&self.internal.popup) }
    }
    pub fn edit(&self) -> &EditState {
        unsafe { ::std::mem::transmute(&self.internal.edit) }
    }
    pub fn scrolled(&self) -> u32 {
        self.internal.scrolled
    }
    pub fn tables(&self) -> &[Table] {
        unsafe { ::std::slice::from_raw_parts(self.internal.tables as *mut _ as *const Table, self.internal.table_count as usize) }
    }
    pub fn prev(&self) -> &Window {
        unsafe { ::std::mem::transmute(self.internal.prev) }
    }
    pub fn next(&self) -> &Window {
        unsafe { ::std::mem::transmute(self.internal.next) }
    }
    pub fn parent(&self) -> &Window {
        unsafe { ::std::mem::transmute(self.internal.parent) }
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.internal.flags = flags;
    }
    pub fn set_bounds(&mut self, rect: Rect) {
        self.internal.bounds = rect;
    }
    pub fn set_scrollbar(&mut self, scroll: Scroll) {
        self.internal.scrollbar = scroll;
    }
    pub fn set_scrollbar_hiding_timer(&mut self, value: f32) {
        self.internal.scrollbar_hiding_timer = value;
    }
}

wrapper_type_no_clone!(PropertyState, nk_property_state);
wrapper_type!(PopupState, nk_popup_state);
wrapper_type!(EditState, nk_edit_state);
wrapper_type_no_clone!(Table, nk_table);

// =============================================================================================

wrapper_type!(RowLayout, nk_row_layout);

impl RowLayout {
    pub fn layout_type(&self) -> &PanelRowLayoutType {
        &self.internal.type_
    }
    pub fn layout_type_mut(&mut self) -> &mut PanelRowLayoutType {
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

    pub fn item(&self) -> &Rect {
        &self.internal.item
    }
    pub fn item_mut(&mut self) -> &mut Rect {
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

wrapper_type!(Panel, nk_panel);

impl Panel {
    pub fn bounds(&self) -> &Rect {
        &self.internal.bounds
    }
    pub fn bounds_mut(&mut self) -> &mut Rect {
        &mut self.internal.bounds
    }
    pub fn set_bounds(&mut self, b: Rect) {
        self.internal.bounds = b
    }

    pub fn panel_type(&self) -> &PanelType {
        &self.internal.type_
    }
    pub fn panel_type_mut(&mut self) -> &mut PanelType {
        &mut self.internal.type_
    }
    pub fn set_panel_type(&mut self, t: PanelType) {
        self.internal.type_ = t
    }

    pub fn flags(&self) -> &Flags {
        &self.internal.flags
    }
    pub fn flags_mut(&mut self) -> &mut Flags {
        &mut self.internal.flags
    }
    pub fn set_flags(&mut self, f: Flags) {
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

    pub fn clip(&self) -> &Rect {
        &self.internal.clip
    }
    pub fn clip_mut(&mut self) -> &mut Rect {
        &mut self.internal.clip
    }
    pub fn set_clip(&mut self, f: Rect) {
        self.internal.clip = f
    }

    pub fn menu(&self) -> &MenuState {
        &self.internal.menu
    }
    pub fn menu_mut(&mut self) -> &mut MenuState {
        &mut self.internal.menu
    }
    pub fn set_menu(&mut self, f: MenuState) {
        self.internal.menu = f
    }

    pub fn row(&self) -> &RowLayout {
        unsafe { ::std::mem::transmute(&self.internal.row) }
    }

    pub fn chart(&self) -> &Chart {
        unsafe { ::std::mem::transmute(&self.internal.chart) }
    }

    pub fn buffer(&self) -> Option<&CommandBuffer> {
        unsafe {
            let ptr = self.internal.buffer;
            if !ptr.is_null() {
                Some(::std::mem::transmute(ptr))
            } else {
                None
            }
        }
    }

    pub fn parent(&self) -> Option<&Panel> {
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

wrapper_type!(Chart, nk_chart);

impl Chart {
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

    pub fn slots(&self) -> &[ChartSlot] {
        &self.internal.slots
    }
}

// =============================================================================================

macro_rules! emit_nk_command {
    ($rs_ty:ident, $nat_ty:ty) => {
        wrapper_type!($rs_ty, $nat_ty);

        impl AsRef<$rs_ty> for Command {
            fn as_ref(&self) -> &$rs_ty {
                unsafe { ::std::mem::transmute(&self.internal) }
            }
        }

        impl $rs_ty {
            pub fn header(&self) -> &Command {
                unsafe { ::std::mem::transmute(&self.internal.header) }
            }
        }
    };
}

wrapper_type!(Command, nk_command);

impl Command {
    pub fn get_type(&self) -> CommandType {
        self.internal.type_.into()
    }
}

emit_nk_command!(CommandScissor, nk_command_scissor);
impl CommandScissor {
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

emit_nk_command!(CommandLine, nk_command_line);
impl CommandLine {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn begin(&self) -> Vec2i {
        self.internal.begin
    }
    pub fn end(&self) -> Vec2i {
        self.internal.end
    }
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandCurve, nk_command_curve);
impl CommandCurve {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn begin(&self) -> Vec2i {
        self.internal.begin
    }
    pub fn end(&self) -> Vec2i {
        self.internal.end
    }
    pub fn color(&self) -> Color {
        self.internal.color
    }
    pub fn ctrl(&self) -> &[Vec2i] {
        &self.internal.ctrl
    }
}

emit_nk_command!(CommandRect, nk_command_rect);
impl CommandRect {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn rounding(&self) -> u16 {
        self.internal.rounding
    }
    pub fn color(&self) -> Color {
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

emit_nk_command!(CommandRectFilled, nk_command_rect_filled);
impl CommandRectFilled {
    pub fn rounding(&self) -> u16 {
        self.internal.rounding
    }
    pub fn color(&self) -> Color {
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

emit_nk_command!(CommandRectMultiColor, nk_command_rect_multi_color);
impl CommandRectMultiColor {
    pub fn left(&self) -> Color {
        self.internal.left
    }
    pub fn top(&self) -> Color {
        self.internal.top
    }
    pub fn right(&self) -> Color {
        self.internal.right
    }
    pub fn bottom(&self) -> Color {
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

emit_nk_command!(CommandTriangle, nk_command_triangle);
impl CommandTriangle {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn a(&self) -> Vec2i {
        self.internal.a
    }
    pub fn b(&self) -> Vec2i {
        self.internal.b
    }
    pub fn c(&self) -> Vec2i {
        self.internal.c
    }
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandTriangleFilled, nk_command_triangle_filled);
impl CommandTriangleFilled {
    pub fn a(&self) -> Vec2i {
        self.internal.a
    }
    pub fn b(&self) -> Vec2i {
        self.internal.b
    }
    pub fn c(&self) -> Vec2i {
        self.internal.c
    }
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandCircle, nk_command_circle);
impl CommandCircle {
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
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandCircleFilled, nk_command_circle_filled);
impl CommandCircleFilled {
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
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandArc, nk_command_arc);
impl CommandArc {
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
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandArcFilled, nk_command_arc_filled);
impl CommandArcFilled {
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
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandPolygon, nk_command_polygon);
impl CommandPolygon {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn points(&self) -> &[Vec2i] {
        unsafe { ::std::slice::from_raw_parts(self.internal.points.as_ptr(), self.internal.point_count as usize) }
    }
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandPolygonFilled, nk_command_polygon_filled);
impl CommandPolygonFilled {
    pub fn points(&self) -> &[Vec2i] {
        unsafe { ::std::slice::from_raw_parts(self.internal.points.as_ptr(), self.internal.point_count as usize) }
    }
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandPolyline, nk_command_polyline);
impl CommandPolyline {
    pub fn line_thickness(&self) -> u16 {
        self.internal.line_thickness
    }
    pub fn points(&self) -> &[Vec2i] {
        unsafe { ::std::slice::from_raw_parts(self.internal.points.as_ptr(), self.internal.point_count as usize) }
    }
    pub fn color(&self) -> Color {
        self.internal.color
    }
}

emit_nk_command!(CommandImage, nk_command_image);
impl CommandImage {
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
    pub fn col(&self) -> Color {
        self.internal.col
    }
    pub fn img(&self) -> Image {
        Image { internal: self.internal.img }
    }
}

emit_nk_command!(CommandText, nk_command_text);
impl CommandText {
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
        unsafe { ::std::slice::from_raw_parts(self.internal.string.as_ptr() as *const u8, self.internal.length as usize) }
    }
    pub fn background(&self) -> Color {
        self.internal.background
    }
    pub fn foreground(&self) -> Color {
        self.internal.foreground
    }
    pub fn font(&self) -> &UserFont {
        unsafe { ::std::mem::transmute(self.internal.font) }
    }
}

// =============================================================================================

wrapper_type!(CommandBuffer, nk_command_buffer);

impl CommandBuffer {
    pub fn stroke_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, line_thickness: f32, color: Color) {
        unsafe {
            nk_stroke_line(&mut self.internal, x0, y0, x1, y1, line_thickness, color);
        }
    }

    pub fn stroke_curve(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, line_thickness: f32, color: Color) {
        unsafe {
            nk_stroke_curve(&mut self.internal, x0, y0, x1, y1, x2, y2, x3, y3, line_thickness, color);
        }
    }

    pub fn stroke_rect(&mut self, bounds: Rect, rounding: f32, line_thickness: f32, color: Color) {
        unsafe {
            nk_stroke_rect(&mut self.internal, bounds, rounding, line_thickness, color);
        }
    }

    pub fn stroke_circle(&mut self, arg2: Rect, line_thickness: f32, color: Color) {
        unsafe {
            nk_stroke_circle(&mut self.internal, arg2, line_thickness, color);
        }
    }

    pub fn stroke_arc(&mut self, cx: f32, cy: f32, radius: f32, a_min: f32, a_max: f32, line_thickness: f32, color: Color) {
        unsafe {
            nk_stroke_arc(&mut self.internal, cx, cy, radius, a_min, a_max, line_thickness, color);
        }
    }

    pub fn stroke_triangle(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, line_thichness: f32, color: Color) {
        unsafe {
            nk_stroke_triangle(&mut self.internal, x0, y0, x1, y1, x2, y2, line_thichness, color);
        }
    }

    pub fn stroke_polyline(&mut self, points: &mut [f32], line_thickness: f32, color: Color) {
        unsafe {
            nk_stroke_polyline(&mut self.internal, &mut points[0] as *mut f32, points.len() as ::std::os::raw::c_int, line_thickness, color);
        }
    }

    pub fn stroke_polygon(&mut self, points: &mut [f32], line_thickness: f32, color: Color) {
        unsafe {
            nk_stroke_polygon(&mut self.internal, &mut points[0] as *mut f32, points.len() as ::std::os::raw::c_int, line_thickness, color);
        }
    }

    pub fn fill_rect(&mut self, arg2: Rect, rounding: f32, color: Color) {
        unsafe {
            nk_fill_rect(&mut self.internal, arg2, rounding, color);
        }
    }

    pub fn fill_rect_multi_color(&mut self, arg2: Rect, left: Color, top: Color, right: Color, bottom: Color) {
        unsafe {
            nk_fill_rect_multi_color(&mut self.internal, arg2, left, top, right, bottom);
        }
    }

    pub fn fill_circle(&mut self, arg2: Rect, color: Color) {
        unsafe {
            nk_fill_circle(&mut self.internal, arg2, color);
        }
    }

    pub fn fill_arc(&mut self, cx: f32, cy: f32, radius: f32, a_min: f32, a_max: f32, color: Color) {
        unsafe {
            nk_fill_arc(&mut self.internal, cx, cy, radius, a_min, a_max, color);
        }
    }

    pub fn fill_triangle(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
        unsafe {
            nk_fill_triangle(&mut self.internal, x0, y0, x1, y1, x2, y2, color);
        }
    }

    pub fn fill_polygon(&mut self, points: &mut [f32], color: Color) {
        unsafe {
            nk_fill_polygon(&mut self.internal, &mut points[0] as *mut f32, points.len() as ::std::os::raw::c_int, color);
        }
    }

    pub fn push_scissor(&mut self, arg2: Rect) {
        unsafe {
            nk_push_scissor(&mut self.internal, arg2);
        }
    }

    pub fn draw_image(&mut self, arg2: Rect, arg3: &Image, arg4: Color) {
        unsafe {
            nk_draw_image(&mut self.internal, arg2, &arg3.internal as *const nk_image, arg4);
        }
    }

    pub fn draw_text(&mut self, arg2: Rect, text: &str, arg3: &UserFont, arg4: Color, arg5: Color) {
        unsafe {
            nk_draw_text(&mut self.internal, arg2, text.as_ptr() as *const i8, text.as_bytes().len() as ::std::os::raw::c_int, &arg3.internal, arg4, arg5);
        }
    }
}

// =============================================================================================

pub fn color_rgb(r: i32, g: i32, b: i32) -> Color {
    unsafe { nk_rgb(r, g, b) }
}

pub fn color_rgb_iv(rgb: &i32) -> Color {
    unsafe { nk_rgb_iv(rgb as *const i32) }
}

pub fn color_rgb_bv(rgb: &u8) -> Color {
    unsafe { nk_rgb_bv(rgb as *const u8) }
}

pub fn color_rgb_fv(rgb: &f32) -> Color {
    unsafe { nk_rgb_fv(rgb as *const f32) }
}

pub fn color_rgb_f(r: f32, g: f32, b: f32) -> Color {
    unsafe { nk_rgb_f(r, g, b) }
}

pub fn color_rgb_hex(rgb: String) -> Color {
    unsafe { nk_rgb_hex(rgb.as_ptr()) }
}

pub fn color_rgba(r: i32, g: i32, b: i32, a: i32) -> Color {
    unsafe { nk_rgba(r, g, b, a) }
}

pub fn color_rgba_u32(rgba: u32) -> Color {
    unsafe { nk_rgba_u32(rgba) }
}

pub fn color_rgba_iv(rgba: &i32) -> Color {
    unsafe { nk_rgba_iv(rgba as *const i32) }
}

pub fn color_rgba_bv(rgb: &u8) -> Color {
    unsafe { nk_rgba_bv(rgb as *const u8) }
}

pub fn color_rgba_fv(rgb: &f32) -> Color {
    unsafe { nk_rgba_fv(rgb as *const f32) }
}

pub fn color_rgba_f(r: f32, g: f32, b: f32, a: f32) -> Color {
    unsafe { nk_rgba_f(r, g, b, a) }
}

pub fn color_rgba_hex(rgba: String) -> Color {
    unsafe { nk_rgba_hex(rgba.as_ptr()) }
}

pub fn color_hsv(h: i32, s: i32, v: i32) -> Color {
    unsafe { nk_hsv(h, s, v) }
}

pub fn color_hsv_iv(hsv: &i32) -> Color {
    unsafe { nk_hsv_iv(hsv as *const i32) }
}

pub fn color_hsv_bv(hsv: &u8) -> Color {
    unsafe { nk_hsv_bv(hsv as *const u8) }
}

pub fn color_hsv_fv(hsv: &f32) -> Color {
    unsafe { nk_hsv_fv(hsv as *const f32) }
}

pub fn color_hsv_f(h: f32, s: f32, v: f32) -> Color {
    unsafe { nk_hsv_f(h, s, v) }
}

pub fn color_hsva(h: i32, s: i32, v: i32, a: i32) -> Color {
    unsafe { nk_hsva(h, s, v, a) }
}

pub fn color_hsva_iv(hsva: &i32) -> Color {
    unsafe { nk_hsva_iv(hsva as *const i32) }
}

pub fn color_hsva_bv(hsv: &u8) -> Color {
    unsafe { nk_hsva_bv(hsv as *const u8) }
}

pub fn color_hsva_fv(hsv: &f32) -> Color {
    unsafe { nk_hsva_fv(hsv as *const f32) }
}

pub fn color_hsva_f(h: f32, s: f32, v: f32, a: f32) -> Color {
    unsafe { nk_hsva_f(h, s, v, a) }
}

pub fn style_get_color_by_name(c: StyleColor) -> Cow<'static, str> {
    unsafe {
        // String::from_bytes_unchecked()
        // CString::from_raw(nk_style_get_color_by_name(c))
        ::std::ffi::CStr::from_ptr(nk_style_get_color_by_name(c)).to_string_lossy()
    }
}

// =============================================================================================

wrapper_type!(Image, nk_image);

impl Image {
    pub fn with_id(id: i32) -> Image {
        Image { internal: unsafe { nk_image_id(id) } }
    }

    pub unsafe fn with_ptr(ptr: *mut c_void) -> Image {
        Image { internal: nk_image_ptr(ptr) }
    }

    pub fn id(&mut self) -> i32 {
        unsafe { self.internal.handle.id }
    }

    pub fn ptr(&mut self) -> *mut c_void {
        unsafe { self.internal.handle.ptr }
    }
}

// =============================================================================================

wrapper_type!(FontGlyph, nk_font_glyph);

impl FontGlyph {
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

wrapper_type!(Font, nk_font);

impl Font {
    pub fn find_glyph(&self, unicode: char) -> &FontGlyph {
        unsafe { ::std::mem::transmute(nk_font_find_glyph(&self.internal as *const _ as *mut nk_font, unicode as u32)) }
    }

    pub fn handle(&self) -> &UserFont {
        unsafe { ::std::mem::transmute(&self.internal.handle as *const _ as *mut nk_user_font) }
    }
}

// =============================================================================================

wrapper_type!(UserFont, nk_user_font);

impl UserFont {
    pub unsafe fn userdata_ptr(&self) -> Handle {
        Handle::from_ptr(self.internal.userdata.ptr)
    }

    pub unsafe fn userdata_id(&self) -> Handle {
        Handle::from_id(self.internal.userdata.id)
    }
}

// =============================================================================================

fn raw_glyph_ranges_to_safe<'a>(arg: *const nk_rune) -> &'a [(u32, u32)] {
    unsafe {
        let len32 = (::std::mem::size_of::<(u32, u32)>() / ::std::mem::size_of::<u32>()) as isize;

        let mut raw2 = arg;

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
