mod pages;

fn main() -> iced::Result {
    // run the home page easily
    iced_kim::run(pages::home::Data::default(), iced::Font::MONOSPACE)
}
