//! SmartTable helpers and data binding extensions.

use fltk::prelude::*;
use fltk_table::{SmartTable, TableOpts};
use ndarray::Array2;

// ==========================================
// 1. Extension Trait (新版架构)
// ==========================================

pub trait TableBindingExt {
    /// Fills the table with a 2D slice. First row is typically headers.
    fn bind_data<T, R>(&mut self, data: &[R])
    where
        T: AsRef<str>,
        R: AsRef<[T]>;
}

// 注意：这里必须针对 SmartTable 实现，因为原生的 Table 没有 set_cell_value
impl TableBindingExt for SmartTable {
    fn bind_data<T, R>(&mut self, data: &[R])
    where
        T: AsRef<str>,
        R: AsRef<[T]>,
    {
        if data.is_empty() {
            return;
        }
        
        let nrows = data.len() as i32;
        let ncols = data[0].as_ref().len() as i32;

        // 动态调整表格行列数
        self.set_opts(TableOpts {
            rows: nrows,
            cols: ncols,
            editable: false,
            ..Default::default()
        });

        // 填充数据
        for (r, row) in data.iter().enumerate() {
            for (c, cell) in row.as_ref().iter().enumerate() {
                self.set_cell_value(r as i32, c as i32, cell.as_ref());
            }
        }
        
        // 自动分配列宽并重绘
        TableHelper::auto_col_widths(self, ncols);
        self.redraw();
    }
}

// ==========================================
// 2. 基础辅助工具 (原版 0.1.0 核心功能)
// ==========================================

pub struct TableHelper;

impl TableHelper {
    /// A SmartTable configured as read-only, no headers, columns resizable.
    pub fn new_standard(x: i32, y: i32, w: i32, h: i32) -> SmartTable {
        let mut t = SmartTable::new(x, y, w, h, "").with_opts(TableOpts {
            rows: 0,
            cols: 0,
            editable: false,
            ..Default::default()
        });
        t.set_col_resize(true);
        t.set_col_header(false);
        t.set_row_header(false);
        t.end();
        t
    }

    /// Fill a SmartTable from an `Array2<String>` and distribute column widths.
    pub fn fill_from_array(table: &mut SmartTable, data: &Array2<String>) {
        let nrows = data.nrows() as i32;
        let ncols = data.ncols() as i32;
        let rows = nrows.max(1);
        let cols = ncols.max(1);

        table.set_opts(TableOpts {
            rows,
            cols,
            editable: false,
            ..Default::default()
        });

        for r in 0..nrows {
            for c in 0..ncols {
                let v = data[[r as usize, c as usize]].as_str();
                table.set_cell_value(r, c, v);
            }
        }

        Self::auto_col_widths(table, cols);
        table.redraw();
    }

    /// Fill a SmartTable from an `Array2<String>`, treating the first row
    /// as headers. It automatically distributes column widths.
    /// Fill a SmartTable from an `Array2<String>`, treating the first row
    /// as the native FLTK column headers.
    pub fn fill_from_array_with_header(table: &mut SmartTable, data: &Array2<String>) {
        let nrows = data.nrows() as i32;
        let ncols = data.ncols() as i32;
        
        if nrows == 0 || ncols == 0 {
            return;
        }

        // 开启 FLTK 原生的列头显示（固定不滚动）
        table.set_col_header(true);
        // 数据行数 = 总行数 - 表头行数
        let data_rows = nrows - 1;

        table.set_opts(TableOpts {
            rows: data_rows,
            cols: ncols,
            editable: false,
            ..Default::default()
        });

        // 1. 填入表头 (取 Array2 的第 0 行)
        for c in 0..ncols {
            let header_text = data[[0, c as usize]].as_str();
            // FLTK 设置列名的方法
            table.set_col_header_value(c, header_text);
        }

        // 2. 填入数据 (从 Array2 的第 1 行开始)
        for r in 1..nrows {
            for c in 0..ncols {
                let v = data[[r as usize, c as usize]].as_str();
                // 注意这里的行索引需要减 1
                table.set_cell_value(r - 1, c, v);
            }
        }

        // 3. 自动分配列宽并重绘
        Self::auto_col_widths(table, ncols);
        table.redraw();
    }

    /// Set the first row to the given headers.
    pub fn set_headers(table: &mut SmartTable, headers: &[&str]) {
        for (i, h) in headers.iter().enumerate() {
            table.set_cell_value(0, i as i32, h);
        }
    }

    /// Clear the table.
    pub fn clear(table: &mut SmartTable) {
        table.set_opts(TableOpts {
            rows: 0,
            cols: 0,
            editable: false,
            ..Default::default()
        });
        table.redraw();
    }

    /// Distribute the table width evenly across `cols` columns.
    pub fn auto_col_widths(table: &mut SmartTable, cols: i32) {
        if cols <= 0 {
            return;
        }
        let w = table.width();
        let cw = w / cols;
        for c in 0..cols {
            table.set_col_width(c, cw);
        }
    }
}