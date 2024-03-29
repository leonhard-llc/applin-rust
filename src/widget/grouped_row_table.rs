#![allow(clippy::many_single_char_names)]
use crate::widget::{RowGroupList, RowList};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GroupedRowTable {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
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
            id: String::new(),
            row_groups: row_groups.into().to_vec(),
            spacing: 0,
        }
    }

    #[must_use]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = id.as_ref().to_string();
        self
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
