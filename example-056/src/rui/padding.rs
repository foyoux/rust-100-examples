use super::*;

pub struct Padding<V: View> {
    child: V,
    padding: f32,
}

impl<V> View for Padding<V>
where
    V: View,
{
    fn print(&self, id: ViewID, cx: &mut Context) {
        println!("Padding {{");
        (self.child).print(id.child(&0), cx);
        println!("}}");
    }

    fn process(&self, event: &Event, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        let mut local_event = event.clone();
        local_event.position -= LocalOffset::new(self.padding, self.padding);
        self.child.process(&local_event, id.child(&0), cx, vger);
    }

    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        vger.save();
        vger.translate([self.padding, self.padding].into());
        self.child.draw(id.child(&0), cx, vger);
        vger.restore();
    }

    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize {
        let child_size = self.child.layout(
            id.child(&0),
            sz - [2.0 * self.padding, 2.0 * self.padding].into(),
            cx,
            vger,
        );
        child_size + LocalSize::new(2.0 * self.padding, 2.0 * self.padding)
    }

    fn hittest(
        &self,
        id: ViewID,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut VGER,
    ) -> Option<ViewID> {
        self.child.hittest(
            id.child(&0),
            pt - LocalOffset::new(self.padding, self.padding),
            cx,
            vger,
        )
    }
}

pub enum PaddingParam {
    Auto,
    Px(f32),
}
pub struct Auto;
impl From<Auto> for PaddingParam {
    fn from(_val: Auto) -> Self {
        PaddingParam::Auto
    }
}
impl From<f32> for PaddingParam {
    fn from(val: f32) -> Self {
        PaddingParam::Px(val)
    }
}

impl<V> Padding<V>
where
    V: View + 'static,
{
    pub fn new(child: V, param: PaddingParam) -> Self {
        Self {
            child: child,
            padding: match param {
                PaddingParam::Auto => 5.0,
                PaddingParam::Px(px) => px,
            },
        }
    }
}
