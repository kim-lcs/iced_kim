/// a icon with unicode char
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CharIcon {
    icon: char,
    font: Option<iced::Font>,
}

#[allow(dead_code)]
impl CharIcon {
    pub fn new(icon: char) -> Self {
        Self { icon, font: None }
    }

    pub fn font(mut self, font: iced::Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn font_maybe(mut self, font: Option<iced::Font>) -> Self {
        self.font = font;
        self
    }

    pub fn smaller<'a>(&self) -> iced::widget::text::Text<'a> {
        self.icon().size(15)
    }

    pub fn small<'a>(&self) -> iced::widget::text::Text<'a> {
        self.icon().size(20)
    }

    pub fn middle<'a>(&self) -> iced::widget::text::Text<'a> {
        self.icon().size(30)
    }

    pub fn large<'a>(&self) -> iced::widget::text::Text<'a> {
        self.icon().size(40)
    }

    pub fn with_color<'a>(&self, color: iced::Color) -> iced::widget::text::Text<'a> {
        self.icon().color(color)
    }

    pub fn icon<'a>(&self) -> iced::widget::text::Text<'a> {
        let text = iced::widget::text(self.icon).align_x(iced::Alignment::Center);
        match self.font {
            Some(font) => text.font(font),
            None => text,
        }
    }
}
