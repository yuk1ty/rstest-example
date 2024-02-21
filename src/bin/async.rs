#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};
    use std::collections::HashMap;

    #[derive(Eq, PartialEq, Hash, Clone)]
    struct ArticleId(String);

    struct Article {
        id: ArticleId,
        title: String,
        body: String,
    }

    struct Database {
        articles: HashMap<ArticleId, Article>,
    }

    #[fixture]
    async fn database() -> Database {
        let mut articles = HashMap::new();
        let article1 = Article {
            id: ArticleId("1".to_string()),
            title: "First article".to_string(),
            body: "First article body".to_string(),
        };
        let article2 = Article {
            id: ArticleId("2".to_string()),
            title: "Second article".to_string(),
            body: "Second article body".to_string(),
        };
        let article3 = Article {
            id: ArticleId("3".to_string()),
            title: "Third article".to_string(),
            body: "Third article body".to_string(),
        };
        articles.insert(article1.id.clone(), article1);
        articles.insert(article2.id.clone(), article2);
        articles.insert(article3.id.clone(), article3);
        Database { articles }
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_articles(#[future(awt)] database: Database) {
        assert_eq!(
            database
                .articles
                .get(&ArticleId("1".to_string()))
                .unwrap()
                .title,
            "First article"
        );
    }
}

fn main() {}
