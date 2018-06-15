use super::VirtualCanvas;
use super::sub_canvas::VirtualCanvasOption;
use super::super::component::ComponentOption;
use super::super::component::operation::Operation;
use super::super::component::operation::OperationExecuter;

impl<'l> VirtualCanvas<'l> {

    fn convert_coption_to_vcoption(&self, option: ComponentOption) -> VirtualCanvasOption {
        VirtualCanvasOption {
            position: option.position,
            angle: option.angle,
            alpha: option.alpha
        }
    }

}

impl<'l> OperationExecuter<'l> for VirtualCanvas<'l> {

    fn operation_execute(&self, operation: &Operation<'l>) {
        match operation {
            Operation::Group { option, operations } => {
                self.sub_canvas(self.convert_coption_to_vcoption(option.clone()), &|c| {
                    for o in operations { c.operation_execute(o); }
                });
            },
            Operation::Copy { t, p, clip, angle } => {
                self.copy(t, p.clone(), clip.clone(), angle.clone()).unwrap();
            },
            Operation::Zoom { t, p, clip, zoom_x, zoom_y, angle } => {
                self.zoom(t, p.clone(), clip.clone(), zoom_x.clone(), zoom_y.clone(), angle.clone()).unwrap();
            }
        }
    }

}

