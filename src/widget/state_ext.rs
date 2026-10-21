//! Extension traits to automatically save and restore UI state.

use fltk::prelude::*;
use fltk::group::{Group, Tabs};
use fltk_table::SmartTable;
use crate::core::state::State;

/// Trait for saving/restoring Tabs active index.
pub trait TabsStateExt {
    fn bind_state(self, key: &str) -> Self;
}

// 修改 state_ext.rs 中的 TabsStateExt

impl TabsStateExt for Tabs {
    fn bind_state(mut self, key: &str) -> Self {
        let key_string = key.to_string();
        let self_clone = self.clone();
        
        // 监听切换，自动保存
        self.set_callback(move |_| {
            if let Some(selected) = self_clone.value() {
                for i in 0..self_clone.children() {
                    if let Some(child) = self_clone.child(i) {
                        if child.as_widget_ptr() == selected.as_widget_ptr() {
                            State::set(&key_string, i);
                            break;
                        }
                    }
                }
            }
        });

        // 启动时自动延迟恢复
        let mut restore_tabs = self.clone();
        let restore_key = key.to_string();
        fltk::app::add_timeout3(0.1, move |_| {
            if let Some(idx) = State::get::<i32>(&restore_key) {
                if idx >= 0 && idx < restore_tabs.children() {
                    if let Some(child) = restore_tabs.child(idx) {
                        let mut grp = unsafe { Group::from_widget_ptr(child.as_widget_ptr()) };
                        let _ = restore_tabs.set_value(&mut grp);
                        restore_tabs.do_callback();
                        restore_tabs.redraw();
                    }
                }
            }
        });
        
        self
    }
}

/// Trait for saving/restoring Table column widths.
pub trait TableStateExt {
    fn bind_col_widths_state(self, key: &str) -> Self;
}

impl TableStateExt for SmartTable {
    fn bind_col_widths_state(mut self, key: &str) -> Self {
        let cols = self.cols();
        
        if let Some(saved_widths) = State::get::<Vec<i32>>(key) {
            if saved_widths.len() == cols as usize {
                for (c, w) in saved_widths.iter().enumerate() {
                    self.set_col_width(c as i32, *w);
                }
            }
        }
        self
    }
}

/// Helper to save all current table widths manually.
pub fn save_table_widths(table: &SmartTable, key: &str) {
    let mut widths = Vec::new();
    for c in 0..table.cols() {
        widths.push(table.col_width(c));
    }
    State::set(key, widths);
}