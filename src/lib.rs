pub extern crate sgl as gl;

pub mod buffer;
pub mod layout;
pub mod program;

pub trait Bindable {
    fn bind(&self);
    fn unbind(&self);
}
