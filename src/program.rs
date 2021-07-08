use std::collections::HashMap;

use gl::UniformLocation;

use crate::Bindable;

pub struct Shader {
    handle: gl::Shader,
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl::delete_shader(self.handle)
    }
}

impl Shader {
    pub fn new(kind: gl::ShaderKind, src: &str) -> Result<Self, String> {
        let handle = gl::create_shader(kind);
        gl::shader_source(handle, src);
        gl::compile_shader(handle);

        if !gl::ShaderProp::get_compile_status(handle) {
            Err(gl::get_shader_info_log(handle))
        } else {
            Ok(Self { handle })
        }
    }
}

pub struct Program {
    handle: gl::Program,
    uniform_cache: HashMap<String, UniformLocation>,
}

impl Drop for Program {
    fn drop(&mut self) {
        gl::delete_program(self.handle)
    }
}

impl Program {
    pub fn new(shaders: &[Shader]) -> Result<Self, String> {
        let handle = gl::create_program();

        for shader in shaders {
            gl::attach_shader(handle, shader.handle);
        }

        gl::link_program(handle);
        if !gl::ProgramProp::get_link_status(handle) {
            Err(gl::get_program_info_log(handle))
        } else {
            Ok(Self {
                handle,
                uniform_cache: HashMap::new(),
            })
        }
    }

    pub fn get_uniform_location(&mut self, name: &str) -> Option<UniformLocation> {
        if let Some(loc) = self.uniform_cache.get(name) {
            Some(*loc)
        } else {
            let loc = gl::get_uniform_location(self.handle, name);
            self.uniform_cache.insert(name.to_string(), loc);
            Some(loc)
        }
    }
}

impl Bindable for Program {
    fn bind(&self) {
        gl::use_program(self.handle)
    }

    fn unbind(&self) {
        todo!() // TODO:
    }
}
