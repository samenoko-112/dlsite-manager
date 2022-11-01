use crate::{application::use_application, application_error::Result};
use rusqlite::{params, OptionalExtension, Row};
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct Setting {
    pub download_root_dir: Option<PathBuf>,
}

impl<'stmt> TryFrom<&'stmt Row<'stmt>> for Setting {
    type Error = rusqlite::Error;

    fn try_from(row: &'stmt Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            download_root_dir: row
                .get::<_, Option<String>>("download_root_dir")?
                .map(|path| PathBuf::from(path)),
        })
    }
}

impl Setting {
    pub fn get_ddl() -> &'static str {
        "
CREATE TABLE IF NOT EXISTS settings (
    download_root_dir TEXT
);"
    }

    pub fn get() -> Result<Setting> {
        Ok(use_application()
            .storage()
            .connection()
            .prepare(
                "
SELECT
    download_root_dir
FROM settings;",
            )?
            .query_row((), |row| Self::try_from(row))
            .optional()?
            .unwrap_or_default())
    }

    pub fn set(setting: Setting) -> Result<()> {
        let connection = use_application().storage().connection();
        connection.execute(
            "
DELETE FROM settings",
            (),
        )?;
        connection
            .prepare(
                "
INSERT INTO settings (
    download_root_dir
) VALUES (
    ?1
)",
            )?
            .insert(params![setting
                .download_root_dir
                .as_ref()
                .map(|path| path.to_str().unwrap())])?;

        Ok(())
    }
}
