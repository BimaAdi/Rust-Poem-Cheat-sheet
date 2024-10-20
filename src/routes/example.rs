use poem_openapi::{
    param::{Path, Query},
    payload::{Json, PlainText},
    OpenApi, Tags,
};

use crate::schema::example::{
    BadRequestResponse, ExampleMultipleResponse, ExamplePathQueryResponse,
    InternalServerErrorResponse, OkResponse, UnprocesableEntityResponse,
};

#[derive(Tags)]
enum ApiExampleTags {
    /// Example various poem implementation
    Example,
}

pub struct ApiExample;

#[OpenApi]
impl ApiExample {
    #[oai(
        path = "/example/hello",
        method = "get",
        tag = "ApiExampleTags::Example"
    )]
    async fn hello(&self) -> PlainText<String> {
        PlainText("hello".to_string())
    }

    #[oai(
        path = "/example/path-query/:path",
        method = "get",
        tag = "ApiExampleTags::Example"
    )]
    async fn path_query(
        &self,
        path: Path<String>,
        query_1: Query<Option<String>>,
        query_2: Query<Option<i64>>,
    ) -> Json<ExamplePathQueryResponse> {
        Json(ExamplePathQueryResponse {
            path: path.0,
            query_1: query_1.0,
            query_2: query_2.0,
        })
    }

    #[oai(
        path = "/example/multiple-response",
        method = "get",
        tag = "ApiExampleTags::Example"
    )]
    async fn multiple_response(&self, status: Query<i32>) -> ExampleMultipleResponse {
        match status.0 {
            200 => ExampleMultipleResponse::Ok(Json(OkResponse {
                data: "some data".to_string(),
            })),
            400 => ExampleMultipleResponse::BadRequest(Json(BadRequestResponse {
                validation_error: "some validataion error".to_string(),
            })),
            500 => {
                ExampleMultipleResponse::InternalServerError(Json(InternalServerErrorResponse {
                    error: "some error".to_string(),
                }))
            }
            _ => ExampleMultipleResponse::Unprocessable(Json(UnprocesableEntityResponse {
                validation_error: format!("invalid status = {}", status.0),
            })),
        }
    }
}
