/*
--- A run method on Screen that calls the draw method on each component
*/

pub trait Draw { 
    fn draw(&self);
}

pub struct Screen { 
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen { 
    pub fn run(&self) { 
        for component in self.components.iter() { 
            component.draw();
        }
    }
}
