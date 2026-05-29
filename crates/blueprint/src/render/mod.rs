mod palette;
mod raster;

pub use palette::size_for;

use std::io::Cursor;
use std::path::Path;

use image::{ImageFormat, RgbaImage};
use tracing::{info, instrument};

use crate::entity::Entity;
use crate::error::{Error, Result};
use crate::world::{Direction, World};

use self::palette::{Color, color_for, darken, lighten};
use self::raster::{draw_grid, fill_background, fill_rect, stroke_rect};

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub tile_size: u32,
    pub padding_tiles: u32,
    pub background: Color,
    pub grid: Option<Color>,
    pub outline: Option<Color>,
    pub direction_notch: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            tile_size: 16,
            padding_tiles: 2,
            background: [28, 30, 36, 255],
            grid: Some([48, 50, 56, 255]),
            outline: Some([14, 15, 18, 255]),
            direction_notch: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderedWorld {
    image: RgbaImage,
}

impl RenderedWorld {
    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn image(&self) -> &RgbaImage {
        &self.image
    }

    pub fn into_image(self) -> RgbaImage {
        self.image
    }

    pub fn export_as_png(&self, path: impl AsRef<Path>) -> Result<()> {
        self.image
            .save_with_format(path, ImageFormat::Png)
            .map_err(|e| Error::Render(e.to_string()))
    }

    pub fn export_as_png_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.image
            .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .map_err(|e| Error::Render(e.to_string()))?;
        Ok(buf)
    }
}

impl World {
    pub fn render(&self) -> RenderedWorld {
        self.render_with(&RenderConfig::default())
    }

    #[instrument(level = "debug", skip_all, fields(entities = self.len()))]
    pub fn render_with(&self, config: &RenderConfig) -> RenderedWorld {
        let bounds = compute_bounds(self);
        let (img_w, img_h) = image_dimensions(&bounds, config);
        let mut image = RgbaImage::new(img_w, img_h);
        fill_background(&mut image, config.background);
        if let Some(grid) = config.grid {
            draw_grid(&mut image, config.tile_size, grid);
        }
        for entity in self.entities() {
            draw_entity(&mut image, entity, &bounds, config);
        }
        info!(width = img_w, height = img_h, "rendered world");
        RenderedWorld { image }
    }
}

#[derive(Debug, Clone, Copy)]
struct WorldBounds {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl WorldBounds {
    fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    fn height(&self) -> f64 {
        self.max_y - self.min_y
    }
}

fn compute_bounds(world: &World) -> WorldBounds {
    let mut bounds = WorldBounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 1.0,
        max_y: 1.0,
    };
    let mut initialised = false;
    for entity in world.entities() {
        let (w, h) = size_for(entity);
        let x0 = entity.position.x - w / 2.0;
        let y0 = entity.position.y - h / 2.0;
        let x1 = entity.position.x + w / 2.0;
        let y1 = entity.position.y + h / 2.0;
        if !initialised {
            bounds = WorldBounds {
                min_x: x0,
                min_y: y0,
                max_x: x1,
                max_y: y1,
            };
            initialised = true;
        } else {
            bounds.min_x = bounds.min_x.min(x0);
            bounds.min_y = bounds.min_y.min(y0);
            bounds.max_x = bounds.max_x.max(x1);
            bounds.max_y = bounds.max_y.max(y1);
        }
    }
    bounds
}

fn image_dimensions(bounds: &WorldBounds, config: &RenderConfig) -> (u32, u32) {
    let pad = config.padding_tiles as f64;
    let w = (bounds.width() + pad * 2.0).max(1.0);
    let h = (bounds.height() + pad * 2.0).max(1.0);
    let px_w = (w * config.tile_size as f64).ceil() as u32;
    let px_h = (h * config.tile_size as f64).ceil() as u32;
    (px_w.max(1), px_h.max(1))
}

fn draw_entity(image: &mut RgbaImage, entity: &Entity, bounds: &WorldBounds, config: &RenderConfig) {
    let (w, h) = size_for(entity);
    let color = color_for(entity);
    let (x0, y0, x1, y1) = entity_pixels(entity, bounds, config, w, h);
    fill_rect(image, x0, y0, x1, y1, color);
    if let Some(outline) = config.outline {
        stroke_rect(image, x0, y0, x1, y1, outline);
    }
    if config.direction_notch {
        draw_direction_notch(image, x0, y0, x1, y1, entity.direction, color);
    }
}

fn entity_pixels(
    entity: &Entity,
    bounds: &WorldBounds,
    config: &RenderConfig,
    w: f64,
    h: f64,
) -> (i32, i32, i32, i32) {
    let pad = config.padding_tiles as f64;
    let ts = config.tile_size as f64;
    let cx = (entity.position.x - bounds.min_x + pad) * ts;
    let cy = (entity.position.y - bounds.min_y + pad) * ts;
    let hw = w * ts / 2.0;
    let hh = h * ts / 2.0;
    (
        (cx - hw).round() as i32,
        (cy - hh).round() as i32,
        (cx + hw).round() as i32,
        (cy + hh).round() as i32,
    )
}

fn draw_direction_notch(
    image: &mut RgbaImage,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    direction: Direction,
    base: Color,
) {
    let notch = lighten(base, 60);
    let shadow = darken(base, 50);
    let thickness = ((x1 - x0).min(y1 - y0).max(2) / 5).max(1);
    let (nx0, ny0, nx1, ny1) = match cardinal(direction) {
        Cardinal::North => (x0, y0, x1, y0 + thickness),
        Cardinal::East => (x1 - thickness, y0, x1, y1),
        Cardinal::South => (x0, y1 - thickness, x1, y1),
        Cardinal::West => (x0, y0, x0 + thickness, y1),
    };
    fill_rect(image, nx0, ny0, nx1, ny1, notch);
    stroke_rect(image, nx0, ny0, nx1, ny1, shadow);
}

#[derive(Debug, Clone, Copy)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

fn cardinal(d: Direction) -> Cardinal {
    match d.as_u8() {
        15 | 0 | 1 => Cardinal::North,
        2..=5 => Cardinal::East,
        6..=9 => Cardinal::South,
        _ => Cardinal::West,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{AssemblingMachine, Entity, Inserter, TransportBelt};

    fn populated_world() -> World {
        let mut w = World::new();
        w.add(Entity::new("assembling-machine-2", (3.5, 3.5), AssemblingMachine::default()));
        w.add(Entity::new("fast-inserter", (5.5, 3.5), Inserter::default()).with_direction(Direction::East));
        w.add(Entity::new("transport-belt", (6.5, 3.5), TransportBelt).with_direction(Direction::East));
        w
    }

    #[test]
    fn render_returns_nonempty_image() {
        let rendered = populated_world().render();
        assert!(rendered.width() > 0);
        assert!(rendered.height() > 0);
    }

    #[test]
    fn render_empty_world_produces_minimum_image() {
        let rendered = World::new().render();
        assert!(rendered.width() >= 1);
        assert!(rendered.height() >= 1);
    }

    #[test]
    fn exports_as_png_bytes() {
        let bytes = populated_world().render().export_as_png_bytes().unwrap();
        assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
    }
}
