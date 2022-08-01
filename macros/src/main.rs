use std::io::Error;

fn main() {
    println!("Hello, world!");
}


trait Karaoke {
    fn pick_song_i_know(&self, song_i_know: Vec<String>, song_list: Vec<String>) -> Option<String>;
    fn sing(&self, song: String) -> Result<(), Error>;
}


struct Bubbles{}

impl Karaoke for Bubbles {
    fn pick_song_i_know(&self, song_i_know: Vec<String>, song_list: Vec<String>) -> Option<String> {
        let mut picked_song = None;

        for song in song_list {
            if song_i_know.contains(&song){
                picked_song = Some(song);
                break;
            }
        }
        picked_song
    }

    fn sing(&self, song: String) -> Result<(), Error> {
        todo!()
    }
}
