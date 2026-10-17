// GraphQL support for Armature framework

pub mod config;
pub mod decorators;
pub mod federation;
pub mod resolver;
pub mod schema_builder;
pub mod schema_docs;
#[cfg(feature = "sdl-export")]
pub mod sdl_static;
#[cfg(feature = "websocket")]
pub mod websocket;

pub use armature_graphql_macros::resolver;
pub use async_graphql;
pub use async_graphql::{
    Context, EmptyMutation, EmptySubscription, Enum, Error, ID, InputObject, MergedObject,
    MergedSubscription, Object, Result, Schema, SimpleObject, Subscription, Union,
};

pub use config::*;
pub use decorators::*;
pub use resolver::*;
pub use schema_builder::*;
pub use schema_docs::*;
#[cfg(feature = "websocket")]
pub use websocket::{ALL_WEBSOCKET_PROTOCOLS, Protocols, select_protocol, serve_graphql_websocket};

use armature_core::Error as ArmatureError;
use armature_log::info;
use std::sync::Arc;

/// GraphQL schema wrapper
pub struct GraphQLSchema<Query, Mutation, Subscription> {
    schema: Arc<Schema<Query, Mutation, Subscription>>,
}

impl<Query, Mutation, Subscription> GraphQLSchema<Query, Mutation, Subscription>
where
    Query: async_graphql::ObjectType + 'static,
    Mutation: async_graphql::ObjectType + 'static,
    Subscription: async_graphql::SubscriptionType + 'static,
{
    pub fn new(schema: Schema<Query, Mutation, Subscription>) -> Self {
        info!("Initializing GraphQL schema");
        Self {
            schema: Arc::new(schema),
        }
    }

    pub fn schema(&self) -> Arc<Schema<Query, Mutation, Subscription>> {
        self.schema.clone()
    }
}

impl<Query, Mutation, Subscription> Clone for GraphQLSchema<Query, Mutation, Subscription> {
    fn clone(&self) -> Self {
        Self {
            schema: self.schema.clone(),
        }
    }
}

/// GraphQL request handling
pub struct GraphQLRequest {
    pub query: String,
    pub variables: Option<serde_json::Value>,
    pub operation_name: Option<String>,
}

impl GraphQLRequest {
    pub fn new(query: String) -> Self {
        Self {
            query,
            variables: None,
            operation_name: None,
        }
    }

    pub fn with_variables(mut self, variables: serde_json::Value) -> Self {
        self.variables = Some(variables);
        self
    }

    pub fn with_operation(mut self, operation_name: String) -> Self {
        self.operation_name = Some(operation_name);
        self
    }
}

/// GraphQL response
pub struct GraphQLResponse {
    pub data: Option<serde_json::Value>,
    pub errors: Vec<String>,
}

impl GraphQLResponse {
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            data: Some(data),
            errors: Vec::new(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            data: None,
            errors: vec![message],
        }
    }

    pub fn to_json(&self) -> Result<String, ArmatureError> {
        serde_json::to_string(self).map_err(|e| ArmatureError::Serialization(e.to_string()))
    }
}

impl serde::Serialize for GraphQLResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;

        if let Some(ref data) = self.data {
            map.serialize_entry("data", data)?;
        } else {
            map.serialize_entry("data", &None::<()>)?;
        }

        if !self.errors.is_empty() {
            map.serialize_entry("errors", &self.errors)?;
        }

        map.end()
    }
}

/// Helper to build GraphQL schemas with DI integration
pub struct SchemaBuilder<Query, Mutation, Subscription> {
    query: Option<Query>,
    mutation: Option<Mutation>,
    subscription: Option<Subscription>,
    config: Option<GraphQLConfig>,
}

impl<Query, Mutation, Subscription> SchemaBuilder<Query, Mutation, Subscription>
where
    Query: async_graphql::ObjectType + 'static,
    Mutation: async_graphql::ObjectType + 'static,
    Subscription: async_graphql::SubscriptionType + 'static,
{
    pub fn new() -> Self {
        Self {
            query: None,
            mutation: None,
            subscription: None,
            config: None,
        }
    }

    pub fn query(mut self, query: Query) -> Self {
        self.query = Some(query);
        self
    }

    pub fn mutation(mut self, mutation: Mutation) -> Self {
        self.mutation = Some(mutation);
        self
    }

    pub fn subscription(mut self, subscription: Subscription) -> Self {
        self.subscription = Some(subscription);
        self
    }

    /// Attach a [`GraphQLConfig`] whose security/behavior knobs (introspection,
    /// max depth, max complexity, validation, tracing) will be applied to the
    /// built schema. Without this, [`GraphQLConfig::default`] (production-safe:
    /// introspection off, depth/complexity/directive limits on) is applied.
    pub fn config(mut self, config: GraphQLConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<Schema<Query, Mutation, Subscription>, ArmatureError> {
        let query = self
            .query
            .ok_or_else(|| ArmatureError::Internal("Query root is required".to_string()))?;
        let mutation = self
            .mutation
            .ok_or_else(|| ArmatureError::Internal("Mutation root is required".to_string()))?;
        let subscription = self
            .subscription
            .ok_or_else(|| ArmatureError::Internal("Subscription root is required".to_string()))?;

        let builder = Schema::build(query, mutation, subscription);
        let builder = self.config.unwrap_or_default().configure(builder);

        Ok(builder.finish())
    }
}

impl<Query, Mutation, Subscription> Default for SchemaBuilder<Query, Mutation, Subscription>
where
    Query: async_graphql::ObjectType + 'static,
    Mutation: async_graphql::ObjectType + 'static,
    Subscription: async_graphql::SubscriptionType + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Pinned `graphql-playground-react` release served by [`graphql_playground_html`].
pub const GRAPHQL_PLAYGROUND_VERSION: &str = "1.7.28";

/// Pinned `graphiql` release served by [`graphiql_html`].
pub const GRAPHIQL_VERSION: &str = "3.9.0";

/// Pinned `react`/`react-dom` release served by [`graphiql_html`].
pub const GRAPHIQL_REACT_VERSION: &str = "18.3.1";

// Subresource Integrity hashes for the exact pinned CDN files below. They were
// computed with `curl -sfL <url> | openssl dgst -sha384 -binary | base64`;
// recompute them whenever a version constant changes.
const PLAYGROUND_CSS_URL: &str =
    "https://cdn.jsdelivr.net/npm/graphql-playground-react@1.7.28/build/static/css/index.css";
const PLAYGROUND_CSS_SRI: &str =
    "sha384-xb+UHILNN4fV3NgQMTjXk0x9A80U0hmkraTFvucUYTILJymGT8E1Aq2278NSi5+3";
const PLAYGROUND_JS_URL: &str =
    "https://cdn.jsdelivr.net/npm/graphql-playground-react@1.7.28/build/static/js/middleware.js";
const PLAYGROUND_JS_SRI: &str =
    "sha384-ardaO17esJ2ZxvY24V1OE6X4j+Z3WKgGMptrlDLmD+2w/JC3nbQ5ZfKGY2zfOPEE";
const REACT_JS_URL: &str = "https://unpkg.com/react@18.3.1/umd/react.production.min.js";
const REACT_JS_SRI: &str =
    "sha384-DGyLxAyjq0f9SPpVevD6IgztCFlnMF6oW/XQGmfe+IsZ8TqEiDrcHkMLKI6fiB/Z";
const REACT_DOM_JS_URL: &str = "https://unpkg.com/react-dom@18.3.1/umd/react-dom.production.min.js";
const REACT_DOM_JS_SRI: &str =
    "sha384-gTGxhz21lVGYNMcdJOyq01Edg0jhn/c22nsx0kyqP0TxaV5WVdsSH1fSDUf5YJj1";
const GRAPHIQL_CSS_URL: &str = "https://unpkg.com/graphiql@3.9.0/graphiql.min.css";
const GRAPHIQL_CSS_SRI: &str =
    "sha384-QMux00XgRtwRLSYIY3kw2rj1ovk5AuuliAchk+HSQbqdbGFnz9GYuqIlOqxhwCE2";
const GRAPHIQL_JS_URL: &str = "https://unpkg.com/graphiql@3.9.0/graphiql.min.js";
const GRAPHIQL_JS_SRI: &str =
    "sha384-8NGfVj4CVlqHajlZj+bPJT4thxPMHMJYn7DWK2CvtopLd02E7qsPHazniBYvVjOO";

/// Encode `endpoint` as a JavaScript string literal that is also safe inside
/// an HTML `<script>` element (`<`, `>`, `&` and U+2028/2029 are escaped, so
/// `</script>` cannot terminate the block).
fn js_string_literal(endpoint: &str) -> String {
    let json = serde_json::to_string(endpoint).unwrap_or_else(|_| "\"\"".to_string());
    json.replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

/// GraphQL playground HTML
///
/// Loads the pinned `graphql-playground-react` release
/// ([`GRAPHQL_PLAYGROUND_VERSION`]) from jsDelivr with Subresource Integrity,
/// so a compromised or changed CDN file is refused by the browser. Only serve
/// this in development (see [`GraphQLConfig::development`]).
///
/// # Example
///
/// ```
/// use armature_graphql::graphql_playground_html;
///
/// let html = graphql_playground_html("/graphql");
/// assert!(html.contains("GraphQL Playground"));
/// assert!(html.contains("integrity=\"sha384-"));
/// ```
pub fn graphql_playground_html(endpoint: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>GraphQL Playground</title>
    <link rel="stylesheet" href="{css_url}" integrity="{css_sri}" crossorigin="anonymous" />
    <script src="{js_url}" integrity="{js_sri}" crossorigin="anonymous"></script>
</head>
<body>
    <div id="root"></div>
    <script>
        window.addEventListener('load', function (event) {{
            GraphQLPlayground.init(document.getElementById('root'), {{
                endpoint: {endpoint}
            }})
        }})
    </script>
</body>
</html>
"#,
        css_url = PLAYGROUND_CSS_URL,
        css_sri = PLAYGROUND_CSS_SRI,
        js_url = PLAYGROUND_JS_URL,
        js_sri = PLAYGROUND_JS_SRI,
        endpoint = js_string_literal(endpoint),
    )
}

/// GraphiQL HTML (lighter alternative)
///
/// Loads pinned `graphiql` ([`GRAPHIQL_VERSION`]) and React
/// ([`GRAPHIQL_REACT_VERSION`]) builds from unpkg with Subresource Integrity.
/// Only serve this in development (see [`GraphQLConfig::development`]).
///
/// # Example
///
/// ```
/// use armature_graphql::graphiql_html;
///
/// let html = graphiql_html("/graphql");
/// assert!(html.contains("graphiql@3.9.0"));
/// assert!(html.contains("integrity=\"sha384-"));
/// ```
pub fn graphiql_html(endpoint: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <title>GraphiQL</title>
    <style>
        body {{
            height: 100vh;
            margin: 0;
            width: 100%;
            overflow: hidden;
        }}
        #graphiql {{
            height: 100vh;
        }}
    </style>
    <script src="{react_url}" integrity="{react_sri}" crossorigin="anonymous"></script>
    <script src="{react_dom_url}" integrity="{react_dom_sri}" crossorigin="anonymous"></script>
    <link rel="stylesheet" href="{css_url}" integrity="{css_sri}" crossorigin="anonymous" />
</head>
<body>
    <div id="graphiql">Loading...</div>
    <script src="{js_url}" integrity="{js_sri}" crossorigin="anonymous" type="application/javascript"></script>
    <script>
        const fetcher = GraphiQL.createFetcher({{
            url: {endpoint},
        }});

        ReactDOM.createRoot(document.getElementById('graphiql')).render(
            React.createElement(GraphiQL, {{ fetcher: fetcher }}),
        );
    </script>
</body>
</html>
"#,
        react_url = REACT_JS_URL,
        react_sri = REACT_JS_SRI,
        react_dom_url = REACT_DOM_JS_URL,
        react_dom_sri = REACT_DOM_JS_SRI,
        css_url = GRAPHIQL_CSS_URL,
        css_sri = GRAPHIQL_CSS_SRI,
        js_url = GRAPHIQL_JS_URL,
        js_sri = GRAPHIQL_JS_SRI,
        endpoint = js_string_literal(endpoint),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphql_request_builder() {
        let req = GraphQLRequest::new("query { hello }".to_string())
            .with_variables(serde_json::json!({"name": "World"}))
            .with_operation("HelloQuery".to_string());

        assert_eq!(req.query, "query { hello }");
        assert!(req.variables.is_some());
        assert_eq!(req.operation_name, Some("HelloQuery".to_string()));
    }

    #[test]
    fn test_graphql_response_serialization() {
        let response = GraphQLResponse::success(serde_json::json!({
            "hello": "world"
        }));

        let json = response.to_json().unwrap();
        assert!(json.contains("hello"));
        assert!(json.contains("world"));
    }

    #[test]
    fn test_graphql_response_error_serialization() {
        let response = GraphQLResponse::error("something went wrong".to_string());

        let json = response.to_json().unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(value["data"].is_null());
        assert!(json.contains("\"data\":null"));
        assert!(value["errors"].is_array());
        assert_eq!(value["errors"][0], "something went wrong");
    }

    fn assert_pinned_with_sri(html: &str) {
        let mut checked = 0;
        for tag in html
            .split('<')
            .filter(|t| t.starts_with("script src") || t.starts_with("link rel=\"stylesheet\""))
        {
            checked += 1;
            assert!(tag.contains("integrity=\"sha384-"), "missing SRI: <{tag}");
            assert!(
                tag.contains("crossorigin=\"anonymous\""),
                "missing crossorigin: <{tag}"
            );
            let url = tag.split('"').nth(1).unwrap_or_default();
            let url = if tag.starts_with("link") {
                tag.split("href=\"")
                    .nth(1)
                    .unwrap()
                    .split('"')
                    .next()
                    .unwrap()
            } else {
                url
            };
            assert!(
                url.contains('@'),
                "CDN URL must pin an exact version: {url}"
            );
        }
        assert!(checked >= 2, "expected CDN assets in page");
    }

    #[test]
    fn test_playground_assets_pinned_with_sri() {
        let html = graphql_playground_html("/graphql");
        assert_pinned_with_sri(&html);
        assert!(html.contains(GRAPHQL_PLAYGROUND_VERSION));
    }

    #[test]
    fn test_graphiql_assets_pinned_with_sri() {
        let html = graphiql_html("/graphql");
        assert_pinned_with_sri(&html);
        assert!(html.contains(GRAPHIQL_VERSION));
        assert!(html.contains(GRAPHIQL_REACT_VERSION));
    }

    #[test]
    fn test_ide_pages_escape_endpoint() {
        let payload = "/g'</script><script>alert(1)</script>";
        for html in [graphql_playground_html(payload), graphiql_html(payload)] {
            assert!(
                !html.contains("<script>alert(1)"),
                "endpoint not escaped: {html}"
            );
            assert!(html.contains("\\u003c/script\\u003e"));
        }
    }

    struct DefaultLimitsQuery;

    #[async_graphql::Object]
    impl DefaultLimitsQuery {
        async fn value(&self) -> i32 {
            1
        }
    }

    #[test]
    fn test_schema_builder_without_config_applies_safe_defaults() {
        let schema = SchemaBuilder::<
            DefaultLimitsQuery,
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        >::new()
        .query(DefaultLimitsQuery)
        .mutation(async_graphql::EmptyMutation)
        .subscription(async_graphql::EmptySubscription)
        .build()
        .unwrap();

        let response = tokio_test::block_on(schema.execute("{ __schema { types { name } } }"));
        assert!(response.data.into_json().unwrap()["__schema"].is_null());

        let mut query = String::from("{");
        for i in 0..=DEFAULT_MAX_COMPLEXITY {
            query.push_str(&format!(" a{i}: value"));
        }
        query.push('}');
        let response = tokio_test::block_on(schema.execute(query));
        assert!(
            !response.errors.is_empty(),
            "complexity cap must apply without config"
        );
    }
}
