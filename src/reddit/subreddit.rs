use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub selftext: Option<String>,
    pub url: Option<String>,
    pub thumbnail: Option<String>,
}

pub struct Subreddit {
    name: String,
    pub posts: Vec<Post>,
}

impl Subreddit {
    pub fn new(name: String) -> Subreddit {
        Subreddit {
            name,
            posts: Vec::new(),
        }
    }

    pub fn fetch_posts(&mut self) -> Result<(), reqwest::Error> {
        // Make the API request
        println!("Fetching posts from {}...", self.name);

        let user_agent: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15";
        let url: String = format!("https://www.reddit.com/r/{}.json", self.name);

        let client = reqwest::blocking::Client::new();
        let response: reqwest::blocking::Response =
            client.get(&url).header(USER_AGENT, user_agent).send()?;

        // Parse the JSON response
        let data = match response.text() {
            Err(err) => panic!("Failed to fetch response: {}", err),
            Ok(text) => match serde_json::from_str::<serde_json::Value>(&text) {
                Err(err) => panic!("Failed to parse JSON: {}", err),
                Ok(data) => data,
            },
        };

        // Extract the posts from the JSON response
        if let Some(posts) = data["data"]["children"].as_array() {
            for post in posts {
                let post_data: serde_json::Value = post["data"].clone();
                match serde_json::from_value::<Post>(post_data) {
                    Ok(post) => self.posts.push(post),
                    Err(err) => eprintln!("Failed to parse post: {}", err),
                }
            }
        }

        println!("Fetched {} posts", self.posts.len());

        Ok(())
    }
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}\n", self.title)?;

        if let Some(selftext) = &self.selftext {
            write!(f, "Self Text: {}\n", selftext)?;
        }

        if let Some(url) = &self.url {
            write!(f, "URL: {}\n", url)?;
        }

        if let Some(thumbnail) = &self.thumbnail {
            write!(f, "Thumbnail: {}\n", thumbnail)?;
        }

        Ok(())
    }
}
