use rocket::{get, post, 
    request::Request, 
    response::self, response::Response, response::Responder, 
    http::ContentType, 
    serde::json::Json
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    message: String
}


// impl<'r> Responder<'r, 'static> for CreateResponse {
//     fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
//         let response: CreateResponse = CreateResponse {
//             message: String::from("create success")
//         };
//         Response::build()
//             .header(ContentType::Plain)
//             .sized_body(serde_json::to_string(&response).unwrap().len(), Cursor::new(serde_json::to_string(&response).unwrap()))
//             .ok()
//     }
// }

// #[post("/create")]              // <- route attribute
// pub async fn create() -> Result<CreateResponse, &'static str> {  // <- request handler
//     Ok(CreateResponse {
//         message: String::from("create success!!!")
//     })
// }


#[get("/index")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/create")]              // <- route attribute
pub async fn create() -> Result<Json<CreateResponse>, &'static str> {  // <- request handler
    let response = CreateResponse{
        message: String::from("create success!!!")
    };
    Ok(Json(response))
}
