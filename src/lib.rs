use prettytable::Table;

pub trait SerializeToTable {
    fn serialize_row(&self, table: &mut Table);
}