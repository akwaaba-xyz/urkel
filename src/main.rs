#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use urkel::apis::configuration::Configuration;
use urkel::apis::Error;

/// Endpoints related to Stores
/// Returns a paginated list of OpenFGA stores.
#[get("/stores?<page_size>&<continuation_token>", format = "json")]
async fn list_stores(
    page_size: Option<i32>,
    continuation_token: Option<&str>,
) -> Result<
    status::Custom<Json<urkel::models::ListStoresResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::list_stores(&config, page_size, continuation_token).await {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// Create a unique OpenFGA store which will be used to store authorization models and relationship tuples.
#[post("/stores", format = "json", data = "<body>")]
async fn create_store(
    body: Json<urkel::models::CreateStoreRequest>,
) -> Result<
    status::Custom<Json<urkel::models::CreateStoreResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::create_store(&config, body.into_inner()).await {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// Returns an OpenFGA store by its identifier
#[get("/stores/<store_id>", format = "json")]
async fn get_store(
    store_id: &str,
) -> Result<
    status::Custom<Json<urkel::models::GetStoreResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::get_store(&config, store_id).await {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// Delete an OpenFGA store. This does not delete the data associated with the store, like tuples or authorization models.
#[delete("/stores/<store_id>")]
async fn delete_store(
    store_id: &str,
) -> Result<status::Custom<()>, status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::delete_store(&config, store_id).await {
        Ok(_) => Ok(status::Custom(Status::Ok, ())),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// Endpoints related to Authorization Models
/// The ReadAuthorizationModels API will return all the authorization models for a certain store.
/// OpenFGA's response will contain an array of all authorization models, sorted in descending order of creation.
#[get(
    "/stores/<store_id>/authorization-models?<page_size>&<continuation_token>",
    format = "json"
)]
async fn list_models(
    store_id: &str,
    page_size: Option<i32>,
    continuation_token: Option<&str>,
) -> Result<
    status::Custom<Json<urkel::models::ReadAuthorizationModelsResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::read_authorization_models(
        &config,
        store_id,
        page_size,
        continuation_token,
    )
    .await
    {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// The WriteAuthorizationModel API will add a new authorization model to a store.
/// Each item in the type_definitions array is a type definition as specified in the field type_definition.
/// The response will return the authorization model's ID in the id field.
#[post(
    "/stores/<store_id>/authorization-models",
    format = "json",
    data = "<body>"
)]
async fn create_model(
    store_id: &str,
    body: Json<urkel::models::WriteAuthorizationModelRequest>,
) -> Result<
    status::Custom<Json<urkel::models::WriteAuthorizationModelResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::write_authorization_model(&config, store_id, body.into_inner())
        .await
    {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// The ReadAuthorizationModel API returns an authorization model by its identifier.
/// The response will return the authorization model for the particular version.
#[get("/stores/<store_id>/authorization-models/<id>", format = "json")]
async fn get_model(
    store_id: &str,
    id: &str,
) -> Result<
    status::Custom<Json<urkel::models::ReadAuthorizationModelResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::read_authorization_model(&config, store_id, id).await {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// Endpoints related to Relationship Tuples
/// The ReadChanges API will return a paginated list of tuple changes (additions and deletions) that occurred
/// in a given store, sorted by ascending time. The response will include a continuation token that is used
/// to get the next set of changes. If there are no changes after the provided continuation token, the same
/// token will be returned in order for it to be used when new changes are recorded.
/// If the store never had any tuples added or removed, this token will be empty.
/// You can use the type parameter to only get the list of tuple changes that affect objects of that type.
#[get(
    "/stores/<store_id>/changes?<type>&<page_size>&<continuation_token>",
    format = "json"
)]
async fn list_changes(
    store_id: &str,
    r#type: Option<&str>,
    page_size: Option<i32>,
    continuation_token: Option<&str>,
) -> Result<
    status::Custom<Json<urkel::models::ReadChangesResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::read_changes(
        &config,
        store_id,
        r#type,
        page_size,
        continuation_token,
    )
    .await
    {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// The Read API will return the tuples for a certain store that match a query filter specified in the body
/// of the request. It is different from the /stores/{store_id}/expand API in that it only returns relationship
/// tuples that are stored in the system and satisfy the query.
/// In the body:
///
/// 1. tuple_key is optional. If tuple_key is not specified, it will return all tuples in the store.2. tuple_key.object is mandatory if tuple_key is specified. It can be a full object (e.g., type:object_id) or type only (e.g., type:).
/// 2. tuple_key.user is mandatory if tuple_key is specified in the case the tuple_key.object is a type only.
#[post("/stores/<store_id>/read", format = "json", data = "<body>")]
async fn read(
    store_id: &str,
    body: Json<urkel::models::ReadRequest>,
) -> Result<
    status::Custom<Json<urkel::models::ReadResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::read(&config, store_id, body.into_inner()).await {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// The Write API will update the tuples for a certain store. Tuples and type definitions allow OpenFGA to determine
/// whether a relationship exists between an object and an user.
/// In the body, writes adds new tuples while deletes removes existing tuples. The API is not idempotent:
/// if, later on, you try to add the same tuple, or if you try to delete a non-existing tuple, it will throw an
/// error.
/// An authorization_model_id may be specified in the body. If it is, it will be used to assert that each written
/// tuple (not deleted) is valid for the model specified. If it is not specified, the latest authorization model
/// ID will be used.
#[post("/stores/<store_id>/write", format = "json", data = "<body>")]
async fn write(
    store_id: &str,
    body: Json<urkel::models::WriteRequest>,
) -> Result<
    status::Custom<Json<serde_json::Value>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::write(&config, store_id, body.into_inner()).await {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// Endpoints related to Relationship Queries
/// The Check API queries to check if the user has a certain relationship with an object in a certain store.
/// A contextual_tuples object may also be included in the body of the request. This object contains one
/// field tuple_keys, which is an array of tuple keys.
/// You may also provide an authorization_model_id in the body. This will be used to assert that the input tuple_key
/// is valid for the model specified. If not specified, the assertion will be made against the latest authorization
/// model ID. It is strongly recommended to specify authorization model id for better performance.
/// The response will return whether the relationship exists in the field allowed.
#[post("/stores/<store_id>/check", format = "json", data = "<body>")]
async fn check(
    store_id: &str,
    body: Json<urkel::models::CheckRequest>,
) -> Result<
    status::Custom<Json<urkel::models::CheckResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::check(&config, store_id, body.into_inner()).await {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// The Expand API will return all users and usersets that have certain relationship with an object in a certain
/// store.
/// This is different from the /stores/{store_id}/read API in that both users and computed usersets are returned.
/// Body parameters tuple_key.object and tuple_key.relation are all required.
/// The response will return a tree whose leaves are the specific users and usersets. Union, intersection and
/// difference operator are located in the intermediate nodes.
#[post("/stores/<store_id>/expand", format = "json", data = "<body>")]
async fn expand(
    store_id: &str,
    body: Json<urkel::models::ExpandRequest>,
) -> Result<
    status::Custom<Json<urkel::models::ExpandResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::expand(&config, store_id, body.into_inner()).await {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// The ListObjects API returns a list of all the objects of the given type that the user has a relation with.
/// To achieve this, both the store tuples and the authorization model are used.
/// An `authorization_model_id` may be specified in the body. If it is, it will be used to decide the underlying
/// implementation used. If it is not specified, the latest authorization model ID will be used.
/// It is strongly recommended to specify authorization model id for better performance.
/// You may also specify `contextual_tuples` that will be treated as regular tuples.
/// The response will contain the related objects in an array in the "objects" field of the response and they
/// will be strings in the object format `<type>:<id>` (e.g. "document:roadmap").
/// Note: If you have `and` or `but not` in your model while using ListObjects, checkout the [caveats](https://openfga.dev/docs/interacting/relationship-queries#caveats-and-when-not-to-use-it-3).
#[post("/stores/<store_id>/list-objects", format = "json", data = "<body>")]
async fn list_objects(
    store_id: &str,
    body: Json<urkel::models::ListObjectsRequest>,
) -> Result<
    status::Custom<Json<urkel::models::ListObjectsResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::list_objects(&config, store_id, body.into_inner()).await {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// The ReadAssertions API will return, for a given authorization model id, all the assertions stored for it.
/// An assertion is an object that contains a tuple key, and the expectation of whether a call to the
/// Check API of that tuple key will return true or false.
#[get(
    "/stores/<store_id>/assertions/<authorization_model_id>",
    format = "json"
)]
async fn list_assertions(
    store_id: &str,
    authorization_model_id: &str,
) -> Result<
    status::Custom<Json<urkel::models::ReadAssertionsResponse>>,
    status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>,
> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::read_assertions(&config, store_id, authorization_model_id)
        .await
    {
        Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

/// The WriteAssertions API will upsert new assertions for an authorization model id, or overwrite the
/// existing ones. An assertion is an object that contains a tuple key, and the expectation of whether
/// a call to the Check API of that tuple key will return true or false.
#[put(
    "/stores/<store_id>/assertions/<authorization_model_id>",
    format = "json",
    data = "<body>"
)]
async fn create_assertions(
    store_id: &str,
    authorization_model_id: &str,
    body: Json<urkel::models::WriteAssertionsRequest>,
) -> Result<status::Custom<()>, status::Custom<Json<urkel::apis::open_fga_api::OpenFGAError>>> {
    let config = Configuration::new();
    match urkel::apis::open_fga_api::write_assertions(
        &config,
        store_id,
        authorization_model_id,
        body.into_inner(),
    )
    .await
    {
        Ok(_) => Ok(status::Custom(Status::Ok, ())),
        Err(error) => match error {
            Error::ResponseError(e) => {
                let custom_status = Status::new(e.status.as_u16());
                Err(status::Custom(custom_status, Json(e.entity.unwrap())))
            }
            _ => {
                let internal_error = urkel::models::InternalErrorMessageResponse {
                    code: Some(urkel::models::InternalErrorCode::InternalError),
                    message: Some("Internal Error.".to_string()),
                };
                let error_wrapper =
                    urkel::apis::open_fga_api::OpenFGAError::Status500(internal_error);
                Err(status::Custom(
                    Status::InternalServerError,
                    Json(error_wrapper),
                ))
            }
        },
    }
}

#[catch(404)]
fn not_found() -> Json<urkel::models::PathUnknownErrorMessageResponse> {
    let path_error = urkel::models::PathUnknownErrorMessageResponse {
        code: Some(urkel::models::NotFoundErrorCode::UndefinedEndpoint),
        message: Some("Not Found".to_string()),
    };
    Json(path_error)
}

#[catch(422)]
fn unprocessable_entity() -> Json<urkel::models::ValidationErrorMessageResponse> {
    let validation_error = urkel::models::ValidationErrorMessageResponse {
        code: Some(urkel::models::ErrorCode::ValidationError),
        message: Some(
            "The request was well-formed but was unable to be followed due to semantic errors."
                .to_string(),
        ),
    };
    Json(validation_error)
}

#[catch(default)]
fn default_catcher() -> Json<urkel::models::InternalErrorMessageResponse> {
    let internal_error = urkel::models::InternalErrorMessageResponse {
        code: Some(urkel::models::InternalErrorCode::InternalError),
        message: Some("Internal error.".to_string()),
    };
    Json(internal_error)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                list_stores,
                create_store,
                get_store,
                delete_store,
                list_models,
                create_model,
                get_model,
                list_changes,
                read,
                write,
                check,
                expand,
                list_objects,
                list_assertions,
                create_assertions
            ],
        )
        .register(
            "/",
            catchers![default_catcher, unprocessable_entity, not_found],
        )
}
