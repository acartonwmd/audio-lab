mod gui;

use gui::beatnik::Beatnik;
use iced::application;

fn main() -> iced::Result {
    application(|| Beatnik::new(), Beatnik::update, Beatnik::view).theme(Beatnik::theme).run()
}
