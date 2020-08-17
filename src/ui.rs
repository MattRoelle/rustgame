use sdl2::{pixels::Color, render::Canvas, video::Window};
use stretch::{
    geometry::{Rect, Size},
    node::Node,
    result::Layout,
    style::{Dimension, Style, FlexDirection, AlignItems, AlignContent, JustifyContent},
};

pub fn pointrect_all(inner: f32) -> Rect<Dimension> {
    Rect {
        start: Dimension::Points(inner),
        end: Dimension::Points(inner),
        top: Dimension::Points(inner),
        bottom: Dimension::Points(inner),
    }
}

pub fn pointrect(start: f32, top: f32, end: f32, bottom: f32) -> Rect<Dimension> {
    Rect {
        start: Dimension::Points(start),
        end: Dimension::Points(end),
        top: Dimension::Points(top),
        bottom: Dimension::Points(bottom),
    }
}

pub struct RenderStyle {}

pub struct UIGraph {
    stretch: stretch::node::Stretch,
    root: UINode,
}

impl UIGraph {
    pub fn new(root: &mut ViewBuilder) -> Self {
        let mut ret = Self {
            stretch: stretch::node::Stretch::new(),
            root: root.build(),
        };

        ret.compute_stretch_node();
        ret.compute_layout();

        return ret;
    }

    pub fn set_root(&mut self, root: UINode) {
        self.root = root;
        self.compute_stretch_node().unwrap();
        self.compute_layout().unwrap();
    }

    fn compute_stretch_node(&mut self) -> Result<(), stretch::Error> {
        self.root.compute_stretch_node(&mut self.stretch)?;
        Ok(())
    }

    fn compute_layout(&mut self) -> Result<(), stretch::Error> {
        self.root.compute_layout(&mut self.stretch)
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.root.draw(&mut self.stretch, canvas);
    }
}

#[derive(Default, Copy, Clone)]
pub struct ViewStyle {
    pub background_color: Option<Color>,
}

pub enum UINodeType {
    View(ViewStyle),
}

pub struct UINode {
    node: Option<Node>,
    layout: Option<Layout>,
    layout_style: Style,
    children: Vec<UINode>,
    node_type: UINodeType,
}

impl UINode {
    pub fn new(node_type: UINodeType, layout_style: Style, children: Vec<UINode>) -> Self {
        Self {
            node: None,
            layout: None,
            layout_style,
            children,
            node_type,
        }
    }

    pub fn compute_stretch_node(
        &mut self,
        stretch: &mut stretch::node::Stretch,
    ) -> Result<Node, stretch::Error> {
        let stretch_node;

        if self.children.is_empty() {
            stretch_node = stretch.new_node(self.layout_style, vec![]).unwrap();
        } else {
            let mut children = Vec::new();
            for child in self.children.iter_mut() {
                children.push(child.compute_stretch_node(stretch).unwrap());
            }

            stretch_node = stretch.new_node(self.layout_style, children).unwrap();
        }

        self.node = Some(stretch_node);

        Ok(stretch_node)
    }

    pub fn compute_layout(
        &mut self,
        stretch: &mut stretch::node::Stretch,
    ) -> Result<(), stretch::Error> {
        stretch.compute_layout(
            self.node.expect("Must call compute_stretch_node() first"),
            Size::undefined(),
        )?;
        Ok(())
    }

    pub fn draw(&mut self, stretch: &mut stretch::node::Stretch, canvas: &mut Canvas<Window>) {
        let layout = stretch
            .layout(self.node.expect("Must call compute_layout() first"))
            .expect("Erorr calling stretch.layout");

        match self.node_type {
            UINodeType::View(v) => {
                canvas.set_draw_color(v.background_color.unwrap_or(Color::RGBA(0, 0, 0, 0)));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        layout.location.x as i32,
                        layout.location.y as i32,
                        layout.size.width as u32,
                        layout.size.height as u32,
                    ))
                    .unwrap();
            }
        };

        for child in self.children.iter_mut() {
            child.draw(stretch, canvas);
        }
    }
}

pub enum UINodeBuilder {
    View(ViewBuilder)
}

impl UINodeBuilder {
    pub fn build(&self) -> UINode {
        match self {
            UINodeBuilder::View(v) => { v.build() }
        }
    }
}

pub trait LayoutStyleBuilder {
    fn layout_style(&mut self) -> &mut Style;
    fn flex_direction(&mut self, flex_direction: FlexDirection) -> &mut Self { self.layout_style().flex_direction = flex_direction; self }
    fn align_items(&mut self, align_items: AlignItems) -> &mut Self { self.layout_style().align_items = align_items; self }
    fn align_content(&mut self, align_content: AlignContent) -> &mut Self { self.layout_style().align_content = align_content; self } 
    fn justify_content(&mut self, justify_content: JustifyContent) -> &mut Self { self.layout_style().justify_content = justify_content; self }
    fn flex_basis(&mut self, flex_basis: Dimension) -> &mut Self { self.layout_style().flex_basis = flex_basis; self }
    fn flex_grow(&mut self, flex_grow: f32) -> &mut Self { self.layout_style().flex_grow = flex_grow; self }
    fn width_px(&mut self, width: f32) -> &mut Self { self.layout_style().size.width = Dimension::Points(width); self }
    fn width_pc(&mut self, width: f32) -> &mut Self { self.layout_style().size.width = Dimension::Percent(width); self }
    fn height_px(&mut self, height: f32) -> &mut Self { self.layout_style().size.height = Dimension::Points(height); self }
    fn height_pc(&mut self, height: f32) -> &mut Self { self.layout_style().size.height = Dimension::Percent(height); self }
    fn min_width_px(&mut self, min_width: f32) -> &mut Self { self.layout_style().min_size.width = Dimension::Points(min_width); self }
    fn min_width_pc(&mut self, min_width: f32) -> &mut Self { self.layout_style().min_size.width = Dimension::Percent(min_width); self }
    fn min_height_px(&mut self, min_height: f32) -> &mut Self { self.layout_style().min_size.height = Dimension::Points(min_height); self }
    fn min_height_pc(&mut self, min_height: f32) -> &mut Self { self.layout_style().min_size.height = Dimension::Percent(min_height); self }
    fn margin_pt(&mut self, start: f32, top: f32, end: f32, bottom: f32) -> &mut Self {
        self.layout_style().margin = Rect {
            start: Dimension::Points(start),
            top: Dimension::Points(top),
            end: Dimension::Points(end),
            bottom: Dimension::Points(bottom)
        };
        self
    }
    fn margin_pt_all(&mut self, v: f32) -> &mut Self {
        self.layout_style().margin = Rect {
            start: Dimension::Points(v),
            top: Dimension::Points(v),
            end: Dimension::Points(v),
            bottom: Dimension::Points(v)
        };
        self
    }
    fn padding_pt(&mut self, start: f32, top: f32, end: f32, bottom: f32) -> &mut Self {
        self.layout_style().padding = Rect {
            start: Dimension::Points(start),
            top: Dimension::Points(top),
            end: Dimension::Points(end),
            bottom: Dimension::Points(bottom)
        };
        self
    }
    fn padding_pt_all(&mut self, v: f32) -> &mut Self {
        self.layout_style().padding = Rect {
            start: Dimension::Points(v),
            top: Dimension::Points(v),
            end: Dimension::Points(v),
            bottom: Dimension::Points(v)
        };
        self
    }
}

#[derive(Clone)]
pub struct ViewBuilder {
    layout_style: Style,
    child_nodes: Vec<ViewBuilder>,
    style: ViewStyle
}

impl Into<UINodeBuilder> for ViewBuilder {
    fn into(self) -> UINodeBuilder {
        UINodeBuilder::View(self)
    }
}

impl LayoutStyleBuilder for ViewBuilder {
    fn layout_style(&mut self) -> &mut Style {
        &mut self.layout_style
    }
}

impl ViewBuilder {
    pub fn bg_color(&mut self, color: Color) -> &mut ViewBuilder {
        self.style.background_color = Some(color);
        self
    }

    pub fn child(&mut self, node: &mut ViewBuilder) -> &mut ViewBuilder {
        self.child_nodes.push(node.clone());
        self
    }

    pub fn children(&mut self, children: &mut Vec<&mut ViewBuilder>) -> &mut ViewBuilder {
        for child in children.iter_mut() {
            self.child_nodes.push(child.clone());
        }
        self
    }

    fn build(&self) -> UINode {
        UINode::new(UINodeType::View(self.style), self.layout_style, self.child_nodes.iter().map(|child| { child.build() }).collect())
    }
}

pub fn view() -> ViewBuilder {
    ViewBuilder {
        layout_style: Style::default(),
        child_nodes: vec![],
        style: ViewStyle::default()
    }
}