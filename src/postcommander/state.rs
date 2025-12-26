use gpui::{Entity, Pixels, Point, Subscription};
use crate::components::TextInput;
use gpui_component::menu::PopupMenu;

/// Resize state for sidebar, editor, and structure panel
pub(crate) struct ResizeState {
    pub sidebar_width: f32,
    pub is_resizing_sidebar: bool,
    pub resize_sidebar_start_x: f32,
    pub resize_sidebar_start_width: f32,

    pub editor_height: f32,
    pub is_resizing_editor: bool,
    pub resize_editor_start_y: f32,
    pub resize_editor_start_height: f32,

    pub structure_panel_width: f32,
    pub is_resizing_structure: bool,
    pub resize_structure_start_x: f32,
    pub resize_structure_start_width: f32,
}

impl ResizeState {
    pub fn new(
        sidebar_width: f32,
        editor_height: f32,
        structure_panel_width: f32,
    ) -> Self {
        Self {
            sidebar_width,
            is_resizing_sidebar: false,
            resize_sidebar_start_x: 0.0,
            resize_sidebar_start_width: 0.0,
            editor_height,
            is_resizing_editor: false,
            resize_editor_start_y: 0.0,
            resize_editor_start_height: 0.0,
            structure_panel_width,
            is_resizing_structure: false,
            resize_structure_start_x: 0.0,
            resize_structure_start_width: 0.0,
        }
    }
}

/// Connection dialog form state
pub(crate) struct ConnectionDialogState {
    pub is_visible: bool,
    pub input_host: Entity<TextInput>,
    pub input_port: Entity<TextInput>,
    pub input_database: Entity<TextInput>,
    pub input_username: Entity<TextInput>,
    pub input_password: Entity<TextInput>,
}

impl ConnectionDialogState {
    pub fn new(
        input_host: Entity<TextInput>,
        input_port: Entity<TextInput>,
        input_database: Entity<TextInput>,
        input_username: Entity<TextInput>,
        input_password: Entity<TextInput>,
    ) -> Self {
        Self {
            is_visible: false,
            input_host,
            input_port,
            input_database,
            input_username,
            input_password,
        }
    }
}

pub(crate) struct SaveQueryDialogState {
    pub is_visible: bool,
    pub input_name: Entity<TextInput>,
    pub input_folder: Entity<TextInput>,
    pub input_description: Entity<TextInput>,
    pub editing_id: Option<String>,
}

impl SaveQueryDialogState {
    pub fn new(
        input_name: Entity<TextInput>,
        input_folder: Entity<TextInput>,
        input_description: Entity<TextInput>,
    ) -> Self {
        Self {
            is_visible: false,
            input_name,
            input_folder,
            input_description,
            editing_id: None,
        }
    }
}

pub(crate) struct PendingCellContextMenu {
    pub col_index: usize,
    pub column_names: Vec<String>,
    pub row_data: Vec<gpui::SharedString>,
    pub position: Point<Pixels>,
    pub table_name: Option<String>,
}

/// Active UI overlays (menus, dialogs, etc.)
pub(crate) struct ActiveOverlays {
    pub context_menu: Option<(Entity<PopupMenu>, Point<Pixels>, String, Subscription)>,
    pub export_menu: Option<(Entity<gpui_component::menu::PopupMenu>, Point<Pixels>, Subscription)>,
    pub cell_context_menu: Option<(Entity<gpui_component::menu::PopupMenu>, Point<Pixels>, Subscription)>,
    pub pending_cell_context_menu: Option<PendingCellContextMenu>,
    pub saved_query_menu: Option<(Entity<PopupMenu>, Point<Pixels>, String, Subscription)>,
}

impl Default for ActiveOverlays {
    fn default() -> Self {
        Self {
            context_menu: None,
            export_menu: None,
            cell_context_menu: None,
            pending_cell_context_menu: None,
            saved_query_menu: None,
        }
    }
}
