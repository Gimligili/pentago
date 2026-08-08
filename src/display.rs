use macroquad::prelude::*;

pub const REFERENCE_WIDTH: f32 = 800.0;
pub const REFERENCE_HEIGHT: f32 = 600.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

impl WindowMode {
    pub const ALL: [WindowMode; 2] = [WindowMode::Windowed, WindowMode::Fullscreen];

    pub fn label(self) -> &'static str {
        match self {
            WindowMode::Windowed => "Windowed",
            WindowMode::Fullscreen => "Fullscreen",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    R800x600,
    R1024x768,
    R1280x720,
    R1600x900,
    R1920x1080,
}

impl Resolution {
    pub const ALL: [Resolution; 5] = [
        Resolution::R800x600,
        Resolution::R1024x768,
        Resolution::R1280x720,
        Resolution::R1600x900,
        Resolution::R1920x1080,
    ];

    pub fn size(self) -> (f32, f32) {
        match self {
            Resolution::R800x600 => (800.0, 600.0),
            Resolution::R1024x768 => (1024.0, 768.0),
            Resolution::R1280x720 => (1280.0, 720.0),
            Resolution::R1600x900 => (1600.0, 900.0),
            Resolution::R1920x1080 => (1920.0, 1080.0),
        }
    }

    pub fn from_size(width: f32, height: f32) -> Option<Self> {
        Self::ALL.iter().copied().find(|resolution| {
            let (w, h) = resolution.size();

            (width - w).abs() < 1.0 && (height - h).abs() < 1.0
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            Resolution::R800x600 => "800 x 600",
            Resolution::R1024x768 => "1024 x 768",
            Resolution::R1280x720 => "1280 x 720",
            Resolution::R1600x900 => "1600 x 900",
            Resolution::R1920x1080 => "1920 x 1080",
        }
    }

    pub fn apply(self) {
        let (width, height) = self.size();
        request_new_screen_size(width, height);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayContext {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub resolution: Resolution,
    pub window_mode: WindowMode,
}

impl DisplayContext {
    pub fn new() -> Self {
        let width = screen_width();
        let height = screen_height();

        let resolution = Resolution::from_size(width, height).unwrap_or(Resolution::R800x600);

        Self {
            width,
            height,
            scale: (width / REFERENCE_WIDTH).min(height / REFERENCE_HEIGHT),
            resolution,
            window_mode: WindowMode::Windowed,
        }
    }

    pub fn refresh(&mut self) {
        self.width = screen_width();
        self.height = screen_height();

        self.scale = (self.width / REFERENCE_WIDTH).min(self.height / REFERENCE_HEIGHT);
        if let Some(resolution) = Resolution::from_size(self.width, self.height) {
            self.resolution = resolution;
        }
    }

    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.resolution = resolution;

        if self.window_mode == WindowMode::Windowed {
            let (width, height) = resolution.size();
            request_new_screen_size(width, height);
        }
    }

    pub fn set_window_mode(&mut self, mode: WindowMode) {
        match mode {
            WindowMode::Windowed => {
                set_fullscreen(false);

                let (width, height) = self.resolution.size();
                request_new_screen_size(width, height);
            }

            WindowMode::Fullscreen => {
                set_fullscreen(true);
            }
        }

        self.window_mode = mode;
    }

    pub fn x(&self, reference_x: f32) -> f32 {
        reference_x * self.width / REFERENCE_WIDTH
    }

    pub fn y(&self, reference_y: f32) -> f32 {
        reference_y * self.height / REFERENCE_HEIGHT
    }
}
