use std::fs::OpenOptions;

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .expect("Ich kann nicht File geöffnet");

    println!("File hat geöffnet: {:?}", file);
}
