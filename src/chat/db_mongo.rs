
use futures::TryStreamExt;
use mongodb::{bson, Client, Database};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use serde_json::Value;
use crate::chat::structs::{Chat, ChatDataType};
use crate::chat::structs::list_mongo::ListMongo;
use crate::chat::structs::status::Status;
use crate::chat::structs::text_buttons::{ContentMedia, ContentText, TextButtons};
use crate::chat::structs::text_mongo::TextMongo;

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

pub struct MongoDb<'r>(pub &'r Database);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MongoDb<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let res = request.guard::<&State<Database>>().await;
        res.map(|c| MongoDb(c))
    }
}

impl<'r> MongoDb<'r> {
    pub async fn get_status(&self, number: &String, app: &String) -> Result<Status, String> {
        let filter = doc! { "number": number.as_str() , "app": app.as_str() };
        let typed_collection = self.0.collection::<Status>("status");
        let f = typed_collection.find_one(filter, None).await.unwrap();
        match f {
            None => { Err("Status Vazio".to_string()) }
            Some(s) => { Ok(s) }
        }
    }

    pub async fn update_status(&self, st:&Status) -> Result<bool,mongodb::error::Error> {
        let filter = doc! { "number": st.number.as_str() , "app": st.app.as_str() };
        let bso = bson::to_bson(st).unwrap();
        let b = bso.as_document().unwrap();

        let typed_collection = self.0.collection::<Status>("status");
        let f = typed_collection.update_one(filter, doc! {"$set": b}, None).await;
        match f {
            Ok(v) => Ok(v.modified_count > 0),
            Err(err) => Err(err)
        }
    }

    pub async fn get_chat(&self, number: &String, app: &String) -> Result<ChatDataType, String> {
        let filter = doc! { "index": number.as_str(),"app": app.as_str()};

        println!("inde é {}  e o app  {}", number, app);

        let typed_collection = self.0.collection::<Chat<Value>>("chat");
        let f = typed_collection.find_one(filter, None).await.unwrap();
        match f {
            None => { Err("Status Vazio".to_string()) }
            Some(s) => {
                println!("inde é {:?}  ", s);

                match s.type_field.as_str() {
                    "text" => {
                        let value: TextMongo = serde_json::from_value(s.data).unwrap();
                        let c: Chat<TextMongo> = Chat {
                            id: s.id,
                            index: s.index,
                            app: s.app,
                            data: value,
                            type_field: s.type_field,
                        };
                        Ok(ChatDataType::Text(c))
                    }
                    "list" => {
                        let value: ListMongo = serde_json::from_value(s.data).unwrap();
                        let c: Chat<ListMongo> = Chat {
                            id: s.id,
                            index: s.index,
                            app: s.app,
                            data: value,
                            type_field: s.type_field,
                        };
                        Ok(ChatDataType::List(c))
                    }
                    "quick_reply" => {
                        let value = s.data.get("type").unwrap();

                        if value.as_str().unwrap() == "text" {
                            let value: TextButtons<ContentText> = serde_json::from_value(s.data).unwrap();
                            let c: Chat<TextButtons<ContentText>> = Chat {
                                id: s.id,
                                index: s.index,
                                app: s.app,
                                data: value,
                                type_field: s.type_field,
                            };
                            Ok(ChatDataType::ButtonText(c))
                        } else {
                            let value: TextButtons<ContentMedia> = serde_json::from_value(s.data).unwrap();
                            let c: Chat<TextButtons<ContentMedia>> = Chat {
                                id: s.id,
                                index: s.index,
                                app: s.app,
                                data: value,
                                type_field: s.type_field,
                            };
                            Ok(ChatDataType::ButtonMidia(c))
                        }
                    }

                    _ => { Ok(ChatDataType::Null) }
                }
            }
        }
    }
    pub async fn get_bot(&self) -> Result<Vec<Value>, mongodb::error::Error> {
        let mut bots = Vec::new();
        let typed_collection = self.0.collection::<Value>("bot");
        let mut f = typed_collection.find(None, None).await.unwrap();
        while let Some(dob) = f.try_next().await? {
            bots.push(dob);
        }

        Ok(bots)
    }
    pub async fn set_bot(&self, st: Value) -> Result<bool, String> {



        let bso = bson::to_bson(&st).unwrap();
        let b = bso.as_document().unwrap();

        let filter = doc! {  "app": st.get("app").unwrap().as_str().unwrap() };

        self.delete_bot(st.get("app").unwrap().as_str().unwrap()).await.unwrap();

        let typed_collection = self.0.collection::<Value>("bot");
        typed_collection.insert_one(st, None).await.unwrap();

        Ok(true)
    }
    pub async fn set_chat(&self, st: Value) -> Result<bool, String> {
        let bso = bson::to_bson(&st).unwrap();
        let b = bso.as_document().unwrap();

        let filter = doc! {  "app": st.get("app").unwrap().as_str().unwrap() };
        let typed_collection = self.0.collection::<Value>("chat");

        match typed_collection.insert_one(st, None).await {
            Ok(x) => { Ok(true) }
            Err(e) => { Err("error ao inserir".to_string()) }
        }

        /*  let f = typed_collection.insert_one(filter,doc! {"$set" : b }, None).await;
          match f {
              Ok(v) => {

                  if v.modified_count == 0 {
                      typed_collection.insert_one(st,None).await.unwrap();
                  }
                  Ok(true)

              },
              Err(err) => Err("".to_string())
          } */
    }
    pub async fn delete_chat(&self, app: &str) -> Result<bool, String> {
        let filter = doc! {  "app": app };
        let typed_collection = self.0.collection::<Value>("chat");


        match typed_collection.delete_many(filter, None).await {
            Ok(x) => { Ok(true) }
            Err(e) => { Err("error ao inserir".to_string()) }
        }

        /*  let f = typed_collection.insert_one(filter,doc! {"$set" : b }, None).await;
          match f {
              Ok(v) => {

                  if v.modified_count == 0 {
                      typed_collection.insert_one(st,None).await.unwrap();
                  }
                  Ok(true)

              },
              Err(err) => Err("".to_string())
          } */
    }
    pub async fn delete_bot(&self, app: &str) -> Result<bool, String> {
        let filter = doc! {  "app": app };
        let typed_collection = self.0.collection::<Value>("bot");


        match typed_collection.delete_many(filter, None).await {
            Ok(x) => { Ok(true) }
            Err(e) => { Err("error ao inserir".to_string()) }
        }

        /*  let f = typed_collection.insert_one(filter,doc! {"$set" : b }, None).await;
          match f {
              Ok(v) => {

                  if v.modified_count == 0 {
                      typed_collection.insert_one(st,None).await.unwrap();
                  }
                  Ok(true)

              },
              Err(err) => Err("".to_string())
          } */
    }
}