use std::fs::File;
use std::io::Read;
mod bookmark;

fn main() -> std::io::Result<()> {
    let mut s = String::new();
    File::open("/home/pierre/.local/share/bookmark/bookmarks.json")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let bookmarks: bookmark::Bookmark = serde_json::from_str(&s).unwrap();

    println!("deserialized = {:?}", bookmarks);
    Ok(())
}
