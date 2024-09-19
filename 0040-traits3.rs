fn main() {
    
}

pub struct FacebookNews {
    pub title: String,
    pub author: String,
    pub content: String,
}

pub struct XTweet {
    pub user_name: String,
    pub content: String,
}

pub trait SummaryCompanyNews {
    fn company_news(&self) -> String;
}

impl SummaryCompanyNews for FacebookNews {
    fn company_news(&self) -> String {
        format!(
            "News info -> title: {} - author: {} - content: {}",
            self.title, self.author, self.content
        )
    }
}

impl SummaryCompanyNews for XTweet {
    fn company_news(&self) -> String {
        format!(
            "Tweet info -> user name: {} - tweet content: {}",
            self.user_name, self.content
        )
    }
}
