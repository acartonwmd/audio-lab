mod gui;

<<<<<<< HEAD
use gui::beatnik::Beatnik;
use iced::application;

fn main() -> iced::Result {
    application(|| Beatnik::new(), Beatnik::update, Beatnik::view).theme(Beatnik::theme).run()
=======
use iced::application;
use gui::audio_lab::AudioLab;

fn main() -> iced::Result {
    application(|| AudioLab::new(), AudioLab::update, AudioLab::view).run()
>>>>>>> 50895b7 (Initial Commit)
}
