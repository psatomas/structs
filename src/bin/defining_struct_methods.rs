struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl TaylorSwiftSong {
    fn display_song_info(self: Self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration (secs): {}", self.duration_secs);
    }
}

fn main() {
    let song = TaylorSwiftSong {
        title: String::from("blank space"),
        release_year: 2014,
        duration_secs: 231,
    };

    song.display_song_info();
}