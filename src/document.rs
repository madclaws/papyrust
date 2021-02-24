use crate::Row;
use std::fs;
#[derive(Default)]
pub struct Document {
  rows: Vec<Row>
}

impl Document {
  ///
  /// # Errors
  pub fn open(filename: &str) -> Result<Self, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut init_rows = vec![];
    for value in contents.lines() {
      init_rows.push(Row::from(value));
    }
    Ok(Self {
      rows: init_rows
    })
  }

  pub fn get_row(&self, index: usize) -> Option<&Row>{
    self.rows.get(index)
  }

  pub fn is_empty(&self) -> bool {
    self.rows.is_empty()
  }

}