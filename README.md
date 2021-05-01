# imageboard
Imageboard - Rust backend

## Showcase Video:

[![](http://img.youtube.com/vi/CcgPEFh49TA/0.jpg)](http://www.youtube.com/watch?v=CcgPEFh49TA "")

http://www.youtube.com/watch?v=CcgPEFh49TA

### How to use:
1. Install Rust, check out - https://www.rust-lang.org/tools/install
2. Make sure you are on the Rust nightly build, like so:
`rustup default nightly`
   
3. Head over to src/secrets.rs and put your mongoDB url, make sure you have a database called "imageboard" and a collection called "images"
4. Head over to google cloud, create a new project, enable Vision API, create a service account and download the json.

5. Place the json in the main folder (outside of src), and rename it to "clientsecret.json"
6. You are set now! Just do `cargo build` then `cargo run`.
7. Go to "http://0.0.0.0:3000" in your browser
