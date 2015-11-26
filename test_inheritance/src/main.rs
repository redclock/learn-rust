use std::ops::Deref;
use std::ops::DerefMut;

struct Node {
    x: f32,
    y: f32,
}

impl Node {
    fn draw(&self) {
        println!("node: x={}, y={}", self.x, self.y)
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

struct Sprite {
    node: Node
}

impl Sprite {
    fn draw(&self) {
        println!("sprite: x={}, y={}", self.node.x, self.node.y)
    }
}

impl Deref for Sprite {
    type Target = Node;

    fn deref<'a>(&'a self) -> &'a Node {
        &self.node
    }
}

impl DerefMut for Sprite {
    //type Target = Node;

    fn deref_mut<'a>(&'a mut self) -> &'a mut Node {
        &mut self.node
    }
}

fn main() {
    let mut node = Node{x: 10.0, y: 20.0};
    node.draw();
    node.move_to(30.0, 40.0);
    node.draw();

    let mut sprite = Sprite{ node: Node { x: 10.0, y: 20.0 } };
    sprite.draw();
    sprite.move_to(30.0, 40.0);
    sprite.draw();
    {
        let mut sprite_node: &mut Node = &mut sprite;
        sprite_node.move_to(100.0, 100.0);
        sprite_node.draw();
    }
    sprite.draw();
    println!("Hello, world!");
}
