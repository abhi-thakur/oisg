use std::io;

use crate::db::{
    self,
    models
};

pub fn get_user_info() -> io::Result<Option<models::UserInfo>> {
    let connection = db::get_connection()?;
    let query = "SELECT USER_NAME, USER_ID, JOINED_AT FROM USER_INFO LIMIT 1";

    let mut found = false;
    let mut res = models::UserInfo::default();
    let _ = connection.iterate(query, |pairs| {
        for &(col, val) in pairs.iter() {
            found = true;
            match col {
                "USER_NAME" => res.user_name = val.unwrap().to_string(),
                "USER_ID" => res.user_id = val.unwrap().to_string(),
                "JOINED_AT" => res.joined_at = val.unwrap().to_string(),
                _ => {}
            }
        }

        false
    });


    if found {
        Ok(Some(res))
    } else {
        Ok(None)
    }
}

pub fn save_user_details(user_info: models::UserInfo) -> io::Result<()> {
    let query = format!(
        "INSERT INTO USER_INFO (USER_NAME, USER_ID) VALUES ('{}', '{}')",
        user_info.user_name, user_info.user_id
    );

    let connection = db::get_connection()?;
    return match connection.execute(query) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.message.unwrap()))
    }
}