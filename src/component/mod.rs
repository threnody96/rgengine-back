use std::cell::RefCell;
use std::rc::Rc;
use ::sdl2::rect::{ Point, Rect };
use ::sdl2::render::Texture;
use self::operation::Operation;

#[derive(Clone,Copy)]
pub struct ComponentOption {
    pub position: Rect,
    pub angle: f64,
    pub alpha: u8
}

pub struct Component<'l, P> {
    option: ComponentOption,
    props: RefCell<P>,
    operations: RefCell<Vec<Operation<'l>>>
}

pub trait RenderableComponent {
    fn render(&self);
}

impl<'l, P> Component<'l, P> {
    
    pub fn new(option: ComponentOption, props: RefCell<P>) -> Self {
        Self { option: option, props: props, operations: RefCell::new(Vec::new()) }
    }

    fn regist(&self, operation: Operation<'l>) {
        self.operations.borrow_mut().push(operation);
    }

    pub fn execute<CP>(&self, child_component: Component<'l, CP>)
        where Component<'l, CP>: RenderableComponent {
        child_component.render();
        let operation = child_component.emit();
        if operation.is_some() { self.regist(operation.unwrap()); }
    }

    pub fn emit(&self) -> Option<Operation<'l>> {
        let mut orig_operations = self.operations.borrow_mut();
        let mut operations: Vec<Operation> = Vec::new();
        while orig_operations.len() > 0 { operations.push(orig_operations.remove(0)); }
        if operations.len() == 0 { return None; }
        Some(Operation::Group { option: self.option, operations: operations })
    }

    pub fn copy(&self, t: Rc<Texture<'l>>, p: Point, clip: Option<Rect>, angle: f64) {
        self.regist(Operation::Copy { t: t, p: p, clip: clip, angle: angle });
    }

    pub fn zoom(&self, t: Rc<Texture<'l>>, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32>, angle: f64) {
        self.regist(Operation::Zoom {
            t: t,
            p: p,
            clip: clip,
            zoom_x: zoom_x,
            zoom_y: zoom_y,
            angle: angle
        });
    }

}

pub mod operation;
