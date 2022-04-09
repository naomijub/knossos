use crate::maze::grid::cell::Cell;
use crate::maze::grid::pole::Pole;
use crate::maze::{formatters::Formatter, grid::Grid};
use crate::utils::color::Color;
use crate::utils::types::Coords;
use image::{ImageBuffer, RgbImage};

use super::ImageWrapper;

pub struct Image {
    wall_width: usize,
    passage_width: usize,
    margin: usize,
    background_color: Color,
    foreground_color: Color,
}

impl Image {
    pub fn new() -> Image {
        Image {
            wall_width: 40,
            passage_width: 40,
            background_color: Color::RGB(250, 250, 250),
            foreground_color: Color::RGB(0, 0, 0),
            margin: 50,
        }
    }

    pub fn wall(mut self, width: usize) -> Self {
        self.wall_width = width;
        self
    }

    pub fn passage(mut self, width: usize) -> Self {
        self.passage_width = width;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground_color = color;
        self
    }

    pub fn margin(mut self, value: usize) -> Self {
        self.margin = value;
        self
    }

    fn cell_width(&self) -> usize {
        self.wall_width * 2 + self.passage_width
    }

    fn sizes(&self, grid: &Grid) -> (usize, usize) {
        // To calculate maze's width and height we use a simple formula that multiplies a single
        // cell width and a number of cells (in a row or column). However, since two cells
        // have a single joint wall, we do the subtraction of the joint walls from the
        // preceding width
        let maze_width = self.cell_width() * grid.width() - (grid.width() - 1) * self.wall_width;
        let maze_height = self.cell_width() * grid.height() - (grid.height() - 1) * self.wall_width;

        let image_width = maze_width + self.margin * 2;
        let image_height = maze_height + self.margin * 2;

        (image_width, image_height)
    }

    fn fill_background(&self, image: &mut RgbImage) {
        for (_, _, pixel) in image.enumerate_pixels_mut() {
            *pixel = match self.background_color {
                Color::RGB(r, g, b) => image::Rgb([r, g, b]),
            }
        }
    }

    fn draw_maze(&self, image: &mut RgbImage, grid: &Grid) {
        for (y, row) in grid.cells().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                self.draw_cell((x, y), cell, image);
            }
        }
    }

    fn draw_cell(&self, coords: Coords, cell: &Cell, image: &mut RgbImage) {
        let (x, y) = coords;
        let walls = cell.get_walls();

        let cell_width_without_joint_wall = self.cell_width() - self.wall_width;
        let start_x = x * cell_width_without_joint_wall + self.margin;
        let start_y = y * cell_width_without_joint_wall + self.margin;

        for y in start_y..=start_y + self.cell_width() {
            for x in start_x..=start_x + self.cell_width() {
                // A cell consists of two main zones: its walls and some empty space between them
                // called "a passage". To draw a cell, the following code checks some particular
                // zones and skips filling pixes with color in case a wall should not display or
                // it's a cell passage. In all other cases, we fill pixes with a given color

                // Top left corner must display only if either Northern or Western wall exists
                if x >= start_x
                    && x <= start_x + self.wall_width
                    && y >= start_y
                    && y <= start_y + self.wall_width
                {
                    if walls.carved(Pole::N) && walls.carved(Pole::W) {
                        continue;
                    }
                }

                // Northern wall must display only if there is no passage carved to North
                if x >= start_x + self.wall_width
                    && x <= start_x + cell_width_without_joint_wall
                    && y >= start_y
                    && y <= start_y + self.wall_width
                {
                    if walls.carved(Pole::N) {
                        continue;
                    }
                }

                // Top right corner must display only if either Northern or Eastern wall exists
                if x >= start_x + cell_width_without_joint_wall
                    && x <= start_x + self.cell_width()
                    && y >= start_y
                    && y <= start_y + self.wall_width
                {
                    if walls.carved(Pole::N) && walls.carved(Pole::E) {
                        continue;
                    }
                }

                // Western wall must display only if there is no passage carved to West
                if x >= start_x
                    && x <= start_x + self.wall_width
                    && y >= start_y + self.wall_width
                    && y <= start_y + cell_width_without_joint_wall
                {
                    if walls.carved(Pole::W) {
                        continue;
                    }
                }

                // Cell's passage must not be colored, i.e. it remains same as an image background
                if x >= start_x + self.wall_width
                    && x <= start_x + cell_width_without_joint_wall
                    && y >= start_y + self.wall_width
                    && y <= start_y + cell_width_without_joint_wall
                {
                    continue;
                }

                // Eastern wall must display only if there is no passage carved to East
                if x >= start_x + cell_width_without_joint_wall
                    && x <= start_x + self.cell_width()
                    && y >= start_y + self.wall_width
                    && y <= start_y + cell_width_without_joint_wall
                {
                    if walls.carved(Pole::E) {
                        continue;
                    }
                }

                // Bottom left corner must display only if either Southern or Western wall exists
                if x >= start_x
                    && x <= start_x + self.wall_width
                    && y >= start_y + cell_width_without_joint_wall
                    && y <= start_y + self.cell_width()
                {
                    if walls.carved(Pole::S) && walls.carved(Pole::W) {
                        continue;
                    }
                }

                // Southern wall must display only if there is no passage carved to South
                if x >= start_x + self.wall_width
                    && x <= start_x + cell_width_without_joint_wall
                    && y >= start_y + cell_width_without_joint_wall
                    && y <= start_y + self.cell_width()
                {
                    if walls.carved(Pole::S) {
                        continue;
                    }
                }

                // Bottom right corner must display only if either Southern or Eastern wall exists
                if x >= start_x + cell_width_without_joint_wall
                    && x <= start_x + self.cell_width()
                    && y >= start_y + cell_width_without_joint_wall
                    && y <= start_y + self.cell_width()
                {
                    if walls.carved(Pole::S) && walls.carved(Pole::E) {
                        continue;
                    }
                }

                // Fill the remaining pixels with a given color
                *image.get_pixel_mut(x as u32, y as u32) = match self.foreground_color {
                    Color::RGB(r, g, b) => image::Rgb([r, g, b]),
                }
            }
        }
    }
}

impl Formatter<ImageWrapper> for Image {
    fn format(&self, grid: &Grid) -> ImageWrapper {
        let (width, height) = self.sizes(grid);
        let mut image: RgbImage = ImageBuffer::new(width as u32, height as u32);

        self.fill_background(&mut image);
        self.draw_maze(&mut image, grid);

        ImageWrapper(image)
    }
}

#[cfg(test)]
mod tests {
    use image::EncodableLayout;

    use super::*;

    #[test]
    fn new_call_default_params() {
        let image = Image::new();
        assert_eq!(40, image.wall_width);
        assert_eq!(40, image.passage_width);
        assert_eq!(Color::RGB(250, 250, 250), image.background_color);
        assert_eq!(Color::RGB(0, 0, 0), image.foreground_color);
        assert_eq!(50, image.margin);
    }

    #[test]
    fn params_change() {
        let image = Image::new()
            .wall(10)
            .passage(5)
            .background(Color::RGB(1, 1, 1))
            .foreground(Color::RGB(100, 100, 100))
            .margin(20);

        assert_eq!(10, image.wall_width);
        assert_eq!(5, image.passage_width);
        assert_eq!(Color::RGB(1, 1, 1), image.background_color);
        assert_eq!(Color::RGB(100, 100, 100), image.foreground_color);
        assert_eq!(20, image.margin);
    }

    #[test]
    fn format() {
        let formatter = Image::new();
        let mut grid = generate_maze();

        let actual = formatter.format(&mut grid).0;
        let expected = image::open("tests/fixtures/maze.png").unwrap();

        assert_eq!(actual.as_bytes(), expected.as_bytes());
    }

    fn generate_maze() -> Grid {
        let mut grid = Grid::new(4, 4);

        grid.carve_passage((0, 0), Pole::S).unwrap();
        grid.carve_passage((0, 1), Pole::E).unwrap();
        grid.carve_passage((0, 2), Pole::E).unwrap();
        grid.carve_passage((0, 2), Pole::S).unwrap();
        grid.carve_passage((0, 3), Pole::E).unwrap();

        grid.carve_passage((1, 0), Pole::E).unwrap();
        grid.carve_passage((1, 1), Pole::E).unwrap();
        grid.carve_passage((1, 1), Pole::S).unwrap();
        grid.carve_passage((1, 2), Pole::E).unwrap();
        grid.carve_passage((1, 3), Pole::E).unwrap();

        grid.carve_passage((2, 0), Pole::E).unwrap();
        grid.carve_passage((2, 2), Pole::E).unwrap();
        grid.carve_passage((2, 3), Pole::E).unwrap();

        grid.carve_passage((3, 1), Pole::N).unwrap();
        grid.carve_passage((3, 1), Pole::S).unwrap();

        grid
    }
}
