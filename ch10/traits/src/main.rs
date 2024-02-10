use traits::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("vtrenton"),
        content: String::from(
            "install gentoo"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanely Cup Chapionship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL."
        ),
    };

    println!("New Article Availible! {}", article.summarize());
}
