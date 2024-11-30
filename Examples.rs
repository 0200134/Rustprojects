trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("The Boring News"),
        author: String::from("John Doe"),
        content: String::from("Lorem ipsum..."),
    };

    println!("New article: {}", article.summarize());
}
