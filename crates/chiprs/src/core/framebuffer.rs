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
    pub fn resize(&mut self, width: u32, height: u32) -> bool {
        self.p.resize_surface(width, height).is_ok()
    }
    pub fn draw_at(&mut self, x: u8, y: u8, ns: &[u8]) -> bool {
        let mut f = false;
        let frame = self.p.frame_mut();
        let width = self.width;
        for (row_idx, row) in frame.chunks_exact_mut(width as usize * 4).enumerate() {
            for (col_idx, pixel) in row.chunks_exact_mut(4).enumerate() {
                if col_idx >= x.into()
                    && (col_idx < x as usize + 8)
                    && row_idx >= y.into()
                    && (row_idx < y as usize + ns.len())
                {
                    pixel[0] = 100;
                    pixel[1] = 100;
                    pixel[2] = 100;
                    pixel[3] = 255;
                }
                //for xi in 0..8 {
                //    f = f || collide_get(&mut x.data[k as usize + i], *n, j, xi);
                //}
            }
        }
        _ = self.p.render();
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
        _ = self.p.render();
    }
}
