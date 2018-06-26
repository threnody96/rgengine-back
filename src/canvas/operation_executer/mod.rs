use super::{ VirtualCanvas, VirtualCanvasOption };
use super::super::component::ComponentOption;
use super::super::component::operation::Operation;
use super::super::component::operation::OperationExecuter;

impl VirtualCanvas {

    fn convert_coption_to_vcoption(&self, option: ComponentOption) -> VirtualCanvasOption {
        VirtualCanvasOption {
            mode: option.mode,
            position: option.position,
            angle: option.angle,
            alpha: option.alpha
        }
    }

}

impl OperationExecuter for VirtualCanvas {

    fn operation_execute(&self, operation: &Operation) {
        match operation {
            Operation::Group { option, operations } => {
                self.sub_canvas(self.convert_coption_to_vcoption(option.clone()), &|c| {
                    for o in operations { c.operation_execute(o); }
                    c.vcanvas.set_blend_mode(option.mode).set_texture_alpha(option.alpha).emit();
                });
            },
            Operation::Copy { t, p, clip, angle } => {
                self.vcanvas.copy(t.clone(), p.clone(), clip.clone(), angle.clone());
            },
            Operation::Zoom { t, p, clip, zoom_x, zoom_y, angle } => {
                self.vcanvas.zoom(t.clone(), p.clone(), clip.clone(), zoom_x.clone(), zoom_y.clone(), angle.clone());
            }
        }
    }

}

