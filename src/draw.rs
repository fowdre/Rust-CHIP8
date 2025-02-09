use raylib::prelude::*;

pub struct MonoChromeDisplay {
    position: Vector2,
    dimensions: Vector2,
    scaled_dimensions: Vector2,
    scale: f32,
    texture: Option<Texture2D>,
    primary_color: Color,
    secondary_color: Color,
}

impl MonoChromeDisplay {
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

    pub fn draw(&mut self, handle: &mut RaylibDrawHandle<'_>) {
        if let Some(texture) = &self.texture {
            handle.draw_texture_ex(
                texture,
                Vector2::new(self.position.x, self.position.y),
                0.0,
                self.scale,
                Color::WHITE,
            );
        }
    }

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

    pub const fn get_position(&self) -> Vector2 {
        self.position
    }

    pub const fn get_dimensions(&self) -> Vector2 {
        self.scaled_dimensions
    }

    pub fn center(&mut self, handle: &RaylibHandle) {
        self.position = Vector2::new(
            (handle.get_screen_width() as f32 - self.scaled_dimensions.x) / 2.0,
            (handle.get_screen_height() as f32 - self.scaled_dimensions.y) / 2.0,
        );
    }
}
