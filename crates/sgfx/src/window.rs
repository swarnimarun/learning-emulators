use anyhow::Context;
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop,
    monitor::{MonitorHandle, VideoMode},
    window::Fullscreen,
};

#[derive(Debug, Default)]
pub struct WindowBuilder {
    size: Option<LogicalSize<u32>>,
    pos: Option<LogicalPosition<u32>>,
    title: Option<String>,
    resizable: bool,
    fullscreen: Option<Fullscreen>,
    transparent: bool,
    decorations: bool,
    always_on_top: bool,
    always_on_bottom: bool,
}

impl WindowBuilder {
    /// Create a new window from the builder.
    pub fn build<T>(self, event_loop: &event_loop::EventLoop<T>) -> anyhow::Result<Window> {
        Window::try_new(self, event_loop)
    }
    /// Set the window size.
    pub fn set_size(mut self, width: u32, height: u32) -> Self {
        self.size = Some(LogicalSize { width, height });
        self
    }
    /// Set the position of the window.
    pub fn set_pos(mut self, x: u32, y: u32) -> Self {
        self.pos = Some(LogicalPosition { x, y });
        self
    }
    /// Sets the title of the window.
    pub fn set_title(mut self, title: impl AsRef<str>) -> Self {
        self.title = Some(title.as_ref().to_string());
        self
    }
    /// Sets whether the window should be resizable.
    pub fn set_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
    /// Set the window as `Fullscreen::Borderless(None)`, i.e. Monitor will be selected automatically(likely primary monitor).
    pub fn set_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = if fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };
        self
    }
    /// Set the window as `Fullscreen::Borderless` with `monitor`.
    pub fn set_monitor(mut self, monitor: MonitorHandle) -> Self {
        self.fullscreen = Some(Fullscreen::Borderless(Some(monitor)));
        self
    }
    /// Set the window as `Fullscreen::Exclusive` with `video_mode`.
    pub fn set_video_mode(mut self, video_mode: VideoMode) -> Self {
        self.fullscreen = Some(Fullscreen::Exclusive(video_mode));
        self
    }
    /// Set the window to be transparent.
    pub fn set_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }
    /// Set the window to be decorated.
    pub fn set_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }
    /// Set the window to be always on top.
    pub fn set_always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }
    /// Set the window to be always on bottom.
    pub fn set_always_on_bottom(mut self, always_on_bottom: bool) -> Self {
        self.always_on_bottom = always_on_bottom;
        self
    }
}

pub struct Window {
    pub winit_window: winit::window::Window,
}

impl Window {
    pub fn try_new<T>(
        builder: WindowBuilder,
        event_loop: &event_loop::EventLoop<T>,
    ) -> anyhow::Result<Self> {
        let winit_window_builder = winit::window::WindowBuilder::new()
            .with_decorations(builder.decorations)
            .with_title(builder.title.context("no title")?)
            .with_resizable(builder.resizable)
            .with_visible(true)
            .with_transparent(builder.transparent)
            .with_window_level(if builder.always_on_top {
                winit::window::WindowLevel::AlwaysOnTop
            } else if builder.always_on_bottom {
                winit::window::WindowLevel::AlwaysOnBottom
            } else {
                winit::window::WindowLevel::Normal
            });
        let winit_window_builder = if !builder.fullscreen.is_some() {
            winit_window_builder
                .with_inner_size(builder.size.context("no size")?)
                .with_position(builder.pos.context("no position")?)
        } else {
            winit_window_builder.with_fullscreen(builder.fullscreen)
        };
        Ok(Self {
            winit_window: winit_window_builder.build(event_loop)?,
        })
    }
}
