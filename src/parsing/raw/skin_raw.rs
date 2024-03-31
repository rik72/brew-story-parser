use brew_derive_macros::Raw;
use serde::Deserialize;

use super::traits::{inner_raws::InnerRaws, raw::Raw};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SkinRaw {
    pub file_name: Option<String>,
    pub name: String,
    pub font_family: Option<String>,
    pub font_size: Option<i32>,
    pub color_text_flow_normal: Option<String>,
    pub color_text_flow_hilight: Option<String>,
    pub color_text_flow_emphasis: Option<String>,
    pub color_text_flow_bg: Option<String>,
    pub color_text_flow_scrollbar: Option<String>,
    pub color_menu_bg: Option<String>,
    pub color_menu_hilight: Option<String>,
    pub color_menu_separator: Option<String>,
    pub color_window_bg: Option<String>,
    pub color_window_text: Option<String>,
    pub color_window_location_image_bg: Option<String>,
    pub color_window_button: Option<String>,
    pub color_window_hover: Option<String>,
    pub color_window_modal_bg: Option<String>,
    pub color_window_modal_border: Option<String>,
}

impl InnerRaws for SkinRaw {
    fn set_inner_raws_file_name(&mut self, _file_name: &String) {
        // NOP
    }
}
