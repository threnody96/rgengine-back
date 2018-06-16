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

pub trait ComponentProps {
    fn update(&self, next_props: Self);
}

pub struct Component<P: ComponentProps> {
    pub props: RefCell<P>,
    operations: RefCell<Vec<Operation>>,
    renderer: Box<ComponentRenderer<P>>
}

pub trait ComponentRenderer<P: ComponentProps> {
    fn option(&self, c: &Component<P>) -> ComponentOption;
    fn update_props(&self, c: &Component<P>, P);
    fn render(&self, c: &Component<P>);
}

impl<P: ComponentProps> Component<P> {
    
    pub fn new(renderer: Box<ComponentRenderer<P>>, props: P) -> Self {
        Self { renderer: renderer, props: RefCell::new(props), operations: RefCell::new(Vec::new()) }
    }

    fn regist(&self, operation: Operation) {
        self.operations.borrow_mut().push(operation);
    }

    fn option(&self) -> ComponentOption {
        self.renderer.option(self)
    }

    pub fn render(&self) -> &Self {
        self.renderer.render(self);
        self
    }

    pub fn update_props(&self, next_props: P) {
        self.renderer.update_props(self, next_props);
    }

    pub fn execute<CP: ComponentProps>(&self, child_component: Rc<Component<CP>>) {
        let operation = child_component.render().emit(true);
        if operation.is_some() { self.regist(operation.unwrap()); }
    }

    pub fn emit(&self, compress: bool) -> Option<Operation> {
        let mut orig_operations = self.operations.borrow_mut();
        let mut operations: Vec<Operation> = Vec::new();
        while orig_operations.len() > 0 { operations.push(orig_operations.remove(0)); }
        if compress && operations.len() == 0 { return None; }
        Some(Operation::Group { option: self.option(), operations: operations })
    }

    pub fn copy(&self, t: Rc<Texture>, p: Point, clip: Option<Rect>, angle: f64) {
        self.regist(Operation::Copy { t: t, p: p, clip: clip, angle: angle });
    }

    pub fn zoom(&self, t: Rc<Texture>, p: Point, clip: Option<Rect>, zoom_x: Option<f32>, zoom_y: Option<f32>, angle: f64) {
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
