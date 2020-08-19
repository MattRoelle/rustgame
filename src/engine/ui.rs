use sdl2::{pixels::Color, render::Canvas, video::Window};
use stretch::{
    geometry::{Point, Rect, Size},
    node::Node,
    result::Layout,
    style::{AlignContent, AlignItems, Dimension, FlexDirection, JustifyContent, Style, Overflow, PositionType, AlignSelf},
};

pub struct UIComponent<Props, Actions>  where Props: Copy {
    props: Props,
    render: fn(props: Props) -> ViewBuilder,
    on_action: fn(props: &mut Props, action: Actions),
    graph: UIGraph,
}

impl<Props, Actions> UIComponent<Props, Actions> where Props: Copy {
    pub fn new(
        initial_props: Props,
        on_action: fn(props: &mut Props, action: Actions),
        render: fn(props: Props) -> ViewBuilder) -> Self {
        Self {
            props: initial_props,
            render,
            on_action,
            graph: UIGraph::new(render(initial_props).clone()),
        }
    }

    pub fn dispatch(&mut self, action: Actions) {
        (self.on_action)(&mut self.props, action);
        self.graph = UIGraph::new((self.render)(self.props).clone());
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.graph.draw(canvas);
    }
}

pub struct UIGraph {
    stretch: stretch::node::Stretch,
    root: UINode,
}

impl UIGraph {
    pub fn new(root: ViewBuilder) -> Self {
        let mut ret = Self {
            stretch: stretch::node::Stretch::new(),
            root: root.build(),
            // root
        };

        ret.compute_stretch_node().unwrap();
        ret.compute_layout().unwrap();

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
        self.root.draw(&mut self.stretch, canvas, None);
    }
}

#[derive(Default, Copy, Clone)]
pub struct ViewStyle {
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<i32>,
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
    // name: String
}

impl UINode {
    // pub fn new(name: String, node_type: UINodeType, layout_style: Style, children: Vec<UINode>) -> Self {
    pub fn new(node_type: UINodeType, layout_style: Style, children: Vec<UINode>) -> Self {
        Self {
            node: None,
            layout: None,
            layout_style,
            children,
            node_type,
            // name
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
        for child in self.children.iter_mut() {
            child.compute_layout(stretch)?
        }
        stretch.compute_layout(
            self.node.expect("Must call compute_stretch_node() first"),
            Size::undefined(),
        )?;
        Ok(())
    }

    pub fn draw(
        &mut self,
        stretch: &mut stretch::node::Stretch,
        canvas: &mut Canvas<Window>,
        pos: Option<Point<f32>>,
    ) {
        let layout = stretch
            .layout(self.node.expect("Must call compute_layout() first"))
            .expect("Erorr calling stretch.layout");

        let cumulative_pos: Point<f32>;
        match pos {
            Some(v) => {
                cumulative_pos = Point {
                    x: v.x + layout.location.x,
                    y: v.y + layout.location.y,
                };
            }
            None => {
                cumulative_pos = Point { x: 0.0, y: 0.0 };
            }
        }

        match self.node_type {
            UINodeType::View(v) => {
                let border_width = v.border_width.unwrap_or(0);

                match v.border_color {
                    Some(c) => {
                        canvas.set_draw_color(c);
                        canvas
                            .fill_rect(sdl2::rect::Rect::new(
                                cumulative_pos.x as i32,
                                cumulative_pos.y as i32,
                                layout.size.width as u32,
                                layout.size.height as u32,
                            ))
                            .unwrap();
                    }
                    None => {}
                }

                match v.background_color {
                    Some(c) => {
                        canvas.set_draw_color(c);
                        canvas
                            .fill_rect(sdl2::rect::Rect::new(
                                (cumulative_pos.x + border_width as f32) as i32,
                                (cumulative_pos.y + border_width as f32) as i32,
                                layout.size.width as u32 - (border_width * 2) as u32,
                                layout.size.height as u32 - (border_width * 2) as u32,
                            ))
                            .unwrap();
                    },
                    None => {}
                }
            }
        };

        for child in self.children.iter_mut() {
            child.draw(stretch, canvas, Some(cumulative_pos));
        }
    }
}

pub enum UINodeBuilder {
    View(ViewBuilder),
}

impl UINodeBuilder {
    pub fn build(&self) -> UINode {
        match self {
            UINodeBuilder::View(v) => v.build(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StyleAttr {
    FlexDirection(FlexDirection),
    AlignItems(AlignItems),
    AlignContent(AlignContent),
    AlignSelf(AlignSelf),
    BgColorRGB(u8, u8, u8),
    JustifyContent(JustifyContent),
    BgColorRGBA(u8, u8, u8, u8),
    FlexBasis(Dimension),
    FlexGrow(f32),
    Width(Dimension),
    Height(Dimension),
    WidthPx(f32),
    HeightPx(f32),
    WidthPct(f32),
    HeightPct(f32),
    MinWidth(Dimension),
    MinHeight(Dimension),
    MinWidthPx(f32),
    MinHeightPx(f32),
    MinWidthPct(f32),
    MinHeightPct(f32),
    MaxWidth(Dimension),
    MaxHeight(Dimension),
    MaxWidthPx(f32),
    MaxHeightPx(f32),
    MaxWidthPct(f32),
    MaxHeightPct(f32),
    MarginPx(f32, f32, f32, f32),
    PaddingPx(f32, f32, f32, f32),
    MarginPct(f32, f32, f32, f32),
    PaddingPct(f32, f32, f32, f32),
    Margin(Dimension, Dimension, Dimension, Dimension),
    Padding(Dimension, Dimension, Dimension, Dimension),
    Overflow(Overflow),
    PositionPx(f32, f32, f32, f32),
    PositionPct(f32, f32, f32, f32),
    Position(Dimension, Dimension, Dimension, Dimension),
    PositionType(PositionType)
}

#[derive(Clone)]
pub struct ViewBuilder {
    layout_style: Style,
    child_nodes: Vec<ViewBuilder>,
    style: ViewStyle,
    // name: String
}

impl Into<UINodeBuilder> for ViewBuilder {
    fn into(self) -> UINodeBuilder {
        UINodeBuilder::View(self)
    }
}

impl ViewBuilder {
    pub fn style(&mut self, attr: StyleAttr) -> ViewBuilder {
        match attr {
            StyleAttr::FlexDirection(x) => { self.layout_style.flex_direction = x }
            StyleAttr::AlignItems(x) => { self.layout_style.align_items = x }
            StyleAttr::AlignContent(x) => { self.layout_style.align_content = x }
            StyleAttr::AlignSelf(x) => { self.layout_style.align_self = x }
            StyleAttr::JustifyContent(x) => { self.layout_style.justify_content = x }
            StyleAttr::BgColorRGB(r, g, b) => { self.style.background_color = Some(Color::RGB(r, g, b)) }
            StyleAttr::BgColorRGBA(r, g, b, a) => { self.style.background_color = Some(Color::RGBA(r, g, b, a))}
            StyleAttr::FlexBasis(x) => { self.layout_style.flex_basis = x }
            StyleAttr::FlexGrow(x) => { self.layout_style.flex_grow = x; }
            StyleAttr::Width(x) => { self.layout_style.size.width = x}
            StyleAttr::Height(x) => { self.layout_style.size.height = x}
            StyleAttr::WidthPx(x) => { self.layout_style.size.width = Dimension::Points(x) }
            StyleAttr::HeightPx(x) => { self.layout_style.size.height = Dimension::Points(x) }
            StyleAttr::WidthPct(x) => { self.layout_style.size.width = Dimension::Percent(x)}
            StyleAttr::HeightPct(x) => { self.layout_style.size.height = Dimension::Percent(x)}
            StyleAttr::MinWidth(x) => { self.layout_style.min_size.width = x}
            StyleAttr::MinHeight(x) => { self.layout_style.min_size.height = x}
            StyleAttr::MinWidthPx(x) => { self.layout_style.min_size.width = Dimension::Points(x) }
            StyleAttr::MinHeightPx(x) => { self.layout_style.min_size.height = Dimension::Points(x) }
            StyleAttr::MinWidthPct(x) => { self.layout_style.min_size.width = Dimension::Percent(x)}
            StyleAttr::MinHeightPct(x) => { self.layout_style.min_size.height = Dimension::Percent(x)}
            StyleAttr::MaxWidth(x) => { self.layout_style.max_size.width = x}
            StyleAttr::MaxHeight(x) => { self.layout_style.max_size.height = x}
            StyleAttr::MaxWidthPx(x) => { self.layout_style.max_size.width = Dimension::Points(x) }
            StyleAttr::MaxHeightPx(x) => { self.layout_style.max_size.height = Dimension::Points(x) }
            StyleAttr::MaxWidthPct(x) => { self.layout_style.max_size.width = Dimension::Percent(x)}
            StyleAttr::MaxHeightPct(x) => { self.layout_style.max_size.height = Dimension::Percent(x)}
            StyleAttr::MarginPx(start, top, end, bottom) => { 
                self.layout_style.margin = Rect {
                    start: Dimension::Points(start),
                    top: Dimension::Points(top),
                    end: Dimension::Points(end),
                    bottom: Dimension::Points(bottom),
                };
            }
            StyleAttr::PaddingPx(start, top, end, bottom) => { 
                self.layout_style.padding = Rect {
                    start: Dimension::Points(start),
                    top: Dimension::Points(top),
                    end: Dimension::Points(end),
                    bottom: Dimension::Points(bottom),
                };
            }
            StyleAttr::MarginPct(start, top, end, bottom) => { 
                self.layout_style.margin = Rect {
                    start: Dimension::Percent(start),
                    top: Dimension::Percent(top),
                    end: Dimension::Percent(end),
                    bottom: Dimension::Percent(bottom),
                };
            }
            StyleAttr::PaddingPct(start, top, end, bottom) => { 
                self.layout_style.padding = Rect {
                    start: Dimension::Percent(start),
                    top: Dimension::Percent(top),
                    end: Dimension::Percent(end),
                    bottom: Dimension::Percent(bottom),
                };
            }
            StyleAttr::Margin(start, top, end, bottom) => {
                self.layout_style.margin = Rect { start, top, end, bottom }
            }
            StyleAttr::Padding(start, top, end, bottom) => {
                self.layout_style.padding = Rect { start, top, end, bottom }
            }
            StyleAttr::Overflow(x) => { self.layout_style.overflow = x}
            StyleAttr::Position(start, top, end, bottom) => {
                self.layout_style.position = Rect { start, top, end, bottom }
            }
            StyleAttr::PositionPx(start, top, end, bottom) => { 
                self.layout_style.position = Rect {
                    start: Dimension::Points(start),
                    top: Dimension::Points(top),
                    end: Dimension::Points(end),
                    bottom: Dimension::Points(bottom),
                };
            }
            StyleAttr::PositionPct(start, top, end, bottom) => { 
                self.layout_style.position = Rect {
                    start: Dimension::Percent(start),
                    top: Dimension::Percent(top),
                    end: Dimension::Percent(end),
                    bottom: Dimension::Percent(bottom),
                };
            }
            StyleAttr::PositionType(x) => { self.layout_style.position_type = x }
        }
        self.clone()
    }

    pub fn style_if(&mut self, attr: StyleAttr, condition: bool) -> ViewBuilder {
        if condition {
            self.style(attr);
        }
        self.clone()
    }

    pub fn class(&mut self, attrs: &[StyleAttr]) -> ViewBuilder {
        for s in attrs.iter() {
            self.style(*s);
        }
        self.clone()
    }

    pub fn class_if(&mut self, class: &[StyleAttr], condition: bool) -> ViewBuilder {
        if condition {
            self.class(class);
        }
        self.clone()
    }

    pub fn child(&mut self, node: ViewBuilder) -> ViewBuilder {
        self.child_nodes.push(node.clone());
        self.clone()
    }

    pub fn children(&mut self, children: &mut Vec<ViewBuilder>) -> ViewBuilder {
        for child in children.iter_mut() {
            self.child_nodes.push(child.clone());
        }
        self.clone()
    }

    fn build(&self) -> UINode {
        UINode::new(
            UINodeType::View(self.style),
            self.layout_style,
            self.child_nodes.iter().map(|child| child.build()).collect(),
        )
    }
}

// pub fn view(name: &str) -> ViewBuilder {
pub fn view() -> ViewBuilder {
    ViewBuilder {
        layout_style: Style::default(),
        child_nodes: vec![],
        style: ViewStyle::default(),
        // name: String::from(name)
    }
}

#[macro_export]
macro_rules! define_class {
    ($name:ident, [$($attributes:expr),*]) => (
        static $name: &'static [StyleAttr] = &[
            $(
                $attributes,
            )*
        ];
    )
}

#[macro_export]
macro_rules! ui_tree {
    ($body:expr) => (
        $body
        .clone()
    )
}

#[macro_export]
macro_rules! view {
    () => (
        view()
    );
    ([
        $($classnames:ident),*
    ]) => (
        view()
            $(
                .class($classnames)
            )*
    );
    ([
        $($classnames:ident),*
    ], [
        $($attrs:expr),*
    ]) => (
        view()
            $(
                .class($classnames)
            )*
            $(
                .style($attrs)
            )*
    );
    ([
        $($classnames:ident),*
    ], [
        $($attrs:expr),*
    ], [
        $($children:expr),*
    ]) => (
        view()
            $(
                .class($classnames)
            )*
            $(
                .style($attrs)
            )*
            .children(&mut vec![
                $(
                    $children
                )*,
            ])
    );
}