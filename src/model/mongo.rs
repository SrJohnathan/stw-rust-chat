use futures::TryStreamExt;
use mongodb::{bson, Client, Database};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use super::models::Status;

pub async fn connection() -> Result<Database, mongodb::error::Error> {
    let client_options = ClientOptions::parse(
        //"mongodb+srv://stw:l1sLXHUz01OACdof@chat-wp.pmlgafg.mongodb.net/?retryWrites=true&w=majority", // production
        "mongodb+srv://stw:9NAkSlpXLYUB7WgV@cluster0.nniry7o.mongodb.net/?retryWrites=true&w=majority"
    )
        .await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("chat-WP");
    Ok(database)
}

pub struct MongoDb<'r>(pub  &'r Database);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MongoDb<'r>  {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let res =  request.guard::<&State<Database>>().await;
       res.map(|c| MongoDb(c))

    }
}


pub async fn select_status(number: String , app:String, db: &Database) -> Result<Vec<Status>, mongodb::error::Error> {
    let mut bots: Vec<Status> = vec![];
    let filter = doc! { "number": number.as_str() , "app": app.as_str() };
    let typed_collection = db.collection::<Status>("status");
    let mut f = typed_collection.find(filter, None).await.unwrap();
    while let Some(dob) = f.try_next().await? {
        bots.push(dob);
    }
    Ok(bots)
}

pub async fn update_status(st: &Status, db: &Database) -> Result<bool, String> {
    let filter = doc! { "number": st.number.clone() };

    let bso = bson::to_bson(st).unwrap();
    let b = bso.as_document().unwrap();

    let typed_collection = db.collection::<Status>("status");
    let f = typed_collection.update_one(filter, doc! {"$set": b}, None).await;
    match f {
        Ok(v) => Ok(v.modified_count > 0),
        Err(err) => Err(String::from("error em atualizar o status"))
    }
}

pub async fn insert_status(st: &Status, db: &Database) -> Result<bool, String> {
    let typed_collection = db.collection::<Status>("status");
    let f = typed_collection.insert_one(st, None).await;
    match f {
        Ok(v) => Ok(true),
        Err(err) => Err(String::from("error em atualizar o status"))
    }
}