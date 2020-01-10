use std::io::Result;
use glium::glutin::{self, Event, WindowEvent};
use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use std::ops::{Deref, DerefMut};

pub struct App {
  pub imgui: Context,
  pub events: glutin::EventsLoop,
  pub display: Display,
  pub platform: WinitPlatform,
  pub renderer: Renderer,

  handle_ui: Option<fn(&mut Ui)>,
}

impl App {
  pub fn create() -> Result<Self> {
    let events = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let window_builder = glutin::WindowBuilder::new()
      .with_title("chip8")
      .with_dimensions(glutin::dpi::LogicalSize::new(640f64, 480f64));

    let display = Display::new(
      window_builder,
      context,
      &events,
    ).expect("Failed to initialize display");

    let mut imgui = Context::create();
    let mut platform = WinitPlatform::init(&mut imgui);
    let renderer = Renderer::init(&mut imgui, &display)
      .expect("Failed to initialize renderer");


    // TODO: Initialize graphics

    let mut result = Self {
      imgui,
      events,
      display,
      platform,
      renderer,

      handle_ui: None,
    };

    result.init_window()?;
    result.init_fonts()?;

    Ok(result)
  }

  fn init_window(&mut self) -> Result<()> {
    let gl_window = self.display.gl_window();
    let window = gl_window.window();

    self.platform.attach_window(
      self.imgui.io_mut(),
      &window,
      HiDpiMode::Rounded,
    );

    Ok(())
  }

  fn init_fonts(&mut self) -> Result<()> {
    let dpi_factor = self.platform.hidpi_factor();
    let font_size = (13.0 * dpi_factor) as f32;

    self.imgui.fonts().add_font(&[
      FontSource::DefaultFontData {
        config: Some(FontConfig {
          size_pixels: font_size,
          ..FontConfig::default()
        }),
      },
      FontSource::TtfData {
        data: include_bytes!("../../assets/Inconsolata-Regular.ttf"),
        config: Some(FontConfig {
          glyph_ranges: FontGlyphRanges::japanese(),
          rasterizer_multiply: 1.75,
          ..FontConfig::default()
        }),
        size_pixels: font_size,
      },
    ]);

    self.imgui.io_mut().font_global_scale = (1.0 / dpi_factor) as f32;

    Ok(())
  }

  pub fn run(self) -> Result<()> {
    let Self {
      mut events,
      display,
      mut imgui,
      mut platform,
      mut renderer,

      handle_ui,
      ..
    } = self;

    let window = display.gl_window();
    let window = window.window();
    let mut run = true;
    let mut last_frame = Instant::now();

    while run {
      events.poll_events(|event| {
        platform.handle_event(imgui.io_mut(), &window, &event);

        if let Event::WindowEvent { event, .. } = event {
          if let WindowEvent::CloseRequested = event {
            run = false
          }
        }
      });

      let io = imgui.io_mut();

      platform
        .prepare_frame(io, &window)
        .expect("Failed to start frame");

      let mut ui = imgui.frame();

      if let Some(handle_ui) = handle_ui {
        handle_ui(&mut ui);
      }

      let mut target = display.draw();

      target.clear_color_srgb(0.1, 0.1, 0.1, 1.0);
      platform.prepare_render(&ui, &window);
      let draw_data = ui.render();
      renderer
        .render(&mut target, draw_data)
        .expect("Rendering failed");

      target.finish().expect("Failed to swap buffers");
    }

    Ok(())
  }

  pub fn on_ui(&mut self, handler: fn(&mut Ui)) {
    self.handle_ui = Some(handler);
  }
}