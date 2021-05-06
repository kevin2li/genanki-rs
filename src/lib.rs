mod apkg_col;
mod apkg_schema;
mod builders;
mod builtin_models;
mod card;
mod db_entries;
mod deck;
mod model;
mod note;
mod package;
mod util;

pub use builders::{Field, Template};
pub use builtin_models::*;
pub use deck::Deck;
pub use model::Model;
pub use note::Note;
pub use package::Package;

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::Python;
    use pyo3::prelude::PyModule;

    #[test]
    fn import_anki() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        py.import("anki").unwrap();
    }
}
