use async_graphql::http::MultipartOptions;
use async_graphql::{BatchResponse as GraphQLBatchResponse, ObjectType, Schema, SubscriptionType};
use poem::http::{header, Method};
use poem::web::Json;
use poem::web::Query;
use poem::{async_trait, Endpoint, Error, FromRequest, Request, RequestBody, Result};
use tokio_util::compat::TokioAsyncReadCompatExt;

pub struct GraphQLRequest(pub async_graphql::Request);

#[async_trait]
impl<'a> FromRequest<'a> for GraphQLRequest {
    type Error = Error;

    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self> {
        Ok(GraphQLRequest(
            GraphQLBatchRequest::from_request(req, body)
                .await?
                .0
                .into_single()
                .map_err(Error::bad_request)?,
        ))
    }
}

/// An extractor for GraphQL batch request.
pub struct GraphQLBatchRequest(pub async_graphql::BatchRequest);

#[async_trait]
impl<'a> FromRequest<'a> for GraphQLBatchRequest {
    type Error = Error;

    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self> {
        if req.method() == Method::GET {
            let req = Query::from_request(req, body)
                .await
                .map_err(Error::bad_request)?
                .0;
            Ok(Self(async_graphql::BatchRequest::Single(req)))
        } else {
            let content_type = req
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .map(ToString::to_string);
            Ok(Self(
                async_graphql::http::receive_batch_body(
                    content_type,
                    body.take()?.into_async_read().compat(),
                    MultipartOptions::default(),
                )
                .await
                .map_err(Error::bad_request)?,
            ))
        }
    }
}

pub struct GraphQL<Query, Mutation, Subscription> {
    schema: Schema<Query, Mutation, Subscription>,
}

impl<Query, Mutation, Subscription> GraphQL<Query, Mutation, Subscription> {
    /// Create a GraphQL query endpoint.
    pub fn new(schema: Schema<Query, Mutation, Subscription>) -> Self {
        Self { schema }
    }
}

#[async_trait]
impl<Query, Mutation, Subscription> Endpoint for GraphQL<Query, Mutation, Subscription>
where
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
{
    type Output = Result<Json<GraphQLBatchResponse>>;

    async fn call(&self, req: Request) -> Self::Output {
        let (req, mut body) = req.split();
        let req = GraphQLBatchRequest::from_request(&req, &mut body).await?;
        Ok(Json(self.schema.execute_batch(req.0).await))
    }
}
