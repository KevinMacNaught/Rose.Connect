use crate::theme::ThemeColors;
use gpui::SharedString;
use super::KanbanCard;

#[derive(Clone, Copy)]
pub enum ColumnStatus {
    Backlog,
    Todo,
    InProgress,
    Done,
}

impl ColumnStatus {
    pub fn accent_color(&self, theme: &ThemeColors) -> u32 {
        match self {
            ColumnStatus::Backlog => theme.text_muted,
            ColumnStatus::Todo => theme.text_accent,
            ColumnStatus::InProgress => theme.status_warning,
            ColumnStatus::Done => theme.status_success,
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx {
            0 => ColumnStatus::Backlog,
            1 => ColumnStatus::Todo,
            2 => ColumnStatus::InProgress,
            _ => ColumnStatus::Done,
        }
    }
}

pub struct KanbanColumn {
    pub title: SharedString,
    pub status: ColumnStatus,
    pub cards: Vec<KanbanCard>,
}
