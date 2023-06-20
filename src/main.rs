mod utils;
use utils::config::Config;

mod reddit;
use reddit::subreddit::Subreddit;

fn main() {
    let config: Config = Config::load();

    let mut subreddits: Vec<Subreddit> = Vec::new();

    for subreddit in config.subreddits {
        println!("Subreddit to scrape: {}", subreddit);
        subreddits.push(Subreddit::new(subreddit));
    }

    for subreddit in &mut subreddits {
        if let Err(err) = subreddit.fetch_posts() {
            eprintln!("Failed to fetch posts: {}", err);
        } else {
            // Posts fetched successfully
            println!("Fetching posts was successful!")
        }
    }

    println!("{}", subreddits[0].posts[3]);
}
