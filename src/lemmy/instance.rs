use lemmy::LemmyPost;

pub fn post() {
    let post = LemmyPost::new(
        String::from("Hello, world!"),
        Some(String::from("This is a test post.")),
        None,
        None,
    );

    println!("{}", post);
}
