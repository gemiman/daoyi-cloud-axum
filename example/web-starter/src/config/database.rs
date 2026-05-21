use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    protocol: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    schema: Option<String>,
}

impl DatabaseConfig {
    pub fn protocol(&self) -> &str {
        self.protocol.as_deref().unwrap_or("mysql")
    }

    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3306)
    }

    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("root")
    }

    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("123456")
    }

    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("demo")
    }

    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("")
    }
}
