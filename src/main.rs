#[macro_use] extern crate rocket;
use urkel::apis::configuration::Configuration;
use urkel::apis::Error;
use rocket::serde::{json::Json};
use rocket::response::status;
use rocket::Request;
use rocket::http::Status;

/// Endpoints related to Stores
/// Returns a paginated list of OpenFGA stores.
#[get("/stores?<page_size>&<continuation_token>")]
async fn list_stores(page_size: Option<i32>, continuation_token: Option<&str>) -> Result<status::Custom<Json<urkel::models::ListStoresResponse>>, status::Custom<Json<urkel::apis::open_fga_api::ListStoresError>>> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::list_stores(&config, page_size, continuation_token)
        .await {
            Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
            Err(error) => {
                match error {
                    Error::ResponseError(e) =>  {
                        let custom_status = Status::new(e.status.as_u16());
                        Err(status::Custom(custom_status, Json(e.entity.unwrap())))
                    },
                    _ => {
                        let internal_error = urkel::models::InternalErrorMessageResponse {
                            code: Some(urkel::models::InternalErrorCode::InternalError),
                            message: Some("Internal Error.".to_string())
                        };
                        let error_wrapper = urkel::apis::open_fga_api::ListStoresError::Status500(internal_error);
                        Err(status::Custom(Status::InternalServerError, Json(error_wrapper)))
                    }
                }
            }
        }
}

/// Create a unique OpenFGA store which will be used to store authorization models and relationship tuples.
#[post("/stores", format = "json", data = "<body>")]
async fn create_store(body: Json<urkel::models::CreateStoreRequest>) -> Json<urkel::models::CreateStoreResponse> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::create_store(&config, body.into_inner())
        .await {
            Ok(result) => Json(result),
            Err(_) => todo!()
        }
}

/// Returns an OpenFGA store by its identifier
#[get("/stores/<store_id>")]
async fn get_store(store_id: &str) -> Json<urkel::models::GetStoreResponse> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::get_store(&config, store_id)
        .await {
            Ok(result) => Json(result),
            Err(_) => todo!()
        }
}

/// Delete an OpenFGA store. This does not delete the data associated with the store, like tuples or authorization models.
// #[delete("/stores/<store_id>")]

/// Endpoints related to Authorization Models
/// The ReadAuthorizationModels API will return all the authorization models for a certain store.
/// OpenFGA's response will contain an array of all authorization models, sorted in descending order of creation.
#[get("/stores/<store_id>/authorization-models?<page_size>&<continuation_token>")]
async fn list_models(store_id: &str, page_size: Option<i32>, continuation_token: Option<&str>) -> Json<urkel::models::ReadAuthorizationModelsResponse> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::read_authorization_models(&config, store_id, page_size, continuation_token)
        .await {
            Ok(result) => Json(result),
            Err(_) => todo!()
        }
}

/// The WriteAuthorizationModel API will add a new authorization model to a store.
/// Each item in the type_definitions array is a type definition as specified in the field type_definition.
/// The response will return the authorization model's ID in the id field.
// #[post("/stores/<store_id>/authorization-models")]

/// The ReadAuthorizationModel API returns an authorization model by its identifier.
/// The response will return the authorization model for the particular version.
#[get("/stores/<store_id>/authorization-models/<id>")]
async fn get_model(store_id: &str, id: &str) -> Json<urkel::models::ReadAuthorizationModelResponse> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::read_authorization_model(&config, store_id, id)
        .await {
            Ok(result) => Json(result),
            Err(_) => todo!()
        }
}

/// Endpoints related to Relationship Tuples
/// The ReadChanges API will return a paginated list of tuple changes (additions and deletions) that occurred 
/// in a given store, sorted by ascending time. The response will include a continuation token that is used 
/// to get the next set of changes. If there are no changes after the provided continuation token, the same 
/// token will be returned in order for it to be used when new changes are recorded. 
/// If the store never had any tuples added or removed, this token will be empty.
/// You can use the type parameter to only get the list of tuple changes that affect objects of that type.
#[get("/stores/<store_id>/changes?<type>&<page_size>&<continuation_token>")]
async fn list_changes(store_id: &str, r#type: Option<&str>, page_size: Option<i32>, continuation_token: Option<&str>) -> Json<urkel::models::ReadChangesResponse> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::read_changes(&config, store_id, r#type, page_size, continuation_token)
        .await {
            Ok(result) => Json(result),
            Err(_) => todo!()
        }
}

/// The Read API will return the tuples for a certain store that match a query filter specified in the body 
/// of the request. It is different from the /stores/{store_id}/expand API in that it only returns relationship 
/// tuples that are stored in the system and satisfy the query.
/// In the body:
/// 
/// 1. tuple_key is optional. If tuple_key is not specified, it will return all tuples in the store.2. tuple_key.object is mandatory if tuple_key is specified. It can be a full object (e.g., type:object_id) or type only (e.g., type:).
/// 2. tuple_key.user is mandatory if tuple_key is specified in the case the tuple_key.object is a type only.
// #[post("/stores/<store_id>/read")]

/// The Write API will update the tuples for a certain store. Tuples and type definitions allow OpenFGA to determine
/// whether a relationship exists between an object and an user.
/// In the body, writes adds new tuples while deletes removes existing tuples. The API is not idempotent: 
/// if, later on, you try to add the same tuple, or if you try to delete a non-existing tuple, it will throw an 
/// error.
/// An authorization_model_id may be specified in the body. If it is, it will be used to assert that each written 
/// tuple (not deleted) is valid for the model specified. If it is not specified, the latest authorization model 
/// ID will be used.
// #[post("/stores/<store_id>/write")]

/// Endpoints related to Relationship Queries
/// The Check API queries to check if the user has a certain relationship with an object in a certain store.
/// A contextual_tuples object may also be included in the body of the request. This object contains one 
/// field tuple_keys, which is an array of tuple keys.
/// You may also provide an authorization_model_id in the body. This will be used to assert that the input tuple_key
/// is valid for the model specified. If not specified, the assertion will be made against the latest authorization 
/// model ID. It is strongly recommended to specify authorization model id for better performance.
/// The response will return whether the relationship exists in the field allowed.
// #[post("/stores/<store_id>/check")]

/// The Expand API will return all users and usersets that have certain relationship with an object in a certain 
/// store.
/// This is different from the /stores/{store_id}/read API in that both users and computed usersets are returned.
/// Body parameters tuple_key.object and tuple_key.relation are all required.
/// The response will return a tree whose leaves are the specific users and usersets. Union, intersection and 
/// difference operator are located in the intermediate nodes.
// #[post("/stores/<store_id>/expand")]

/// The ListObjects API returns a list of all the objects of the given type that the user has a relation with. 
/// To achieve this, both the store tuples and the authorization model are used.
/// An `authorization_model_id` may be specified in the body. If it is, it will be used to decide the underlying 
/// implementation used. If it is not specified, the latest authorization model ID will be used. 
/// It is strongly recommended to specify authorization model id for better performance.
/// You may also specify `contextual_tuples` that will be treated as regular tuples.
/// The response will contain the related objects in an array in the "objects" field of the response and they 
/// will be strings in the object format `<type>:<id>` (e.g. "document:roadmap").
/// Note: If you have `and` or `but not` in your model while using ListObjects, checkout the [caveats](https://openfga.dev/docs/interacting/relationship-queries#caveats-and-when-not-to-use-it-3).
// #[post("/stores/<store_id>/list-objects")]

/// [EXPERIMENTAL] Streaming variation of the ListObjects API (see ListObjects for more info)
// #[post("/stores/<store_id>/streamed-list-objects")]

/// The ReadAssertions API will return, for a given authorization model id, all the assertions stored for it. 
/// An assertion is an object that contains a tuple key, and the expectation of whether a call to the 
/// Check API of that tuple key will return true or false.
#[get("/stores/<store_id>/assertions/<authorization_model_id>")]
async fn list_assertions(store_id: &str, authorization_model_id: &str) -> Json<urkel::models::ReadAssertionsResponse> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::read_assertions(&config, store_id, authorization_model_id)
        .await {
            Ok(result) => Json(result),
            Err(_) => todo!()
        }
}

/// he WriteAssertions API will upsert new assertions for an authorization model id, or overwrite the 
/// existing ones. An assertion is an object that contains a tuple key, and the expectation of whether 
/// a call to the Check API of that tuple key will return true or false.
// #[put("/stores/<store_id>/assertions/<authorization_model_id>")]

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          EXAMPLE: curl --data-binary @file.txt http://localhost:8000

      GET /stores

          returns a paginated list of OpenFGA stores.
    "
}

#[allow(warnings)]
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, list_stores, create_store, get_store, list_models, get_model, list_changes, list_assertions])
}