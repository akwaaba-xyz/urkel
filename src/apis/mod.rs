use rocket::futures::{stream, StreamExt};
use std::env;

// gRPC
pub mod openfga {
    tonic::include_proto!("openfga.v1");
}
use open_fga_service_client::OpenFgaServiceClient;
use openfga::*;

use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel, Request,
    Status,
};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BatchCheckResponse {
    #[serde(rename = "allowed", skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    #[serde(rename = "_request", skip_serializing_if = "Option::is_none")]
    pub request: Option<CheckRequest>,
    #[serde(rename = "err", skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
}

impl BatchCheckResponse {
    pub fn new() -> BatchCheckResponse {
        BatchCheckResponse {
            allowed: None,
            request: None,
            err: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CheckNOfMRequest {
    #[serde(rename = "checks")]
    pub checks: Vec<CheckRequest>,
    #[serde(rename = "n")]
    pub num: usize,
}

impl CheckNOfMRequest {
    pub fn new(checks: Vec<CheckRequest>, num: usize) -> CheckNOfMRequest {
        CheckNOfMRequest {
            checks: checks,
            num: num,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CheckHorizontalRequest {
    #[serde(rename = "read_from")]
    pub read_from: Box<ObjectRelation>,
    #[serde(rename = "check_for")]
    pub check_for: Box<ObjectRelation>,
    #[serde(
        rename = "authorization_model_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_model_id: Option<String>,
}

impl CheckHorizontalRequest {
    pub fn new(read_from: ObjectRelation, check_for: ObjectRelation) -> CheckHorizontalRequest {
        CheckHorizontalRequest {
            read_from: Box::new(read_from),
            check_for: Box::new(check_for),
            authorization_model_id: None,
        }
    }
}

const CONCURRENT_REQUESTS: usize = 2;

pub async fn get_default_client() -> Result<
    OpenFgaServiceClient<
        InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
    >,
    Box<dyn std::error::Error>,
> {
    let token = env::var("OPENFGA_BEARER_TOKEN").map_err(|_| {
        "Pass a valid preshared token via `OPENFGA_BEARER_TOKEN` environment variable.".to_string()
    })?;
    let mut default_base_path = "grpc://[::1]:8081".to_owned();

    if let Ok(fga_addr) = env::var("OPENFGA_ADDR") {
        default_base_path = fga_addr.clone();
    }

    let channel = Channel::from_shared(default_base_path)?.connect().await?;

    let bearer_token = format!("Bearer {}", token);
    let header_value: MetadataValue<_> = bearer_token.parse()?;

    let service = OpenFgaServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut()
            .insert("authorization", header_value.clone());
        Ok(req)
    });
    Ok(service)
}

pub async fn get_store(
    store_id: &str,
) -> Result<tonic::Response<GetStoreResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(GetStoreRequest {
        store_id: store_id.into(),
    });

    let response = client.get_store(request).await?;
    Ok(response)
}

pub async fn list_stores(
    page_size: Option<i32>,
    continuation_token: Option<&str>,
) -> Result<tonic::Response<ListStoresResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(ListStoresRequest {
        page_size: page_size,
        continuation_token: continuation_token.unwrap_or("").into(),
    });

    let response = client.list_stores(request).await?;
    Ok(response)
}

pub async fn create_store(
    body: CreateStoreRequest,
) -> Result<tonic::Response<CreateStoreResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let response = client.create_store(body).await?;
    Ok(response)
}

pub async fn delete_store(store_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(DeleteStoreRequest {
        store_id: store_id.into(),
    });

    client.delete_store(request).await?;
    Ok(())
}

pub async fn read_authorization_models(
    store_id: &str,
    page_size: Option<i32>,
    continuation_token: Option<&str>,
) -> Result<tonic::Response<ReadAuthorizationModelsResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(ReadAuthorizationModelsRequest {
        store_id: Some(store_id.to_string()),
        page_size: page_size,
        continuation_token: continuation_token.unwrap_or("").into(),
    });

    let response = client.read_authorization_models(request).await?;
    Ok(response)
}

pub async fn write_authorization_model(
    store_id: &str,
    body: WriteAuthorizationModelRequest,
) -> Result<tonic::Response<WriteAuthorizationModelResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(WriteAuthorizationModelRequest {
        store_id: Some(store_id.to_string()),
        type_definitions: body.type_definitions,
        schema_version: body.schema_version,
    });

    let response = client.write_authorization_model(request).await?;
    Ok(response)
}

pub async fn read_authorization_model(
    store_id: &str,
    id: &str,
) -> Result<tonic::Response<ReadAuthorizationModelResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(ReadAuthorizationModelRequest {
        store_id: Some(store_id.to_string()),
        id: id.into(),
    });

    let response = client.read_authorization_model(request).await?;
    Ok(response)
}

pub async fn read_changes(
    store_id: &str,
    r#type: Option<&str>,
    page_size: Option<i32>,
    continuation_token: Option<&str>,
) -> Result<tonic::Response<ReadChangesResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(ReadChangesRequest {
        store_id: Some(store_id.to_string()),
        r#type: r#type.unwrap_or("").into(),
        page_size: page_size,
        continuation_token: continuation_token.unwrap_or("").into(),
    });

    let response = client.read_changes(request).await?;
    Ok(response)
}

pub async fn read(
    store_id: &str,
    body: ReadRequest,
) -> Result<tonic::Response<ReadResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(ReadRequest {
        store_id: Some(store_id.to_string()),
        tuple_key: body.tuple_key,
        page_size: body.page_size,
        continuation_token: body.continuation_token.into(),
    });

    let response = client.read(request).await?;
    Ok(response)
}

pub async fn write(
    store_id: &str,
    body: WriteRequest,
) -> Result<tonic::Response<WriteResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(WriteRequest {
        store_id: Some(store_id.to_string()),
        writes: body.writes,
        deletes: body.deletes,
        authorization_model_id: body.authorization_model_id,
    });

    let response = client.write(request).await?;
    Ok(response)
}

pub async fn check(
    store_id: &str,
    body: CheckRequest,
) -> Result<tonic::Response<CheckResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(CheckRequest {
        store_id: Some(store_id.to_string()),
        tuple_key: body.tuple_key,
        contextual_tuples: body.contextual_tuples,
        authorization_model_id: body.authorization_model_id,
        trace: body.trace,
    });

    let response = client.check(request).await?;
    Ok(response)
}

pub async fn expand(
    store_id: &str,
    body: ExpandRequest,
) -> Result<tonic::Response<ExpandResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(ExpandRequest {
        store_id: Some(store_id.to_string()),
        tuple_key: body.tuple_key,
        authorization_model_id: body.authorization_model_id,
    });

    let response = client.expand(request).await?;
    Ok(response)
}

pub async fn list_objects(
    store_id: &str,
    body: ListObjectsRequest,
) -> Result<tonic::Response<ListObjectsResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(ListObjectsRequest {
        store_id: Some(store_id.to_string()),
        r#type: body.r#type,
        relation: body.relation,
        user: body.user,
        contextual_tuples: body.contextual_tuples,
        authorization_model_id: body.authorization_model_id,
    });

    let response = client.list_objects(request).await?;
    Ok(response)
}

pub async fn read_assertions(
    store_id: &str,
    authorization_model_id: &str,
) -> Result<tonic::Response<ReadAssertionsResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(ReadAssertionsRequest {
        store_id: store_id.to_string(),
        authorization_model_id: authorization_model_id.into(),
    });

    let response = client.read_assertions(request).await?;
    Ok(response)
}

pub async fn write_assertions(
    store_id: &str,
    authorization_model_id: &str,
    body: WriteAssertionsRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let request = tonic::Request::new(WriteAssertionsRequest {
        store_id: Some(store_id.to_string()),
        authorization_model_id: authorization_model_id.into(),
        assertions: body.assertions,
    });

    client.write_assertions(request).await?;
    Ok(())
}

pub async fn read_until_end(
    store_id: &str,
    body: ReadRequest,
) -> Result<tonic::Response<ReadResponse>, Box<dyn std::error::Error>> {
    let mut client = get_default_client().await?;

    let mut local_var_continuation_token = Some(body.continuation_token);
    let mut tuples = Vec::new();

    while let Some(ref_continuation_token) = local_var_continuation_token {
        let loop_request = tonic::Request::new(ReadRequest {
            store_id: Some(store_id.to_string()),
            tuple_key: body.tuple_key.clone(),
            page_size: body.page_size.clone(),
            continuation_token: ref_continuation_token,
        });

        let loop_response = client.read(loop_request).await?.into_inner();

        tuples.extend(loop_response.tuples);

        if loop_response.continuation_token != "" {
            local_var_continuation_token = Some(loop_response.continuation_token);
        } else {
            local_var_continuation_token = None;
        }
    }
    let new_response = tonic::Response::new(ReadResponse {
        tuples: tuples,
        continuation_token: "".to_owned(),
    });
    Ok(new_response)
}

pub async fn batch_check(
    store_id: &str,
    bodies: Vec<CheckRequest>,
) -> Vec<Result<BatchCheckResponse, BatchCheckResponse>> {
    let local_var_futures = stream::iter(bodies)
        .map(|body| async move {
            let mut client = get_default_client().await.expect("building tonic client");

            let request = tonic::Request::new(CheckRequest {
                store_id: Some(store_id.to_string()),
                tuple_key: body.tuple_key.clone(),
                contextual_tuples: body.contextual_tuples.clone(),
                authorization_model_id: body.authorization_model_id.clone(),
                trace: body.trace.clone(),
            });

            let response = client.check(request).await;
            match response {
                Ok(check) => Ok(BatchCheckResponse {
                    allowed: Some(check.into_inner().allowed),
                    request: Some(body),
                    err: None,
                }),
                Err(error) => Ok(BatchCheckResponse {
                    allowed: Some(false),
                    request: Some(body),
                    err: Some(error.to_string()),
                }),
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    let results = local_var_futures
        .collect::<Vec<Result<BatchCheckResponse, BatchCheckResponse>>>()
        .await;
    results
}

pub async fn check_n_of_m(
    store_id: &str,
    body: CheckNOfMRequest,
) -> Result<tonic::Response<CheckResponse>, Box<dyn std::error::Error>> {
    let n = body.num;
    let checks = body.checks;
    if checks.len() < 1 {
        let validation_error = crate::models::ValidationErrorMessageResponse {
            code: Some(crate::models::ErrorCode::ValidationError),
            message: Some("Must provide at least one check.".into()),
        };
        return Err(Box::new(validation_error));
    }
    if n <= 0 || n > (checks.len() - 1) {
        let validation_error = crate::models::ValidationErrorMessageResponse {
            code: Some(crate::models::ErrorCode::ValidationError),
            message: Some("Invalid n value provided.".into()),
        };
        return Err(Box::new(validation_error));
    }
    let results: Vec<Result<BatchCheckResponse, BatchCheckResponse>> =
        batch_check(store_id, checks).await;
    let results = results
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(|result| result.allowed.unwrap_or(false))
        .collect::<Vec<_>>();
    if results.len() >= n {
        Ok(tonic::Response::new(CheckResponse {
            allowed: true,
            resolution: None,
        }))
    } else {
        Ok(tonic::Response::new(CheckResponse {
            allowed: false,
            resolution: None,
        }))
    }
}

pub async fn check_horizontal(
    store_id: &str,
    body: CheckHorizontalRequest,
) -> Result<tonic::Response<CheckResponse>, Box<dyn std::error::Error>> {
    let read_from = ReadRequest {
        store_id: Some(store_id.to_string()),
        tuple_key: Some(TupleKey {
            object: Some(body.read_from.object.clone()),
            relation: Some(body.read_from.relation.clone()),
            user: None,
        }),
        page_size: Some(100),
        continuation_token: "".to_owned(),
    };

    let local_var_response: ReadResponse = read_until_end(store_id, read_from).await?.into_inner();
    let mut check_requests = Vec::new();
    let tuples = local_var_response.tuples;

    if tuples.len() == 0 {
        let err_message =
            "Object relation from 'read_from' returned no results to compare to.".to_string();
        let validation_error = crate::models::ValidationErrorMessageResponse {
            code: Some(crate::models::ErrorCode::ValidationError),
            message: Some(err_message.clone()),
        };
        return Err(Box::new(validation_error));
    }

    tuples
        .into_iter()
        .map(|tuple| {
            let mut check_request = CheckRequest {
                store_id: Some(store_id.to_string()),
                tuple_key: Some(TupleKey {
                    object: Some(body.check_for.object.clone()),
                    relation: Some(body.check_for.relation.clone()),
                    user: None,
                }),
                authorization_model_id: body.authorization_model_id.clone(),
                contextual_tuples: None,
                trace: None,
            };
            if let Some(tuple_key) = tuple.key {
                if let Some(ref mut request_tuple) = &mut check_request.tuple_key {
                    request_tuple.user = tuple_key.user.clone();
                }
            }
            check_requests.push(check_request);
        })
        .for_each(drop);

    // Check one to fail eagerly since checks are homogenous
    check(store_id, check_requests[0].clone()).await?;

    let local_var_response: Vec<Result<BatchCheckResponse, BatchCheckResponse>> =
        batch_check(store_id, check_requests).await;
    let batch_result_len: usize = local_var_response.len();
    let local_var_allows = local_var_response
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(|result| result.allowed.unwrap_or(false))
        .collect::<Vec<_>>();

    if local_var_allows.len() == batch_result_len {
        Ok(tonic::Response::new(CheckResponse {
            allowed: true,
            resolution: None,
        }))
    } else {
        Ok(tonic::Response::new(CheckResponse {
            allowed: false,
            resolution: None,
        }))
    }
}
