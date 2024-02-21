use std::net::Ipv6Addr;

struct Config {
    host: Ipv6Addr,
    port: u16,
    database: String,
    username: String,
    password: String,
}

struct Connection;

struct DB {
    conn: Connection,
}

impl DB {
    fn new(config: Config) -> Result<Self, &'static str> {
        Ok(Self {
            conn: Self::connect(config)?,
        })
    }

    fn connect(_config: Config) -> Result<Connection, &'static str> {
        // データベースに接続する
        Ok(Connection)
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        // データベースから切断する
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv6Addr;

    use rstest::{fixture, rstest};

    use crate::{Config, DB};

    #[fixture]
    fn db() -> DB {
        let config = super::Config {
            host: Ipv6Addr::LOCALHOST,
            port: 5432,
            database: "mydb".to_string(),
            username: "myuser".to_string(),
            password: "mypass".to_string(),
        };
        DB::new(config).unwrap()
    }

    #[rstest]
    fn should_connect_database(db: DB) {
        // データベースを使ったテスト
    }

    // 他のテストでも同様にconfigを使う場合
}

fn main() {}
