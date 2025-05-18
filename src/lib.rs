#[cfg(feature = "sync")]
pub(crate) type Xrc<T> = std::sync::Arc<T>;

#[cfg(not(feature = "sync"))]
pub(crate) type Xrc<T> = std::rc::Rc<T>;

pub mod ast;

#[cfg(feature = "parser")]
pub mod parser;

#[cfg(feature = "printer")]
pub mod printer;

#[cfg(feature = "html-printer")]
pub mod html_printer;
