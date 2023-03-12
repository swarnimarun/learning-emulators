//! shaders are required to draw pretty much anything meaningful to the screen
//! WIP!

use crate::renderer;

/// simple interface for handlng all the shader operations like loading,
/// storing, compiling and etc.
pub struct Shader {
    pub(crate) shader_module: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(renderer: &renderer::Renderer, source: impl AsRef<str>) -> Self {
        let shader_source = wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(source.as_ref()));
        Self {
            shader_module: renderer
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: shader_source,
                }),
        }
    }
}
