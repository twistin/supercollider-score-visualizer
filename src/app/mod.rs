// Submódulo que gestiona la lógica de actualización del estado visual
pub mod update;
// Submódulo que contiene la función de renderizado de la vista
pub mod view;

// Reexportamos las funciones principales para facilitar su uso desde otros módulos
pub use update::update;
pub use view::view;
