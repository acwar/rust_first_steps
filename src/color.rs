use ggez::graphics::Color;
use lerp::Lerp;

#[derive(Lerp, PartialEq, Debug)]
pub struct Colores {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

pub const NEGRO: Colores = Colores{r:0.0,g:0.0,b:0.0};
pub const BLANCO: Colores = Colores{r:255.0,g:255.0,b:255.0};

impl Colores{
    pub fn new() -> [Colores; 5]{
        [
            NEGRO,
            Colores{r:255.0,g:0.0,b:0.0},
            Colores{r:255.0,g: 191.0,b: 0.0},
            Colores{r:255.0,g:215.0,b:0.0},
            BLANCO
        ]
    }

    pub fn to_ggez_color(&self) -> Color {
        Color::from_rgb(self.r as u8, self.g as u8, self.b as u8)
    }
}

impl Clone for Colores{
    fn clone(&self) -> Self {
        Colores{
            r: self.r,
            g: self.g,
            b: self.b
        }
    }
}