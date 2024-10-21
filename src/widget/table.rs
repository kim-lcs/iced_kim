use iced::advanced::renderer::Quad;
use iced::advanced::widget::tree;
use iced::advanced::{layout, renderer, Clipboard, Layout, Shell, Widget};
use iced::border::{self, Border, Radius};
use iced::event::{self, Event};
use iced::widget::text::Wrapping;
use iced::{mouse, Pixels, Point};
use iced::{overlay, Font};
use iced::{touch, Alignment};
use iced::{Background, Color, Element, Length, Rectangle, Shadow, Size, Theme, Vector};
use tree::Tree;

/// create a table widget with head and source data
/// * `head` - the table head infos
/// * `source` - the table source datas, the source data must derive from [TableRow]
pub fn table<'a, T, Message, Theme>(
    heads: &'a Vec<TableHead>,
    source: &'a Vec<T>,
) -> Table<'a, T, Message, Theme>
where
    Theme: 'a + Catalog,
    T: 'a + TableRow,
{
    Table::new(heads, source)
}

/// create a normal table column with source data section name and column shown text
/// * `name` - source data section name
/// * `text` - table column shown text
pub fn table_head(name: impl Into<String>, text: impl Into<String>) -> TableHead {
    TableHead::new(name, text)
}

/// create a new table column head by the type of index
/// * `text` - index column shown text
pub fn table_index(text: impl Into<String>) -> TableHead {
    TableHead::new("", text).head_type(TableHeadType::Index)
}

// ! table row
pub trait TableRow {
    fn get_value(&self, filed_name: &str) -> String;
}

/// ! table

#[allow(missing_debug_implementations)]
pub struct Table<'a, T, Message, Theme>
where
    Theme: 'a + Catalog,
    T: 'a + TableRow,
{
    /// all heads of table
    heads: &'a Vec<TableHead>,
    /// the font of table
    font: Font,
    /// the source data of table
    source: &'a Vec<T>,
    /// whether to display a row background color in zebra type
    show_stripe: bool,
    /// a callback when selected row
    on_row_selected: Option<OnRowSelected<'a, Message>>,
    /// a callback when row moved
    on_row_moved: Option<OnRowMoved<'a, Message>>,
    /// a callback when column moved
    on_col_moved: Option<OnColMoved<'a, Message>>,
    /// a callback when column width changed
    on_col_width_changed: Option<Box<dyn Fn(&'a str, f32) -> Message + 'a>>,
    /// the head height
    head_height: Length,
    /// the row height
    row_height: Length,
    /// the width of table
    width: Length,
    /// the height of table
    height: Length,
    /// the class of table
    class: Theme::Class<'a>,
    /// the rectangle of heads
    head_rects: Vec<Cell>,
    /// the rectangle of cells
    cell_rects: Vec<Cell>,
    /// the rectangle of head spliter
    head_spliter_rects: Vec<Cell>,
}

impl<'a, T, Message, Theme> Table<'a, T, Message, Theme>
where
    Theme: 'a + Catalog,
    T: 'a + TableRow,
{
    /// Creates a new [`Table`] with the given heads and source.
    pub fn new(heads: &'a Vec<TableHead>, source: &'a Vec<T>) -> Self {
        // println!("table new");
        Self {
            heads,
            head_height: Length::Fixed(40.0),
            row_height: Length::Fixed(30.0),
            font: Font::with_name("微软雅黑"),
            source,
            show_stripe: true,
            on_row_selected: None,
            on_row_moved: None,
            on_col_moved: None,
            on_col_width_changed: None,
            width: Length::Fixed(400.0),
            height: Length::Fixed(300.0),
            class: Theme::default(),
            cell_rects: Vec::new(),
            head_rects: Vec::new(),
            head_spliter_rects: Vec::new(),
        }
    }

    /// Sets the width of the [`Button`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Button`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn head_height(mut self, head_height: impl Into<Length>) -> Self {
        self.head_height = head_height.into();
        self
    }

    pub fn row_height(mut self, row_height: impl Into<Length>) -> Self {
        self.row_height = row_height.into();
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub fn show_stripe(mut self, show_stripe: bool) -> Self {
        self.show_stripe = show_stripe;
        self
    }

    /// Sets the style of the [`Table`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// a callback of row selected
    /// * `param1` - the row index selected
    pub fn on_row_selected(mut self, on_row_selected: impl Fn(i32) -> Message + 'a) -> Self {
        self.on_row_selected = Some(OnRowSelected::Closure(Box::new(on_row_selected)));
        self
    }

    /// a callback of row moved
    /// # Examples
    /// ``` no_run
    /// // the message define
    /// #[derive(Message, Clone)]
    /// enum Msg {
    ///     RowMoved(i32, i32),
    /// }
    ///
    /// // view
    /// let table = table(&self.table_heads, &self.table_datas)
    ///     .on_row_moved(|a| Msg::RowMoved(a.src_row_index,a.dst_row_index).into());
    ///
    /// // update
    /// match msg {
    ///      Msg::RowMoved { src, dst } => {
    ///         let moved = self.table_datas.remove(*src as usize);
    ///         self.table_datas.insert(*dst as usize, moved);
    ///     }
    /// }
    /// ```
    pub fn on_row_moved(mut self, on_row_moved: impl Fn(RowMovedData) -> Message + 'a) -> Self {
        self.on_row_moved = Some(OnRowMoved::Closure(Box::new(on_row_moved)));
        self
    }

    /// a callback of row moved, if on_row_moved is None, the default behavior is ignored.
    pub fn on_row_moved_maybe(
        mut self,
        on_row_moved: Option<impl Fn(RowMovedData) -> Message + 'a>,
    ) -> Self {
        match on_row_moved {
            Some(on_row_moved) => {
                self.on_row_moved = Some(OnRowMoved::Closure(Box::new(on_row_moved)))
            }
            None => self.on_row_moved = None,
        }
        self
    }

    /// a callback of column moved
    pub fn on_col_moved(mut self, on_col_moved: impl Fn(ColMovedData) -> Message + 'a) -> Self {
        self.on_col_moved = Some(OnColMoved::Closure(Box::new(on_col_moved)));
        self
    }

    /// a callback of column moved, if on_col_moved is None, the default behavior is ignored.
    pub fn on_col_moved_maybe(
        mut self,
        on_col_moved: Option<impl Fn(ColMovedData) -> Message + 'a>,
    ) -> Self {
        match on_col_moved {
            Some(on_col_moved) => {
                self.on_col_moved = Some(OnColMoved::Closure(Box::new(on_col_moved)))
            }
            None => self.on_col_moved = None,
        }
        self
    }

    /// a callback of column width changed
    /// * `name` - the column name
    /// * `width` - the column width
    pub fn on_col_width_changed(
        mut self,
        on_col_width_changed: impl Fn(&'a str, f32) -> Message + 'a,
    ) -> Self {
        self.on_col_width_changed = Some(Box::new(on_col_width_changed));
        self
    }

    /// calculate the fill type width
    fn cal_fill_width(&self, width: f32) -> f32 {
        let mut fixed_len = 0.0;
        let mut fill_cnt = 0;
        for head in self.heads.iter() {
            match head.width {
                Length::Fixed(val) => {
                    fixed_len += val;
                }
                Length::FillPortion(val) => fill_cnt += val,
                _ => fill_cnt += 1,
            };
        }
        let fill_width = if fill_cnt > 0 {
            (width - fixed_len) / fill_cnt as f32
        } else {
            0.0
        };
        fill_width
    }

    /// calculates the col width
    fn cal_col_width(&self, head: &TableHead, width_fill: f32) -> f32 {
        let col_width = match head.width {
            Length::Fixed(val) => val,
            Length::FillPortion(val) => width_fill * (val as f32),
            _ => width_fill,
        };
        col_width
    }

    fn cal_row_height(&self) -> f32 {
        let row_height = match self.row_height {
            Length::Fixed(val) => val,
            _ => 40.0,
        };
        row_height
    }

    fn cal_header_height(&self) -> f32 {
        let header_height = match self.head_height {
            Length::Shrink => 40.0,
            Length::Fixed(val) => val,
            _ => 60.0,
        };
        header_height
    }

    fn get_fill_text(
        &self,
        content: impl Into<String>,
        size: impl Into<Pixels>,
        bounds: Size,
        align_x: Alignment,
        align_y: Alignment,
    ) -> iced::advanced::Text {
        iced::advanced::text::Text {
            content: content.into(),
            bounds,
            size: size.into(),
            line_height: iced::advanced::text::LineHeight::Relative(1.0),
            font: self.font,
            horizontal_alignment: align_x.into(),
            vertical_alignment: align_y.into(),
            shaping: iced::advanced::text::Shaping::Basic,
            wrapping: Wrapping::default(),
        }
    }

    /// get the text position in a rect by alignment
    fn get_text_position(&self, rect: &Rectangle, align_x: Alignment, align_y: Alignment) -> Point {
        // [left]: the position is left_top, [right]: the position is right_top, [center]: the position is center
        Point::new(
            match align_x {
                Alignment::Start => rect.x,
                Alignment::Center => rect.x + rect.width / 2.0,
                Alignment::End => rect.x + rect.width,
            },
            match align_y {
                Alignment::Start => rect.y,
                Alignment::Center => rect.y + rect.height / 2.0,
                Alignment::End => rect.y + rect.height,
            },
        )
    }
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Table<'a, T, Message, Theme>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: 'a + Catalog,
    T: 'a + TableRow,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        let size = Size {
            width: self.width,
            height: self.height,
        };
        size
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let _ = renderer;
        let _ = clipboard;
        let _ = viewport;

        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();
        let table_width = bounds.width;
        let table_height = bounds.height;
        let width_fill = self.cal_fill_width(table_width);
        let offset_x = state.get_offset_x();
        let offset_y = state.get_offset_y();
        let head_height = self.cal_header_height();

        let mut split_rects = Vec::new();
        let mut head_rects = Vec::new();
        // ! head size
        let mut top = bounds.y;
        let mut left = bounds.x;
        for (col_idx, head) in self.heads.iter().enumerate() {
            let col_width = self.cal_col_width(head, width_fill);
            // * spliter rectangle
            // only fixed type can change width by user move cursor
            let left_top = Point::new(left + col_width - 2.5 - offset_x, top);
            let size = Size::new(5.0, head_height);
            let rect = Rectangle::new(left_top, size);
            let fixed = if let Length::Fixed(_) = head.width {
                true
            } else {
                false
            };
            split_rects.push(Cell {
                rect,
                row: 0,
                col: col_idx as i32,
                fixed: fixed,
            });
            // * head rectangle
            let left_top = Point::new(left, top);
            let size = Size::new(col_width, head_height);
            let rect = Rectangle::new(left_top, size);
            head_rects.push(Cell {
                rect,
                row: 0,
                col: col_idx as i32,
                fixed,
            });
            left += col_width;
        }
        self.head_spliter_rects = split_rects;
        self.head_rects = head_rects;

        let body_width_real = left - bounds.x;

        // ! rows cell size
        let mut cells = Vec::new();
        let row_height = self.cal_row_height();
        top = bounds.y - offset_y + head_height;
        for (row_idx, _) in self.source.iter().enumerate() {
            left = bounds.x - offset_x;
            for (col_idx, head) in self.heads.iter().enumerate() {
                let col_width = self.cal_col_width(head, width_fill);
                let left_top = Point::new(left, top);
                let size = Size::new(col_width, row_height);
                let rect = Rectangle::new(left_top, size);
                left += col_width;
                cells.push(Cell {
                    rect,
                    row: row_idx as i32,
                    col: col_idx as i32,
                    fixed: false,
                });
            }
            top += row_height;
        }
        self.cell_rects = cells;

        let body_height_real = top - (bounds.y - offset_y + head_height); // row_height * self.source.len() as f32;

        // ! horizontal scroll bar
        state.scrollbar_x.show = table_width < body_width_real;
        if state.scrollbar_x.show {
            let top = bounds.y + table_height - state.track_thin_size;
            let thumb_width = table_width / body_width_real * table_width;
            let mut track_width = table_width;
            if state.scrollbar_y.show {
                track_width -= state.track_thin_size / 2.0;
            }
            state.scrollbar_x.track_rect = Rectangle {
                x: bounds.x,
                y: top,
                width: track_width,
                height: state.track_thin_size,
            };
            if state.scrollbar_x.offset > 0.0 {
                let overflow = (state.scrollbar_x.offset + thumb_width) - track_width;
                if overflow > 0.0 {
                    state.scrollbar_x.offset -= overflow * track_width / thumb_width;
                }
            }

            state.scrollbar_x.thumb_rect = Rectangle {
                x: bounds.x + state.scrollbar_x.offset,
                y: top,
                width: thumb_width,
                height: state.track_thin_size,
            };
        } else {
            state.scrollbar_x.offset = 0.0;
        }

        // ! vertical scroll bar
        let body_height = bounds.height - head_height;
        state.scrollbar_y.show = body_height < body_height_real;

        if state.scrollbar_y.show {
            let top = bounds.y + head_height;
            let left = bounds.x + table_width - state.track_thin_size;
            let mut track_height = body_height;
            if state.scrollbar_x.show {
                track_height -= state.track_thin_size / 2.0;
            }
            // ! view_height / real_height = thumb_height / track_height
            let thumb_height = body_height / body_height_real * track_height;
            state.scrollbar_y.track_rect = Rectangle {
                x: left,
                y: top,
                width: state.track_thin_size,
                height: track_height,
            };

            state.scrollbar_y.thumb_rect = Rectangle {
                x: left,
                y: top + state.scrollbar_y.offset,
                width: state.track_thin_size,
                height: thumb_height,
            };
        } else {
            state.scrollbar_y.offset = 0.0;
        }

        // ! handle event
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                state.is_pressed = true;
                state.time_pressed = std::time::SystemTime::now();
                if let Some(position) = cursor.position() {
                    state.point_pressed = position;
                }
                // click head spliter
                for cell in self.head_spliter_rects.iter() {
                    if cursor.is_over(cell.rect) {
                        return event::Status::Captured;
                    }
                }
                // click head
                for cell in self.head_rects.iter() {
                    if cursor.is_over(cell.rect) {
                        state.col_pressed = cell.col;
                        return event::Status::Captured;
                    }
                }
                // click row
                for cell in self.cell_rects.iter() {
                    if cursor.is_over(cell.rect) {
                        state.row_pressed = cell.row;
                        return event::Status::Captured;
                    }
                }
                return event::Status::Ignored;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                let state = tree.state.downcast_mut::<State>();
                state.is_pressed = false;
                state.can_col_move = false;
                state.can_row_move = false;
                if state.is_cell_hover {
                    // handle row moved
                    if state.row_pressed >= 0
                        && state.row_hover >= 0
                        && state.row_hover != state.row_pressed
                    {
                        if let Some(message) = self
                            .on_row_moved
                            .as_ref()
                            .map(|a| a.get(state.row_pressed, state.row_hover))
                        {
                            shell.publish(message);
                        }
                    }

                    // handle row selected
                    if let Some(message) = self
                        .on_row_selected
                        .as_ref()
                        .map(|a| a.get(state.row_hover, state.col_hover))
                    {
                        state.row_selected = state.row_hover;
                        state.col_selected = state.col_hover;
                        shell.publish(message);
                    }
                    return event::Status::Captured;
                }
                if state.is_head_hover {
                    // handle column moved
                    if state.col_pressed >= 0
                        && state.col_hover >= 0
                        && state.col_hover != state.col_pressed
                    {
                        let src = self.heads.get(state.col_pressed as usize);
                        let dst = self.heads.get(state.col_hover as usize);
                        if let (Some(src), Some(dst)) = (src, dst) {
                            if let Some(message) = self.on_col_moved.as_ref().map(move |a| {
                                a.get(ColMovedData {
                                    src_col_index: state.col_pressed,
                                    dst_col_index: state.col_hover,
                                    src_col_name: &src.name,
                                    dst_col_name: &dst.name,
                                })
                            }) {
                                shell.publish(message);
                            }
                        }
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_hover = cursor.is_over(bounds);

                // * ajust column width
                if state.is_pressed {
                    if state.is_head_spliter_hover {
                        if let Some(on_col_width_changed) = &self.on_col_width_changed {
                            let width = if let Length::Fixed(val) =
                                self.heads[state.col_selected as usize].width
                            {
                                val
                            } else {
                                0.0
                            };
                            let delta_x = position.x - state.point_pressed.x;
                            if width + delta_x > 10.0 {
                                state.point_pressed = position;
                                let name = &self.heads[state.col_selected as usize].name;
                                let message = on_col_width_changed(name, width + delta_x);
                                shell.publish(message);
                            }
                            return event::Status::Captured;
                        }
                    }
                } else {
                    // check whether cursor is over head spliter
                    for cell in self.head_spliter_rects.iter() {
                        if cell.fixed && cursor.is_over(cell.rect) {
                            state.is_head_spliter_hover = true;
                            state.col_selected = cell.col;
                            return event::Status::Captured;
                        }
                    }
                    state.is_head_spliter_hover = false;
                }
                // * check whether cursor is over horizontal scrollbar, and scroll the scrollbar
                if state.is_pressed {
                    if state.scrollbar_x.is_thumb_hover {
                        let delta_x = position.x - state.point_pressed.x;
                        state.point_pressed = position;
                        let offset = state.scrollbar_x.offset + delta_x;
                        if offset >= 0.0
                            && offset + state.scrollbar_x.thumb_rect.width
                                <= state.scrollbar_x.track_rect.width
                        {
                            state.scrollbar_x.offset = offset;
                            state.x_offset =
                                offset / state.scrollbar_x.track_rect.width * body_width_real;
                        }
                        return event::Status::Captured;
                    }
                } else {
                    if cursor.is_over(state.scrollbar_x.thumb_rect) {
                        state.scrollbar_x.is_thumb_hover = true;
                        return event::Status::Captured;
                    }
                    state.scrollbar_x.is_thumb_hover = false;

                    if cursor.is_over(state.scrollbar_x.track_rect) {
                        state.scrollbar_x.is_track_hover = true;
                        return event::Status::Captured;
                    }
                    state.scrollbar_x.is_track_hover = false;
                }
                // * check whether cursor is over vertical scrollbar, and scroll the scrollbar
                if state.is_pressed {
                    if state.scrollbar_y.is_thumb_hover {
                        let delta = position.y - state.point_pressed.y;
                        state.point_pressed = position;
                        let offset = state.scrollbar_y.offset + delta;
                        if offset >= 0.0
                            && offset + state.scrollbar_y.thumb_rect.height
                                <= state.scrollbar_y.track_rect.height
                        {
                            state.scrollbar_y.offset = offset;
                            // ! offset_real / real_height = offset / track_height  =>  offset_real = offset / track_height * real_height
                            state.y_offset =
                                offset / state.scrollbar_y.track_rect.height * body_height_real;
                        }
                        return event::Status::Captured;
                    }
                } else {
                    if cursor.is_over(state.scrollbar_y.thumb_rect) {
                        state.scrollbar_y.is_thumb_hover = true;
                        return event::Status::Captured;
                    }
                    state.scrollbar_y.is_thumb_hover = false;

                    if cursor.is_over(state.scrollbar_y.track_rect) {
                        state.scrollbar_y.is_track_hover = true;
                        return event::Status::Captured;
                    }
                    state.scrollbar_y.is_track_hover = false;
                }
                // * check whether can move the row or column
                if state.is_pressed && state.time_pressed.elapsed().unwrap().as_millis() > 100 {
                    if !state.can_col_move && !state.can_row_move {
                        if state.is_cell_hover {
                            if self.on_row_moved.is_some() {
                                if !state.can_row_move {
                                    state.row_selected = state.row_hover;
                                    state.can_row_move = true;
                                }
                            }
                        } else if state.is_head_hover {
                            if self.on_col_moved.is_some() {
                                if !state.can_col_move {
                                    state.col_selected = state.col_hover;
                                    state.can_col_move = true;
                                }
                            }
                        }
                    }
                } else {
                    state.can_row_move = false;
                    state.can_col_move = false;
                }
                // * check whether cursor is over head or cell
                if state.is_hover {
                    // * check whether cursor is over head
                    let mut capture = false;
                    for cell in self.head_rects.iter() {
                        if cursor.is_over(cell.rect) {
                            state.is_head_hover = true;
                            state.col_hover = cell.col;
                            capture = true;
                            break;
                        }
                    }
                    if !capture {
                        state.is_head_hover = false;
                    }
                    // * check whether cursor is over cell
                    if !capture {
                        for cell in self.cell_rects.iter() {
                            if cursor.is_over(cell.rect) {
                                state.is_cell_hover = true;
                                state.row_hover = cell.row;
                                state.col_hover = cell.col;
                                capture = true;
                                break;
                            }
                        }
                    } else {
                        state.is_cell_hover = false;
                    }
                    if capture {
                        return event::Status::Captured;
                    }
                }
                state.is_cell_hover = false;
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();
        let offset_x = bounds.x - state.get_offset_x();
        let offset_y = bounds.y - state.y_offset;

        // ****************** head ******************
        renderer.start_layer(bounds);

        let width = bounds.width;
        // let height = bounds.height;
        let width_fill = self.cal_fill_width(width);
        let style = theme.style(&self.class, Status::Active);

        // ! draw head
        let mut left = offset_x;
        let header_height = self.cal_header_height();

        for (col_idx, head) in self.heads.iter().enumerate() {
            let cell = self.head_rects.get(col_idx);
            if let Some(cell) = cell {
                let rect = cell.rect;
                // * draw head cell background
                renderer.fill_quad(
                    Quad {
                        bounds: rect,
                        border: style.border,
                        ..Default::default()
                    },
                    style
                        .head_backcolor
                        .unwrap_or(Background::Color(Color::TRANSPARENT)),
                );
                // * draw head cell text
                renderer.fill_text(
                    self.get_fill_text(&head.text, 16, rect.size(), head.align_x, head.align_y),
                    self.get_text_position(&rect, head.align_x, head.align_y),
                    style.head_color,
                    rect,
                );
                left += rect.width;
            }
        }

        // ! draw head spliter
        for cell in self.head_spliter_rects.iter() {
            renderer.fill_quad(
                Quad {
                    bounds: cell.rect.shrink([0, 2]),
                    border: style.border,
                    ..Default::default()
                },
                Color::WHITE,
            );
        }

        // ! draw the column dragging of head
        if state.can_col_move {
            let col_idx = state.col_pressed as usize;
            let head = &self.heads.get(col_idx);
            let head_cell = self.head_rects.get(col_idx);
            if let (Some(head), Some(cell), Some(position)) = (head, head_cell, cursor.position()) {
                let mut rect = cell.rect.clone();
                rect.x = position.x;
                // * draw cell background
                renderer.fill_quad(
                    Quad {
                        bounds: rect,
                        border: style.border,
                        ..Default::default()
                    },
                    style
                        .selected_row_background
                        .unwrap_or(Background::Color(Color::TRANSPARENT)),
                );
                // * draw cell text
                renderer.fill_text(
                    self.get_fill_text(&head.text, 16, rect.size(), head.align_x, head.align_y),
                    self.get_text_position(&rect, head.align_x, head.align_y),
                    style.text_color,
                    rect,
                );
            }
        }

        renderer.end_layer();

        // ****************** body ******************
        let bounds = Rectangle {
            x: bounds.x,
            y: bounds.y + header_height,
            width: bounds.width,
            height: bounds.height - header_height,
        };
        renderer.start_layer(bounds);

        let mut top = offset_y + header_height;

        // ! draw rows
        let row_height = self.cal_row_height();
        let bottom = bounds.y + bounds.height;
        for (row_idx, row) in self.source.iter().enumerate() {
            left = offset_x;

            // * background color
            let background_color = style.content_background;
            let background = if state.is_cell_hover && state.row_hover == (row_idx as i32) {
                style
                    .hover_row_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT))
            } else if state.row_selected == (row_idx as i32) {
                style
                    .selected_row_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT))
            } else if self.show_stripe {
                if let Some(background_color) = background_color {
                    if row_idx % 2 == 0 {
                        background_color.scale_alpha(0.2)
                    } else {
                        background_color.scale_alpha(0.8)
                    }
                } else {
                    Background::Color(if row_idx % 2 == 0 {
                        Color::WHITE.scale_alpha(0.2)
                    } else {
                        Color::BLACK.scale_alpha(0.2)
                    })
                }
            } else {
                background_color.unwrap_or(Background::Color(Color::TRANSPARENT))
            };

            // * draw cells
            for head in self.heads.iter() {
                let value = match head.head_type {
                    TableHeadType::Normal => row.get_value(&head.name),
                    TableHeadType::Index => (row_idx + 1).to_string(),
                };
                let col_width = self.cal_col_width(head, width_fill);
                let left_top = Point::new(left, top);
                let size = Size::new(col_width, row_height);
                let rect = Rectangle::new(left_top, size);

                // * draw cell background
                renderer.fill_quad(
                    Quad {
                        bounds: rect,
                        border: style.border,
                        ..Default::default()
                    },
                    background,
                );
                // * draw cell text
                renderer.fill_text(
                    self.get_fill_text(value, 16, rect.size(), head.align_x, head.align_y),
                    self.get_text_position(&rect, head.align_x, head.align_y),
                    style.text_color,
                    rect,
                );
                left += col_width;
            }
            top += row_height;

            // * if the rows are not visible, then we don't need to draw them.
            if top > bottom {
                break;
            }
        }

        // ! draw scroll bar
        if state.is_hover || state.is_pressed {
            // * x y scrollbar cross point
            if state.scrollbar_x.show && state.scrollbar_y.show {
                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: state.scrollbar_y.track_rect.x,
                            y: state.scrollbar_x.track_rect.y,
                            width: state.track_thin_size,
                            height: state.track_thin_size,
                        },
                        ..Default::default()
                    },
                    style.scrollbar_track_color,
                );
            }

            // * horizontal scroll bar
            if state.scrollbar_x.show {
                renderer.fill_quad(
                    Quad {
                        bounds: state.scrollbar_x.track_rect,
                        border: border::rounded(state.track_thin_size / 2.0),
                        ..Default::default()
                    },
                    style.scrollbar_track_color,
                );
                renderer.fill_quad(
                    Quad {
                        bounds: state.scrollbar_x.thumb_rect,
                        border: border::rounded(state.track_thin_size / 2.0),
                        ..Default::default()
                    },
                    if state.is_pressed && state.scrollbar_x.is_thumb_hover {
                        style.scrollbar_thumb_active_color
                    } else if state.scrollbar_x.is_thumb_hover {
                        style.scrollbar_thumb_hover_color
                    } else {
                        style.scrollbar_thumb_color
                    },
                );
            }

            // * vertical scroll bar
            if state.scrollbar_y.show {
                renderer.fill_quad(
                    Quad {
                        bounds: state.scrollbar_y.track_rect,
                        border: border::rounded(state.track_thin_size / 2.0),
                        ..Default::default()
                    },
                    style.scrollbar_track_color,
                );
                renderer.fill_quad(
                    Quad {
                        bounds: state.scrollbar_y.thumb_rect,
                        border: border::rounded(state.track_thin_size / 2.0),
                        ..Default::default()
                    },
                    if state.is_pressed && state.scrollbar_y.is_thumb_hover {
                        style.scrollbar_thumb_active_color
                    } else if state.scrollbar_y.is_thumb_hover {
                        style.scrollbar_thumb_hover_color
                    } else {
                        style.scrollbar_thumb_color
                    },
                );
            }
        }

        // ! draw the row dragging
        if state.can_row_move {
            let row_idx = state.row_pressed as usize;
            let row = self.source.get(row_idx);
            if let Some(row) = row {
                for (col_idx, head) in self.heads.iter().enumerate() {
                    let value = match head.head_type {
                        TableHeadType::Normal => row.get_value(&head.name),
                        TableHeadType::Index => (row_idx + 1).to_string(),
                    };
                    let cell = self
                        .cell_rects
                        .iter()
                        .find(|x| x.col == col_idx as i32 && x.row == row_idx as i32);
                    if let (Some(cell), Some(position)) = (cell, cursor.position()) {
                        let mut rect = cell.rect.clone();
                        rect.y = position.y;
                        // * draw cell background
                        renderer.fill_quad(
                            Quad {
                                bounds: rect,
                                border: style.border,
                                ..Default::default()
                            },
                            style
                                .selected_row_background
                                .unwrap_or(Background::Color(Color::TRANSPARENT)),
                        );
                        // * draw cell text
                        renderer.fill_text(
                            self.get_fill_text(value, 16, rect.size(), head.align_x, head.align_y),
                            self.get_text_position(&rect, head.align_x, head.align_y),
                            style.text_color,
                            rect,
                        );
                    }
                }
            }
        }

        // ! draw the column dragging of content
        if state.can_col_move {
            let col_idx = state.col_pressed as usize;
            let head = &self.heads.get(col_idx);
            if let Some(head) = head {
                for (row_idx, row) in self.source.iter().enumerate() {
                    let value = match head.head_type {
                        TableHeadType::Normal => row.get_value(&head.name),
                        TableHeadType::Index => (row_idx + 1).to_string(),
                    };
                    let rect = self
                        .cell_rects
                        .iter()
                        .find(|x| x.row == row_idx as i32 && x.col == col_idx as i32);
                    if let (Some(rect), Some(position)) = (rect, cursor.position()) {
                        let mut rect = rect.rect.clone();
                        rect.x = position.x;
                        // * draw cell background
                        renderer.fill_quad(
                            Quad {
                                bounds: rect,
                                border: style.border,
                                ..Default::default()
                            },
                            style.hover_row_background.unwrap(),
                        );
                        // * draw cell text
                        renderer.fill_text(
                            self.get_fill_text(value, 16, rect.size(), head.align_x, head.align_y),
                            self.get_text_position(&rect, head.align_x, head.align_y),
                            style.text_color,
                            rect,
                        );
                    }
                }
            }
        }
        renderer.end_layer();
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let is_mouse_over = cursor.is_over(layout.bounds());

        if state.can_row_move || state.can_col_move {
            // TODO: the grabbing cursor should like a hand grabbing, but there it is a cross.
            mouse::Interaction::Grabbing
        } else if is_mouse_over {
            if state.is_head_spliter_hover {
                mouse::Interaction::ResizingHorizontally
            } else if self.on_row_selected.is_some() {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            }
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let _ = tree;
        let _ = layout;
        let _ = renderer;
        let _ = translation;
        None
    }
}

impl<'a, T, Message, Theme, Renderer> From<Table<'a, T, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    T: 'a + TableRow,
{
    fn from(table: Table<'a, T, Message, Theme>) -> Self {
        Self::new(table)
    }
}

// ! table head
/// table head format
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct TableHead {
    /// column name to link the source data
    pub name: String,
    /// column text to show in table
    pub text: String,
    /// column width
    pub width: Length,
    /// align for x axis, default is Center aligned
    pub align_x: Alignment,
    /// align for y axis, default is Center aligned
    pub align_y: Alignment,
    /// the type of the head, default is Normal
    pub head_type: TableHeadType,
}

impl Default for TableHead {
    fn default() -> Self {
        Self {
            name: String::new(),
            text: String::new(),
            width: Length::Fill,
            align_x: Alignment::Center,
            align_y: Alignment::Center,
            head_type: TableHeadType::Normal,
        }
    }
}

#[allow(unused)]
impl TableHead {
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// align center for x axis and y axis
    pub fn align_center(mut self) -> Self {
        self.align_x = Alignment::Center;
        self.align_y = Alignment::Center;
        self
    }

    pub fn align_x(mut self, align_x: Alignment) -> Self {
        self.align_x = align_x;
        self
    }

    pub fn align_y(mut self, align_y: Alignment) -> Self {
        self.align_y = align_y;
        self
    }

    pub fn head_type(mut self, head_type: TableHeadType) -> Self {
        self.head_type = head_type;
        self
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum TableHeadType {
    Normal,
    // Selection,
    /// show a index column in the table, start from 1
    Index,
    // Expand,
}

// ! user event
enum OnRowSelected<'a, Message> {
    // Direct(Message),
    Closure(Box<dyn Fn(i32) -> Message + 'a>),
}

impl<'a, Message: Clone> OnRowSelected<'a, Message> {
    fn get(&self, row: i32, _col: i32) -> Message {
        match self {
            // OnRowSelected::Direct(message) => message.clone(),
            OnRowSelected::Closure(f) => f(row),
        }
    }
}

/// the data after row moved
pub struct RowMovedData {
    /// the source index of the row before moved
    pub src_row_index: i32,
    /// the destination index of the row after moved
    pub dst_row_index: i32,
}
#[allow(unused)]
enum OnRowMoved<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn(RowMovedData) -> Message + 'a>),
}

impl<'a, Message: Clone> OnRowMoved<'a, Message> {
    fn get(&self, src_row_index: i32, dst_row_index: i32) -> Message {
        match self {
            OnRowMoved::Direct(message) => message.clone(),
            OnRowMoved::Closure(f) => f(RowMovedData {
                src_row_index,
                dst_row_index,
            }),
        }
    }
}

/// the data after column moved
pub struct ColMovedData<'a> {
    pub src_col_index: i32,
    pub dst_col_index: i32,
    pub src_col_name: &'a str,
    pub dst_col_name: &'a str,
}

#[allow(unused)]
enum OnColMoved<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn(ColMovedData) -> Message + 'a>),
}

impl<'a, Message: Clone> OnColMoved<'a, Message> {
    fn get(&self, data: ColMovedData) -> Message {
        match self {
            OnColMoved::Direct(message) => message.clone(),
            OnColMoved::Closure(f) => f(data),
        }
    }
}

// ! cell
///  cell for [`Table`], user opration with cell
struct Cell {
    rect: Rectangle,
    row: i32,
    col: i32,
    /// whether the cell width is fixed or not
    fixed: bool,
}

// ! State
/// the state of [`Table`]
#[derive(Debug, Clone, Copy, PartialEq)]
struct State {
    is_pressed: bool,
    point_pressed: Point,
    time_pressed: std::time::SystemTime,
    /// the row index when key or mouse is pressed
    row_pressed: i32,
    /// the column index when key or mouse is pressed
    col_pressed: i32,
    can_row_move: bool,
    can_col_move: bool,

    is_hover: bool,
    is_head_hover: bool,
    is_cell_hover: bool,
    is_head_spliter_hover: bool,
    row_hover: i32,
    col_hover: i32,
    row_selected: i32,
    col_selected: i32,

    x_offset: f32,
    y_offset: f32,

    track_thin_size: f32,

    scrollbar_x: Scrollbar,
    scrollbar_y: Scrollbar,
}

impl Default for State {
    fn default() -> Self {
        // println!("State::default()");
        Self {
            is_pressed: false,
            point_pressed: Point::new(0.0, 0.0),
            time_pressed: std::time::SystemTime::now(),
            row_pressed: -1,
            col_pressed: -1,
            can_row_move: false,
            can_col_move: false,

            is_hover: false,
            is_head_hover: false,
            is_cell_hover: false,
            is_head_spliter_hover: false,
            row_hover: -1,
            col_hover: -1,
            row_selected: -1,
            col_selected: -1,

            x_offset: 0.0,
            y_offset: 0.0,

            track_thin_size: 10.0,

            scrollbar_x: Scrollbar::default(),
            scrollbar_y: Scrollbar::default(),
        }
    }
}

impl State {
    fn get_offset_x(&self) -> f32 {
        if self.scrollbar_x.show {
            self.x_offset
        } else {
            0.0
        }
    }

    fn get_offset_y(&self) -> f32 {
        if self.scrollbar_y.show {
            self.y_offset
        } else {
            0.0
        }
    }
}

/// The possible status of a [`Table`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Table`] can be pressed.
    Active,
    /// The [`Table`] can be pressed and it is being hovered.
    Hovered,
    /// The [`Table`] is being pressed.
    Pressed,
    /// The [`Table`] cannot be pressed.
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Scrollbar {
    show: bool,
    offset: f32,
    is_thumb_hover: bool,
    is_track_hover: bool,
    thumb_rect: Rectangle,
    track_rect: Rectangle,
}

impl Default for Scrollbar {
    fn default() -> Self {
        Self {
            show: false,
            offset: 0.0,
            is_thumb_hover: false,
            is_track_hover: false,
            thumb_rect: Default::default(),
            track_rect: Default::default(),
        }
    }
}

// ! style
/// The style of a table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Background`] of the head.
    pub head_backcolor: Option<Background>,
    /// The text [`Color`] of the head.
    pub head_color: Color,
    /// The [`Background`] of the content.
    pub content_background: Option<Background>,
    /// The text [`Color`] of the content.
    pub text_color: Color,
    /// The [`Background`] of the selected row.
    pub selected_row_background: Option<Background>,
    /// The text [`Color`] of the selected row.
    pub selected_row_color: Color,
    /// The [`Background`] of the hover row.
    pub hover_row_background: Option<Background>,
    /// The text [`Color`] of the hover row.
    pub hover_row_color: Color,

    // ! scrollbar
    pub scrollbar_track_color: Background,
    pub scrollbar_thumb_color: Background,
    pub scrollbar_thumb_hover_color: Background,
    pub scrollbar_thumb_active_color: Background,

    /// The [`Border`] of the table.
    pub border: Border,
    /// The [`Shadow`] of the table.
    pub shadow: Shadow,
}

impl Style {
    /// Updates the head [`Style`] with the given [`Background`] And [`Color`].
    pub fn with_header(self, text_color: Color, background: impl Into<Background>) -> Self {
        Self {
            head_color: text_color,
            head_backcolor: Some(background.into()),
            ..self
        }
    }

    /// Updates the Content [`Style`] with the given [`Background`] And [`Color`].
    pub fn with_content(self, text_color: Color, background: impl Into<Background>) -> Self {
        Self {
            content_background: Some(background.into()),
            text_color,
            ..self
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            head_backcolor: Some(Background::Color(Color::from_rgb8(30, 144, 255))), // dodger blue
            head_color: Color::WHITE,
            content_background: None,
            text_color: Color::BLACK,
            selected_row_background: Some(Background::Color(Color::WHITE.scale_alpha(0.8))),
            selected_row_color: Color::WHITE,
            hover_row_background: Some(Background::Color(Color::WHITE.scale_alpha(0.6))),
            hover_row_color: Color::WHITE,

            scrollbar_track_color: Background::Color(Color::from_rgb8(211, 211, 211)),
            scrollbar_thumb_color: Background::Color(Color::from_rgb8(153, 153, 153)),
            scrollbar_thumb_hover_color: Background::Color(Color::from_rgb8(187, 187, 187)),
            scrollbar_thumb_active_color: Background::Color(Color::from_rgb8(30, 144, 255)),

            border: Border {
                color: Color::from_rgb8(128, 128, 128),
                width: 1.0,
                radius: Radius::new(0.0),
            },
            shadow: Shadow::default(),
        }
    }
}

/// The theme catalog of a [`Table`].
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Table`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// A primary Table; denoting a main action.
pub fn primary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        head_backcolor: Some(Background::Color(palette.primary.base.color)),
        head_color: palette.primary.base.text,

        content_background: Some(Background::Color(palette.secondary.base.color)),
        text_color: palette.secondary.base.text,

        selected_row_background: Some(Background::Color(palette.primary.weak.color)),
        selected_row_color: palette.primary.weak.text,

        hover_row_background: Some(Background::Color(
            palette.primary.weak.color.scale_alpha(0.8),
        )),
        hover_row_color: palette.primary.weak.text,

        scrollbar_track_color: Background::Color(palette.secondary.base.color),
        scrollbar_thumb_color: Background::Color(palette.secondary.strong.color),
        scrollbar_thumb_hover_color: Background::Color(palette.primary.base.color),
        scrollbar_thumb_active_color: Background::Color(palette.primary.strong.color),

        border: border::rounded(0),
        ..Style::default()
    };
    base
}
