#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn display_song_info(self: Self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration (secs): {}", self.duration_secs);
    }

    fn double_length(mut self) {
        self.duration_secs = self.duration_secs * 2;
        println!("{:#?}", self);
    }
}

fn main() {
    let song = TaylorSwiftSong {
        title: String::from("Blank space"),
        release_year: 2014,
        duration_secs: 231,
    };

    song.double_length();
}