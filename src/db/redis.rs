pub struct Redis {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    connection: Option<Connection>,
}

impl Redis {
    pub fn new(host: &str, port: u16) -> Redis {
        Redis {
            host: host.to_string(),
            port,
            username: None,
            password: None,
            connection: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        // TODO: Implement Redis connection logic
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        // TODO: Implement Redis disconnection logic
        Ok(())
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        conn.set(key, value)?;
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<Option<String>, Error> {
        let mut conn = self.get_connection()?;
        let value: Option<String> = conn.get(key)?;
        Ok(value)
    }

    pub fn delete(&mut self, key: &str) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        let _: () = conn.del(key)?;
        Ok(())
    }

    fn get_connection(&mut self) -> Result<&mut Connection, Error> {
        if self.connection.is_none() {
            self.connect()?;
        }
        Ok(self.connection.as_mut().unwrap())
    }
}

impl Drop for Redis {
    fn drop(&mut self) {
        self.disconnect().unwrap();
    }
}
