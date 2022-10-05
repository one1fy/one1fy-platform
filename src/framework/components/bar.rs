use uuid::Uuid;
use skia_safe::{ Canvas };
use crate::components::*;

pub enum Orientation {
    HORIZONTAL,
    VERTICAL
}

pub struct BarContainer {
    pub id: Uuid,
    pub onLoad: fn(),
    pub visible: bool,
    pub height: u32,
    pub width: u32,
    pub left: u32,
    pub top: u32,
    pub children: Vec<Box<dyn Component_Traits>>,
    pub orientation: Orientation,
    pub remaining_x: u32,
    pub remaining_y: u32,
}

impl BarContainer {
    pub fn new(
        onLoad: fn(),
        visible: bool,
        height: u32,
        width: u32,
        left: u32,
        top: u32,
        children: Vec<Box<dyn Component_Traits>>,
        orientation: Orientation,
    ) -> BarContainer {
        let id = Uuid::new_v4();
        let rem_x = width;
        let rem_y = height;
        BarContainer {
            id: Uuid::new_v4(),
            onLoad: onLoad,
            visible: visible,
            height: height,
            width: width,
            left: left,
            top: top,
            children: children,
            orientation: orientation,
            remaining_x: width,
            remaining_y: height,
        }
        
    }

    pub fn calculate_coordinate(container_width: u32, num_children: u32, current_child: u32, child_width: u32) -> u32 {
        let current_slice: u32 = container_width * ((current_child + 1) / num_children);
        let previous_slice: u32 = container_width * (current_child / num_children);
        let left_to_change: u32 = ((current_slice + previous_slice) / 2) - child_width / 2;
        left_to_change
    }

    pub fn add_to_children(&mut self, child: Box<dyn Component_Traits>) {
        match &self.orientation {
            HORIZONTAL => {
                println!("{}", child.get_width());
                println!("{}", self.remaining_x);
                if (self.remaining_x - child.get_width() >= 0) {
                    self.remaining_x = self.remaining_x - child.get_width();
                    self.children.push(child);
                    let size: u32 = self.children.len() as u32;
                    for i in 0..self.children.len() {
                        let cur = &mut self.children[i];
                        //TODO: build setters for box
                        cur.set_left((BarContainer::calculate_coordinate(self.width, size, i as u32, cur.get_width())));
                        cur.set_top((self.height / 2 - cur.get_height() / 2));
                    }
                }
                else {
                    println!("Insufficient horizontal space in container.");
                }
            },
            VERTICAL => {
                if (self.remaining_y - child.get_height() >= 0) {
                    self.remaining_y = self.remaining_y - child.get_height();
                    self.children.push(child);
                    let size = self.children.len() as u32;
                    for i in 0..self.children.len() {
                        let cur = &mut self.children[i];
                        //TODO: build setters for box
                        cur.set_top((BarContainer::calculate_coordinate(self.height, size, i as u32, cur.get_height())));
                        cur.set_left((self.width / 2 - cur.get_width() / 2));
                    }
                }
                else {
                    println!("Insufficient vertical space in container.");
                }
            }
        }
        
    }

    

    
}
impl Draw for BarContainer{
    fn draw(&self, canvas: &mut Canvas) {
        for child in self.children.iter() {
            child.draw(canvas);
        }
    }
}