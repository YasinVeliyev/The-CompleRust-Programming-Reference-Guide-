struct Audio(String);

struct Video(String);

trait Playable {
    fn play(&self);
    fn pause() {
        println!("Paused")
    }
}

impl Playable for Audio {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

pub fn player() {
    println!("Super player");
    let audio = Audio("ambient_music.mp3".to_string());
    let video = Video("big_buck_bunny.mkv".to_string());
    audio.play();
    video.play();
}
