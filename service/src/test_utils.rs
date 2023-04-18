use abi::Config;
use sqlx_mock_db_tester::TestPg;
use std::ops::Deref;
use std::path::Path;

pub struct TestConfig {
    #[allow(dead_code)]
    tdb: TestPg,
    pub config: Config,
}

impl Deref for TestConfig {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl TestConfig {
    pub fn new() -> Self {
        let mut config = Config::load("fixtures/config.yml").unwrap();
        println!("{}", config.db.server_url());
        let tdb = TestPg::new(config.db.server_url(), Path::new("../migrations"));
        config.db.dbname = tdb.dbname.clone();

        Self { tdb, config }
    }

    pub fn with_server_port(port: u16) -> Self {
        let mut config = TestConfig::new();
        config.config.server.port = port;
        config
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self::new()
    }
}
