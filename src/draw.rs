//! Drawing utilities for the emulator.
//!
//! This module contains the `MonoChromeDisplay` struct, which represents a simple monochrome display.

use raylib::prelude::*;

/// A monochrome display
///
/// Represents a simple display with a fixed **primary** and **secondary** color.
/// It supports scaling, centering, and updating pixel data.
#[derive(Debug)]
pub struct MonoChromeDisplay {
    /// The position of the display on the screen (top-left corner).
    position: Vector2,

    /// The original dimensions (width, height) of the display.
    dimensions: Vector2,

    /// The scaled dimensions (width, height) after applying the scale factor.
    scaled_dimensions: Vector2,

    /// The scale factor applied to the display.
    scale: f32,

    /// The texture representing the display pixels (if available).
    texture: Option<Texture2D>,

    /// The primary color used for active pixels (on).
    primary_color: Color,

    /// The secondary color used for inactive pixels (off).
    secondary_color: Color,
}

impl MonoChromeDisplay {
    /// Creates a new `MonoChromeDisplay` with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `position` - The initial position of the display (top-left corner).
    /// * `dimensions` - The width and height of the display before scaling.
    /// * `primary_color` - The color representing active pixels (on).
    /// * `secondary_color` - The color representing inactive pixels (off).
    /// * `scale` - A scaling factor to enlarge or shrink the display.
    ///
    /// # Returns
    ///
    /// A new instance of `MonoChromeDisplay`.
    ///
    /// # Example
    /// ```
    /// let display = MonoChromeDisplay::new(
    ///     Vector2::new(0.0, 0.0),
    ///     Vector2::new(64.0, 32.0),
    ///     Color::BLACK,
    ///     Color::WHITE,
    ///     1.0
    /// );
    /// ```
    pub fn new(
        position: Vector2,
        dimensions: Vector2,
        primary_color: Color,
        secondary_color: Color,
        scale: f32,
    ) -> Self {
        Self {
            position,
            dimensions,
            scaled_dimensions: dimensions * scale,
            scale,
            texture: None,
            primary_color,
            secondary_color,
        }
    }

    /// Draws the display onto the screen using Raylib.
    ///
    /// # Arguments
    ///
    /// * `handle` - A mutable reference to `RaylibDrawHandle` for rendering.
    ///
    /// This function renders the texture onto the screen, if available.
    pub fn draw(&mut self, handle: &mut RaylibDrawHandle<'_>) {
        if let Some(texture) = &self.texture {
            handle.draw_texture_ex(texture, self.position, 0.0, self.scale, Color::WHITE);
        }
    }

    /// Updates the display texture with new pixel data.
    ///
    /// # Arguments
    ///
    /// * `rl_handle` - A mutable reference to the Raylib handle.
    /// * `rl_thread` - A reference to the Raylib thread.
    /// * `pixels` - A boolean array representing pixel states (`true` for primary color, `false` for secondary color).
    ///
    /// This function updates the texture based on the `pixels` array. It creates a new texture if none exists.
    pub fn update(
        &mut self,
        rl_handle: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        pixels: &[bool],
    ) {
        let (width, height) = (self.dimensions.x as usize, self.dimensions.y as usize);
        let mut pixel_data = vec![0; width * height * 4];

        for (i, pixel) in pixels.iter().enumerate() {
            if *pixel {
                pixel_data[i * 4] = self.primary_color.r;
                pixel_data[i * 4 + 1] = self.primary_color.g;
                pixel_data[i * 4 + 2] = self.primary_color.b;
                pixel_data[i * 4 + 3] = 255;
            } else {
                pixel_data[i * 4] = self.secondary_color.r;
                pixel_data[i * 4 + 1] = self.secondary_color.g;
                pixel_data[i * 4 + 2] = self.secondary_color.b;
                pixel_data[i * 4 + 3] = 255;
            }
        }

        if let Some(texture) = &mut self.texture {
            texture.update_texture(&pixel_data);
        } else {
            self.texture = Some(
                rl_handle
                    .load_texture_from_image(
                        rl_thread,
                        &Image::gen_image_color(width as i32, height as i32, Color::BLANK),
                    )
                    .expect("Could not load texture"),
            );
        }
    }

    /// Centers the display within the current screen dimensions.
    ///
    /// # Arguments
    ///
    /// * `handle` - A reference to the Raylib handle.
    ///
    /// This function adjusts the `position` to ensure the display is centered on the screen.
    pub fn center(&mut self, handle: &RaylibHandle) {
        self.position = Vector2::new(
            (handle.get_screen_width() as f32 - self.scaled_dimensions.x) / 2.0,
            (handle.get_screen_height() as f32 - self.scaled_dimensions.y) / 2.0,
        );
    }
}
