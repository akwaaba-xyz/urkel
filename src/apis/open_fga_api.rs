/*
 * OpenFGA
 *
 * A high performance and flexible authorization/permission engine built for developers and inspired by Google Zanzibar.
 *
 * The version of the OpenAPI document: 0.1
 * Contact: community@openfga.dev
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of all API methods
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenFGAError {
    Status400(crate::models::ValidationErrorMessageResponse),
    Status404(crate::models::PathUnknownErrorMessageResponse),
    Status500(crate::models::InternalErrorMessageResponse),
    UnknownValue(serde_json::Value),
}

/// The Check API queries to check if the user has a certain relationship with an object in a certain store. A `contextual_tuples` object may also be included in the body of the request. This object contains one field `tuple_keys`, which is an array of tuple keys. You may also provide an `authorization_model_id` in the body. This will be used to assert that the input `tuple_key` is valid for the model specified. If not specified, the assertion will be made against the latest authorization model ID. It is strongly recommended to specify authorization model id for better performance. The response will return whether the relationship exists in the field `allowed`.  ## Example In order to check if user `user:anne` of type `user` has a `reader` relationship with object `document:2021-budget` given the following contextual tuple ```json {   \"user\": \"user:anne\",   \"relation\": \"member\",   \"object\": \"time_slot:office_hours\" } ``` the Check API can be used with the following request body: ```json {   \"tuple_key\": {     \"user\": \"user:anne\",     \"relation\": \"reader\",     \"object\": \"document:2021-budget\"   },   \"contextual_tuples\": {     \"tuple_keys\": [       {         \"user\": \"user:anne\",         \"relation\": \"member\",         \"object\": \"time_slot:office_hours\"       }     ]   },   \"authorization_model_id\": \"01G50QVV17PECNVAHX1GG4Y5NC\" } ``` OpenFGA's response will include `{ \"allowed\": true }` if there is a relationship and `{ \"allowed\": false }` if there isn't.
pub async fn check(
    configuration: &configuration::Configuration,
    store_id: &str,
    body: crate::models::CheckRequest,
) -> Result<crate::models::CheckResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/check",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a unique OpenFGA store which will be used to store authorization models and relationship tuples.
pub async fn create_store(
    configuration: &configuration::Configuration,
    body: crate::models::CreateStoreRequest,
) -> Result<crate::models::CreateStoreResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/stores", configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an OpenFGA store. This does not delete the data associated with the store, like tuples or authorization models.
pub async fn delete_store(
    configuration: &configuration::Configuration,
    store_id: &str,
) -> Result<(), Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Expand API will return all users and usersets that have certain relationship with an object in a certain store. This is different from the `/stores/{store_id}/read` API in that both users and computed usersets are returned. Body parameters `tuple_key.object` and `tuple_key.relation` are all required. The response will return a tree whose leaves are the specific users and usersets. Union, intersection and difference operator are located in the intermediate nodes.  ## Example To expand all users that have the `reader` relationship with object `document:2021-budget`, use the Expand API with the following request body ```json {   \"tuple_key\": {     \"object\": \"document:2021-budget\",     \"relation\": \"reader\"   },   \"authorization_model_id\": \"01G50QVV17PECNVAHX1GG4Y5NC\" } ``` OpenFGA's response will be a userset tree of the users and usersets that have read access to the document. ```json {   \"tree\":{     \"root\":{       \"type\":\"document:2021-budget#reader\",       \"union\":{         \"nodes\":[           {             \"type\":\"document:2021-budget#reader\",             \"leaf\":{               \"users\":{                 \"users\":[                   \"user:bob\"                 ]               }             }           },           {             \"type\":\"document:2021-budget#reader\",             \"leaf\":{               \"computed\":{                 \"userset\":\"document:2021-budget#writer\"               }             }           }         ]       }     }   } } ``` The caller can then call expand API for the `writer` relationship for the `document:2021-budget`.
pub async fn expand(
    configuration: &configuration::Configuration,
    store_id: &str,
    body: crate::models::ExpandRequest,
) -> Result<crate::models::ExpandResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/expand",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an OpenFGA store by its identifier
pub async fn get_store(
    configuration: &configuration::Configuration,
    store_id: &str,
) -> Result<crate::models::GetStoreResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The ListObjects API returns a list of all the objects of the given type that the user has a relation with. To achieve this, both the store tuples and the authorization model are used. An `authorization_model_id` may be specified in the body. If it is, it will be used to decide the underlying implementation used. If it is not specified, the latest authorization model ID will be used. It is strongly recommended to specify authorization model id for better performance. You may also specify `contextual_tuples` that will be treated as regular tuples. The response will contain the related objects in an array in the \"objects\" field of the response and they will be strings in the object format `<type>:<id>` (e.g. \"document:roadmap\"). Note: If you have `and` or `but not` in your model while using ListObjects, checkout the [caveats](https://openfga.dev/docs/interacting/relationship-queries#caveats-and-when-not-to-use-it-3).
pub async fn list_objects(
    configuration: &configuration::Configuration,
    store_id: &str,
    body: crate::models::ListObjectsRequest,
) -> Result<crate::models::ListObjectsResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/list-objects",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a paginated list of OpenFGA stores.
pub async fn list_stores(
    configuration: &configuration::Configuration,
    page_size: Option<i32>,
    continuation_token: Option<&str>,
) -> Result<crate::models::ListStoresResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/stores", configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = continuation_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("continuation_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Read API will return the tuples for a certain store that match a query filter specified in the body of the request. It is different from the `/stores/{store_id}/expand` API in that it only returns relationship tuples that are stored in the system and satisfy the query.  In the body: 1. tuple_key is optional.  If tuple_key is not specified, it will return all tuples in the store.2. `tuple_key.object` is mandatory if tuple_key is specified. It can be a full object (e.g., `type:object_id`) or type only (e.g., `type:`). 3. `tuple_key.user` is mandatory if tuple_key is specified in the case the `tuple_key.object` is a type only. ## Examples ### Query for all objects in a type definition To query for all objects that `user:bob` has `reader` relationship in the document type definition, call read API with body of ```json {  \"tuple_key\": {      \"user\": \"user:bob\",      \"relation\": \"reader\",      \"object\": \"document:\"   } } ``` The API will return tuples and an optional continuation token, something like ```json {   \"tuples\": [     {       \"key\": {         \"user\": \"user:bob\",         \"relation\": \"reader\",         \"object\": \"document:2021-budget\"       },       \"timestamp\": \"2021-10-06T15:32:11.128Z\"     }   ] } ``` This means that `user:bob` has a `reader` relationship with 1 document `document:2021-budget`. ### Query for all stored relationship tuples that have a particular relation and object To query for all users that have `reader` relationship with `document:2021-budget`, call read API with body of  ```json {   \"tuple_key\": {      \"object\": \"document:2021-budget\",      \"relation\": \"reader\"    } } ``` The API will return something like  ```json {   \"tuples\": [     {       \"key\": {         \"user\": \"user:bob\",         \"relation\": \"reader\",         \"object\": \"document:2021-budget\"       },       \"timestamp\": \"2021-10-06T15:32:11.128Z\"     }   ] } ``` This means that `document:2021-budget` has 1 `reader` (`user:bob`).  Note that the API will not return writers such as `user:anne` even when all writers are readers.  This is because only direct relationship are returned for the READ API. ### Query for all users with all relationships for a particular document To query for all users that have any relationship with `document:2021-budget`, call read API with body of  ```json {   \"tuple_key\": {       \"object\": \"document:2021-budget\"    } } ``` The API will return something like  ```json {   \"tuples\": [     {       \"key\": {         \"user\": \"user:anne\",         \"relation\": \"writer\",         \"object\": \"document:2021-budget\"       },       \"timestamp\": \"2021-10-05T13:42:12.356Z\"     },     {       \"key\": {         \"user\": \"user:bob\",         \"relation\": \"reader\",         \"object\": \"document:2021-budget\"       },       \"timestamp\": \"2021-10-06T15:32:11.128Z\"     }   ] } ``` This means that `document:2021-budget` has 1 `reader` (`user:bob`) and 1 `writer` (`user:anne`).
pub async fn read(
    configuration: &configuration::Configuration,
    store_id: &str,
    body: crate::models::ReadRequest,
) -> Result<crate::models::ReadResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/read",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The ReadAssertions API will return, for a given authorization model id, all the assertions stored for it. An assertion is an object that contains a tuple key, and the expectation of whether a call to the Check API of that tuple key will return true or false.
pub async fn read_assertions(
    configuration: &configuration::Configuration,
    store_id: &str,
    authorization_model_id: &str,
) -> Result<crate::models::ReadAssertionsResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/assertions/{authorization_model_id}",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id),
        authorization_model_id = crate::apis::urlencode(authorization_model_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The ReadAuthorizationModel API returns an authorization model by its identifier. The response will return the authorization model for the particular version.  ## Example To retrieve the authorization model with ID `01G5JAVJ41T49E9TT3SKVS7X1J` for the store, call the GET authorization-models by ID API with `01G5JAVJ41T49E9TT3SKVS7X1J` as the `id` path parameter.  The API will return: ```json {   \"authorization_model\":{     \"id\":\"01G5JAVJ41T49E9TT3SKVS7X1J\",     \"type_definitions\":[       {         \"type\":\"user\"       },       {         \"type\":\"document\",         \"relations\":{           \"reader\":{             \"union\":{               \"child\":[                 {                   \"this\":{}                 },                 {                   \"computedUserset\":{                     \"object\":\"\",                     \"relation\":\"writer\"                   }                 }               ]             }           },           \"writer\":{             \"this\":{}           }         }       }     ]   } } ``` In the above example, there are 2 types (`user` and `document`). The `document` type has 2 relations (`writer` and `reader`).
pub async fn read_authorization_model(
    configuration: &configuration::Configuration,
    store_id: &str,
    id: &str,
) -> Result<crate::models::ReadAuthorizationModelResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/authorization-models/{id}",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id),
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The ReadAuthorizationModels API will return all the authorization models for a certain store. OpenFGA's response will contain an array of all authorization models, sorted in descending order of creation.  ## Example Assume that a store's authorization model has been configured twice. To get all the authorization models that have been created in this store, call GET authorization-models. The API will return a response that looks like: ```json {   \"authorization_models\": [     {       \"id\": \"01G50QVV17PECNVAHX1GG4Y5NC\",       \"type_definitions\": [...]     },     {       \"id\": \"01G4ZW8F4A07AKQ8RHSVG9RW04\",       \"type_definitions\": [...]     },   ] } ``` If there are more authorization models available, the response will contain an extra field `continuation_token`: ```json {   \"authorization_models\": [     {       \"id\": \"01G50QVV17PECNVAHX1GG4Y5NC\",       \"type_definitions\": [...]     },     {       \"id\": \"01G4ZW8F4A07AKQ8RHSVG9RW04\",       \"type_definitions\": [...]     },   ],   \"continuation_token\": \"eyJwayI6IkxBVEVTVF9OU0NPTkZJR19hdXRoMHN0b3JlIiwic2siOiIxem1qbXF3MWZLZExTcUoyN01MdTdqTjh0cWgifQ==\" } ```
pub async fn read_authorization_models(
    configuration: &configuration::Configuration,
    store_id: &str,
    page_size: Option<i32>,
    continuation_token: Option<&str>,
) -> Result<crate::models::ReadAuthorizationModelsResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/authorization-models",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = continuation_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("continuation_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The ReadChanges API will return a paginated list of tuple changes (additions and deletions) that occurred in a given store, sorted by ascending time. The response will include a continuation token that is used to get the next set of changes. If there are no changes after the provided continuation token, the same token will be returned in order for it to be used when new changes are recorded. If the store never had any tuples added or removed, this token will be empty. You can use the `type` parameter to only get the list of tuple changes that affect objects of that type.
pub async fn read_changes(
    configuration: &configuration::Configuration,
    store_id: &str,
    r#type: Option<&str>,
    page_size: Option<i32>,
    continuation_token: Option<&str>,
) -> Result<crate::models::ReadChangesResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/changes",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = continuation_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("continuation_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates an existing store.
pub async fn update_store(
    configuration: &configuration::Configuration,
    store_id: &str,
    body: crate::models::UpdateStoreRequest,
) -> Result<crate::models::UpdateStoreResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Write API will update the tuples for a certain store. Tuples and type definitions allow OpenFGA to determine whether a relationship exists between an object and an user. In the body, `writes` adds new tuples while `deletes` removes existing tuples. The API is not idempotent: if, later on, you try to add the same tuple, or if you try to delete a non-existing tuple, it will throw an error. An `authorization_model_id` may be specified in the body. If it is, it will be used to assert that each written tuple (not deleted) is valid for the model specified. If it is not specified, the latest authorization model ID will be used. ## Example ### Adding relationships To add `user:anne` as a `writer` for `document:2021-budget`, call write API with the following  ```json {   \"writes\": {     \"tuple_keys\": [       {         \"user\": \"user:anne\",         \"relation\": \"writer\",         \"object\": \"document:2021-budget\"       }     ]   },   \"authorization_model_id\": \"01G50QVV17PECNVAHX1GG4Y5NC\" } ``` ### Removing relationships To remove `user:bob` as a `reader` for `document:2021-budget`, call write API with the following  ```json {   \"deletes\": {     \"tuple_keys\": [       {         \"user\": \"user:bob\",         \"relation\": \"reader\",         \"object\": \"document:2021-budget\"       }     ]   } } ```
pub async fn write(
    configuration: &configuration::Configuration,
    store_id: &str,
    body: crate::models::WriteRequest,
) -> Result<serde_json::Value, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/write",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The WriteAssertions API will upsert new assertions for an authorization model id, or overwrite the existing ones. An assertion is an object that contains a tuple key, and the expectation of whether a call to the Check API of that tuple key will return true or false.
pub async fn write_assertions(
    configuration: &configuration::Configuration,
    store_id: &str,
    authorization_model_id: &str,
    body: crate::models::WriteAssertionsRequest,
) -> Result<(), Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/assertions/{authorization_model_id}",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id),
        authorization_model_id = crate::apis::urlencode(authorization_model_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The WriteAuthorizationModel API will add a new authorization model to a store. Each item in the `type_definitions` array is a type definition as specified in the field `type_definition`. The response will return the authorization model's ID in the `id` field.  ## Example To add an authorization model with `user` and `document` type definitions, call POST authorization-models API with the body:  ```json {   \"type_definitions\":[     {       \"type\":\"user\"     },     {       \"type\":\"document\",       \"relations\":{         \"reader\":{           \"union\":{             \"child\":[               {                 \"this\":{}               },               {                 \"computedUserset\":{                   \"object\":\"\",                   \"relation\":\"writer\"                 }               }             ]           }         },         \"writer\":{           \"this\":{}         }       }     }   ] } ``` OpenFGA's response will include the version id for this authorization model, which will look like  ``` {\"authorization_model_id\": \"01G50QVV17PECNVAHX1GG4Y5NC\"} ```
pub async fn write_authorization_model(
    configuration: &configuration::Configuration,
    store_id: &str,
    body: crate::models::WriteAuthorizationModelRequest,
) -> Result<crate::models::WriteAuthorizationModelResponse, Error<OpenFGAError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/stores/{store_id}/authorization-models",
        configuration.base_path,
        store_id = crate::apis::urlencode(store_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFGAError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
