use std::cell::RefCell;
use std::io::Result;
use std::ops::{Deref, DerefMut};
use std::time::Instant;

use glium::{Display, Surface};
use glium::glutin::{self, Event, WindowEvent};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::borrow::Borrow;

type UiCallback = RefCell<dyn FnOnce(&mut Ui) -> ()>;
type EventCallback = RefCell<dyn FnOnce(&mut Event) -> ()>;

pub struct App<'a> {
  pub run: bool,
  pub imgui: Context,
  pub events: glutin::EventsLoop,
  pub display: Display,
  pub platform: WinitPlatform,
  pub renderer: Renderer,

  handle_ui: Option<Box<dyn FnMut(&mut Ui) + 'a>>,
  handle_event: Option<Box<dyn FnMut(&Event) + 'a>>,
  handle_update: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a> App<'a> {
  pub fn create(title: &str, width: f32, height: f32) -> Result<Self> {
    let events = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let window_builder = glutin::WindowBuilder::new()
      .with_title(title)
      .with_resizable(false)
      .with_dimensions(
        glutin::dpi::LogicalSize::new(
          width as f64,
          height as f64
        )
      );

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
      run: true,
      imgui,
      events,
      display,
      platform,
      renderer,

      handle_ui: None,
      handle_event: None,
      handle_update: None,
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
      mut run,
      mut events,
      display,
      mut imgui,
      mut platform,
      mut renderer,

      mut handle_ui,
      mut handle_event,
      mut handle_update,
      ..
    } = self;

    let window = display.gl_window();
    let window = window.window();
    let mut last_frame = Instant::now();

    while run {
      events.poll_events(|event| {
        platform.handle_event(imgui.io_mut(), &window, &event);

        if let Some(handle_event) = handle_event.as_mut() {
          handle_event(&event);
        }

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

      if let Some(handle_ui) = handle_ui.as_mut() {
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

      if let Some(handle_update) = handle_update.as_mut() {
        handle_update();
      }
//      if let Some(handle_update) = handle_update.borrow() {
//        handle_update.deref()();
//      }
    }

    Ok(())
  }

  pub fn on_ui(&mut self, handler: impl FnMut(&mut Ui) -> () + 'a) -> &mut Self {
    self.handle_ui = Some(Box::new(handler));
    self
  }

  pub fn on_event(&mut self, handler: impl FnMut(&Event) -> () + 'a) -> &mut Self {
    self.handle_event = Some(Box::new(handler));
    self
  }

  pub fn on_update(&mut self, handler: impl FnMut() -> () + 'a) -> &mut Self {
    self.handle_update = Some(Box::new(handler));
    self
  }
}