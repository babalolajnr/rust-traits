pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let news_article = NewsArticle {
        author: "Abdulqudduus Babalola".to_string(),
        content: "Hello my name is Abdulqudduus Babalola and I am a Software Engineer".to_string(),
        headline: "The Software Engineer".to_string(),
        location: "Nigeria".to_string(),
    };

    let news_article = &news_article.summarize();
    println!("{}", news_article);

}