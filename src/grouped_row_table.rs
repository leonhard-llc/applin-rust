#![allow(clippy::many_single_char_names)]

use serde::{Deserialize, Serialize};

use crate::row_group_list::RowGroupList;
use crate::row_list::RowList;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GroupedRowTable {
    #[serde(default)]
    pub row_groups: Vec<RowList>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub spacing: u16,
}

impl GroupedRowTable {
    /// Makes a table widget with grouped rows.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new(row_groups: impl Into<RowGroupList>) -> Self {
        Self {
            row_groups: row_groups.into().to_vec(),
            spacing: 0,
        }
    }

    /// Appends a group of rows.
    #[must_use]
    pub fn with_row_group(mut self, row_list: impl Into<RowList>) -> Self {
        self.row_groups.push(row_list.into());
        self
    }

    #[must_use]
    pub fn with_spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }
}

/// Makes a table that shows groups of rows.
/// Pass a tuple of row groups.
/// For each row group, pass a tuple of rows.
/// For each row, pass a tuple of widgets or None.
pub fn grouped_row_table(row_groups: impl Into<RowGroupList>) -> GroupedRowTable {
    GroupedRowTable::new(row_groups)
}

/// Makes a table.
/// Pass a tuple of rows.
/// For each row, pass a tuple of widgets or None.
pub fn table(rows: impl Into<RowList>) -> GroupedRowTable {
    GroupedRowTable::new((rows.into(),))
}
