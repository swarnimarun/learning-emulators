use color_eyre::eyre::Result;
use pixels::{Pixels, SurfaceTexture};

use bit_field::BitField;

/// FrameBuffer
#[derive(Debug)]
pub struct FrameBuffer {
    p: pixels::Pixels,
    width: u32,
    height: u32,
}

impl FrameBuffer {
    pub fn draw_at(&mut self, x: u8, y: u8, ns: &[u8]) -> bool {
        let mut f = false;
        let frame = self.p.frame_mut();
        let width = self.width;
        for (rowIdx, row) in frame.chunks_exact_mut(width as usize * 4).enumerate() {
            for (colIdx, pixel) in row.chunks_exact_mut(4).enumerate() {
                if colIdx >= x.into()
                    && (colIdx < x as usize + 8)
                    && rowIdx >= y.into()
                    && (rowIdx < y as usize + ns.len())
                {
                    // pixel_draw()
                }
                //for xi in 0..8 {
                //    f = f || collide_get(&mut x.data[k as usize + i], *n, j, xi);
                //}
            }
        }
        self.p.render();
        f
    }
    pub fn new(window: &winit::window::Window) -> Result<Self> {
        let width = 32;
        let height = 64;

        let size = window.inner_size();

        Ok(FrameBuffer {
            p: Pixels::new(
                width,
                height,
                SurfaceTexture::new(size.width, size.height, &window),
            )?,
            width,
            height,
        })
    }
    pub fn clear(&mut self) {
        self.p.clear_color(pixels::wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        });
    }
}
