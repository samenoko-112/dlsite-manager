use crate::{
    application_error::Result,
    database::tables::v1::{
        AccountTable, DisplayLanguageSettingTable, LatestProductQueryTable, ProductTable,
        SettingTable,
    },
};
use rusqlite::Connection;
use std::path::Path;

pub struct Storage {
    connection: Connection,
}

impl Storage {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            connection: Connection::open(path)?.into(),
        })
    }

    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }

    pub fn prepare(&self) -> Result<()> {
        self.connection.execute_batch(&format!(
            "
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

BEGIN;
{}
{}
{}
{}
{}
COMMIT;
",
            SettingTable::get_ddl(),
            DisplayLanguageSettingTable::get_ddl(),
            AccountTable::get_ddl(),
            ProductTable::get_ddl(),
            LatestProductQueryTable::get_ddl(),
        ))?;

        Ok(())
    }

    pub fn drop(self) -> Result<()> {
        self.connection.close().map_err(|(_, err)| err)?;
        Ok(())
    }
}
