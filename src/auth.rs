use futures::{FutureExt, TryFutureExt};
use mongodb::{bson::doc, Client};

use crate::{error::{Error, ErrorKind, Result}, model::user::User};

pub async fn auth_with_password(
    client: &Client,
    username: &String,
    password: &String,
) -> Result<bool> {
    let db = client.database("auth-web");
    let user_collection = db.collection_with_type::<User>("user");
    let user = user_collection
        .find_one(doc! {"username":username}, None)
        .map_err(Error::from)
        .await?;
    user.map(|user| &user.password == password).ok_or(Error { kind: ErrorKind::AuthenticationError, label: "Authentication failed".to_owned()})
}
