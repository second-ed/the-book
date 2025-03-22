pub trait Summary {
    fn summarise(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
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
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn use_traits() {
    let tweet = Tweet {
        username: String::from("big_al_from_the_winchester"),
        content: String::from("dogs can't look up"),
        reply: false,
        retweet: false,
    };

    dbg!(tweet.summarise());

    let article = NewsArticle {
        headline: String::from("Dogs definitely can look up"),
        location: String::from("Everywhere"),
        author: String::from("a proper journalist"),
        content: String::from("Everyone is sure that dogs can look up"),
    };

    dbg!(article.summarise());

    notify(&tweet);
    notify(&article);
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarise());
}
