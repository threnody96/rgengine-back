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
    pub props: RefCell<P>,
    operations: RefCell<Vec<Operation<'l>>>,
    renderer: Box<ComponentRenderer<'l, P>>
}

pub trait ComponentRenderer<'l, P> {
    fn render(&self, c: &'l Component<'l, P>);
}

impl<'l, P> Component<'l, P> {
    
    pub fn new(renderer: Box<ComponentRenderer<'l, P>>, props: P) -> Self {
        Self { renderer: renderer, props: RefCell::new(props), operations: RefCell::new(Vec::new()) }
    }

    fn regist(&self, operation: Operation<'l>) {
        self.operations.borrow_mut().push(operation);
    }

    pub fn render(&'l self) {
        self.renderer.render(self);
    }

    pub fn execute<CP>(&self, option: ComponentOption, child_component: &'l Component<'l, CP>) {
        child_component.render();
        let operation = child_component.emit(option);
        if operation.is_some() { self.regist(operation.unwrap()); }
    }

    pub fn emit(&self, option: ComponentOption) -> Option<Operation<'l>> {
        let mut orig_operations = self.operations.borrow_mut();
        let mut operations: Vec<Operation> = Vec::new();
        while orig_operations.len() > 0 { operations.push(orig_operations.remove(0)); }
        if operations.len() == 0 { return None; }
        Some(Operation::Group { option: option, operations: operations })
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
