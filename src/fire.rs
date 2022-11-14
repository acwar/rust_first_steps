use std::borrow::Borrow;
use lerp::{LerpIter};
use lerp::num_traits::ToPrimitive;
use rand::Rng;
use crate::color;
use crate::color::Colores;


pub struct Fire {
    pub(crate) palette: Vec<Colores>,
    pub(crate) bonfire: Vec<Vec<usize>>,
    fire_width: u8,
    fire_height: u8
}

impl Fire {
    pub fn new(steps: usize, fire_width: u8,fire_height: u8) -> Fire {
        let mut response = Fire {
            palette: Self::compute_palette(steps),
            bonfire: vec![],
            fire_width,
            fire_height
        };
        response.start_bonfire();

        response
    }

    pub fn burn_frame(&mut self){

        for idx in 0..self.bonfire.len(){
            for idy in 0..self.bonfire[idx].len(){
                let pixel = &self.palette[self.bonfire[idx][idy]];
                if pixel != color::NEGRO.borrow() && idy>0{
                    let random = rand::thread_rng().gen_range(-2..3);
                    let mut nuplace = (idx as i32 + random  as i32 - 1) % self.fire_width  as i32;
                    if nuplace<0 {
                        nuplace = self.fire_width as i32 + nuplace - 1;
                    }
                    self.bonfire[nuplace as usize][idy - 1] = (self.bonfire[idx][idy] as i32 - (rand::thread_rng().gen_range(0..3) % 2)) as usize;
                }else {
                    if idy>0 {
                        self.bonfire[idx][idy - 1] = 0
                    }
                }
            }
        }

    }

    //Realmente las paletas deberian venir de los color, don't you think??
    fn compute_palette(steps: usize) -> Vec<Colores>{
        let mut response: Vec<Colores> = vec![];

        let base_colors = Colores::new();
        for x in 0..base_colors.len()-1 {
            let items_r: Vec<_> = base_colors[x].r.lerp_iter(base_colors[x+1].r, steps).collect();
            let items_g: Vec<_> = base_colors[x].g.lerp_iter(base_colors[x+1].g, steps).collect();
            let items_b: Vec<_> = base_colors[x].b.lerp_iter(base_colors[x+1].b, steps).collect();

            for i in 0..items_g.len() {
                response.push(Colores{
                    r: items_r[i].floor(),
                    g: items_g[i].floor(),
                    b: items_b[i].floor()})
            }
        }
        response.push(base_colors[base_colors.len()-1].clone());

        response
    }

    fn start_bonfire(&mut self) {
        let mut response: Vec<Vec<usize>> = vec![];
        for _x in 0..self.fire_width{
            let mut column: Vec<usize> = vec![];
            for _y in 0..self.fire_height-1{
                column.push(0)
            }
            column.push((self.palette.len() - 1).to_usize().unwrap());
            response.push(column);
        }
        self.bonfire = response;

    }
}