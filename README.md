# imageboard
Imageboard - Rust backend(Actix)

## Showcase Video:

[![](http://img.youtube.com/vi/CcgPEFh49TA/0.jpg)](http://www.youtube.com/watch?v=CcgPEFh49TA "")

http://www.youtube.com/watch?v=CcgPEFh49TA

## Features:
* Upload Images
* Each image is labeled with its contents and objects using Google's Vision API
* Anonymous comments on each image

### How to use:
1. Install Rust, check out - https://www.rust-lang.org/tools/install
2. Have Python3 installed, and Google's Vision API, like so: `python3 -m pip install --upgrade google-cloud-vision`
3. Make sure you are on the Rust nightly build, like so:
`rustup default nightly`
   
4. Head over to src/secrets.rs and put your mongoDB url, make sure you have a database called "imageboard" and a collection called "images"
5. Head over to google cloud, create a new project, enable Vision API, create a service account and download the json.

6. Place the json in the main folder (outside of src), and rename it to "clientsecret.json"
7. You are set now! Just do `cargo build` then `cargo run`.
8. Go to "http://0.0.0.0:3000" in your browser
