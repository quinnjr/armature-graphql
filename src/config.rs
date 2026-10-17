/// GraphQL server configuration
#[derive(Debug, Clone)]
pub struct GraphQLConfig {
    /// GraphQL endpoint path
    pub endpoint: String,

    /// Enable GraphQL Playground (interactive GraphQL IDE)
    pub enable_playground: bool,

    /// Playground endpoint path (if enabled)
    pub playground_endpoint: String,

    /// Enable GraphiQL (lighter alternative to Playground)
    pub enable_graphiql: bool,

    /// GraphiQL endpoint path (if enabled)
    pub graphiql_endpoint: String,

    /// Enable schema documentation endpoint
    pub enable_schema_docs: bool,

    /// Schema documentation endpoint path
    pub schema_docs_endpoint: String,

    /// Enable introspection queries (required for playgrounds and docs)
    pub enable_introspection: bool,

    /// Maximum query depth (0 = unlimited)
    pub max_depth: usize,

    /// Maximum query complexity (0 = unlimited)
    ///
    /// Every selected field — including each aliased copy of a field —
    /// costs at least 1, so this limit also bounds the total number of
    /// fields and aliases a single operation can request. async-graphql has
    /// no separate alias-count limit; this is the knob that covers alias
    /// amplification (`a1: expensive a2: expensive ...`).
    pub max_complexity: usize,

    /// Maximum number of directives on a single field (0 = unlimited)
    pub max_directives: usize,

    /// Maximum recursion depth of the query parser/validator. Guards against
    /// stack exhaustion from deeply nested input values or fragments.
    /// async-graphql's own default is 32.
    pub max_recursive_depth: usize,

    /// Enable query validation
    pub enable_validation: bool,

    /// Enable Apollo Tracing
    pub enable_tracing: bool,
}

/// Default maximum query depth applied by [`GraphQLConfig::new`].
pub const DEFAULT_MAX_DEPTH: usize = 15;

/// Default maximum query complexity applied by [`GraphQLConfig::new`].
pub const DEFAULT_MAX_COMPLEXITY: usize = 1000;

/// Default maximum directives per field applied by [`GraphQLConfig::new`].
pub const DEFAULT_MAX_DIRECTIVES: usize = 10;

/// Default parser/validator recursion limit applied by [`GraphQLConfig::new`].
pub const DEFAULT_MAX_RECURSIVE_DEPTH: usize = 32;

impl GraphQLConfig {
    /// Create a new GraphQL configuration with production-safe defaults.
    ///
    /// The defaults are locked down: playground, GraphiQL, the schema docs
    /// endpoint and introspection are all **off**, and query depth,
    /// complexity, directive count and parser recursion are capped
    /// ([`DEFAULT_MAX_DEPTH`], [`DEFAULT_MAX_COMPLEXITY`],
    /// [`DEFAULT_MAX_DIRECTIVES`], [`DEFAULT_MAX_RECURSIVE_DEPTH`]).
    /// Opt in to the developer tooling with [`GraphQLConfig::development`]
    /// or the individual `with_*` builders.
    ///
    /// # Example
    ///
    /// ```
    /// use armature_graphql::GraphQLConfig;
    ///
    /// let config = GraphQLConfig::new("/graphql");
    /// assert_eq!(config.endpoint, "/graphql");
    /// assert!(!config.enable_playground);
    /// assert!(!config.enable_introspection);
    /// assert_eq!(config.max_depth, 15);
    /// assert_eq!(config.max_complexity, 1000);
    /// ```
    pub fn new(endpoint: impl Into<String>) -> Self {
        let endpoint = endpoint.into();
        Self {
            playground_endpoint: format!("{}/playground", endpoint),
            graphiql_endpoint: format!("{}/graphiql", endpoint),
            schema_docs_endpoint: format!("{}/schema", endpoint),
            endpoint,
            enable_playground: false,
            enable_graphiql: false,
            enable_schema_docs: false,
            enable_introspection: false,
            max_depth: DEFAULT_MAX_DEPTH,
            max_complexity: DEFAULT_MAX_COMPLEXITY,
            max_directives: DEFAULT_MAX_DIRECTIVES,
            max_recursive_depth: DEFAULT_MAX_RECURSIVE_DEPTH,
            enable_validation: true,
            enable_tracing: false,
        }
    }

    /// Create a production configuration (playgrounds disabled).
    ///
    /// Identical to [`GraphQLConfig::new`], which is already production-safe;
    /// kept as an explicit, self-documenting constructor.
    ///
    /// # Example
    ///
    /// ```
    /// use armature_graphql::GraphQLConfig;
    ///
    /// let config = GraphQLConfig::production("/graphql");
    /// assert!(!config.enable_playground);
    /// assert!(!config.enable_graphiql);
    /// assert!(!config.enable_introspection); // Disabled for security
    /// assert_eq!(config.max_depth, 15); // DoS protection enabled by default
    /// assert_eq!(config.max_complexity, 1000); // DoS protection enabled by default
    /// ```
    pub fn production(endpoint: impl Into<String>) -> Self {
        Self::new(endpoint)
    }

    /// Create a development configuration (all features enabled).
    ///
    /// Turns on playground, GraphiQL, schema docs, introspection and
    /// tracing. The depth/complexity limits from [`GraphQLConfig::new`] stay
    /// in place. Never use this for an internet-facing deployment.
    ///
    /// # Example
    ///
    /// ```
    /// use armature_graphql::GraphQLConfig;
    ///
    /// let config = GraphQLConfig::development("/graphql");
    /// assert!(config.enable_playground);
    /// assert!(config.enable_graphiql);
    /// assert!(config.enable_schema_docs);
    /// assert!(config.enable_introspection);
    /// ```
    pub fn development(endpoint: impl Into<String>) -> Self {
        let mut config = Self::new(endpoint);
        config.enable_playground = true;
        config.enable_graphiql = true;
        config.enable_schema_docs = true;
        config.enable_introspection = true;
        config.enable_tracing = true;
        config
    }

    /// Enable or disable GraphQL Playground
    pub fn with_playground(mut self, enable: bool) -> Self {
        self.enable_playground = enable;
        self
    }

    /// Set custom playground endpoint
    pub fn with_playground_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.playground_endpoint = endpoint.into();
        self
    }

    /// Enable or disable GraphiQL
    pub fn with_graphiql(mut self, enable: bool) -> Self {
        self.enable_graphiql = enable;
        self
    }

    /// Set custom GraphiQL endpoint
    pub fn with_graphiql_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.graphiql_endpoint = endpoint.into();
        self
    }

    /// Enable or disable schema documentation endpoint
    pub fn with_schema_docs(mut self, enable: bool) -> Self {
        self.enable_schema_docs = enable;
        self
    }

    /// Set custom schema documentation endpoint
    pub fn with_schema_docs_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.schema_docs_endpoint = endpoint.into();
        self
    }

    /// Enable or disable introspection queries
    pub fn with_introspection(mut self, enable: bool) -> Self {
        self.enable_introspection = enable;
        self
    }

    /// Set maximum query depth (0 disables the limit — not recommended)
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Set maximum query complexity (0 disables the limit — not recommended)
    pub fn with_max_complexity(mut self, complexity: usize) -> Self {
        self.max_complexity = complexity;
        self
    }

    /// Set the maximum number of directives on a single field (0 disables
    /// the limit — not recommended)
    pub fn with_max_directives(mut self, max_directives: usize) -> Self {
        self.max_directives = max_directives;
        self
    }

    /// Set the parser/validator recursion limit (must be non-zero; a value
    /// of 0 falls back to [`DEFAULT_MAX_RECURSIVE_DEPTH`])
    pub fn with_max_recursive_depth(mut self, depth: usize) -> Self {
        self.max_recursive_depth = depth;
        self
    }

    /// Enable or disable query validation
    pub fn with_validation(mut self, enable: bool) -> Self {
        self.enable_validation = enable;
        self
    }

    /// Enable or disable Apollo Tracing
    pub fn with_tracing(mut self, enable: bool) -> Self {
        self.enable_tracing = enable;
        self
    }

    /// Apply this configuration's security/behavior knobs onto an
    /// `async-graphql` [`SchemaBuilder`](async_graphql::SchemaBuilder).
    ///
    /// This is the only place these knobs take effect — building a schema
    /// via a bare `Schema::build(...).finish()` silently ignores every
    /// field on [`GraphQLConfig`]. In particular, [`GraphQLConfig::production`]
    /// documents introspection as "Disabled for security", but that only
    /// becomes true once the config is threaded through this method.
    ///
    /// Applies, in order:
    /// - `.limit_depth(max_depth)` when `max_depth != 0`
    /// - `.limit_complexity(max_complexity)` when `max_complexity != 0`
    /// - `.limit_directives(max_directives)` when `max_directives != 0`
    /// - `.limit_recursive_depth(max_recursive_depth)` (0 falls back to
    ///   [`DEFAULT_MAX_RECURSIVE_DEPTH`])
    /// - `.disable_introspection()` when `!enable_introspection`
    /// - `.validation_mode(ValidationMode::Fast)` and `.disable_suggestions()`
    ///   when `!enable_validation` (async-graphql has no way to fully turn
    ///   validation off; this is the closest approximation it exposes —
    ///   faster/looser validation and no field-name suggestions, which also
    ///   avoids leaking schema shape hints in error messages)
    /// - `.extension(ApolloTracing)` when `enable_tracing`
    ///
    /// # Example
    ///
    /// ```
    /// use armature_graphql::GraphQLConfig;
    /// use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
    ///
    /// struct Query;
    ///
    /// #[Object]
    /// impl Query {
    ///     async fn hello(&self) -> &str {
    ///         "hi"
    ///     }
    /// }
    ///
    /// let config = GraphQLConfig::production("/graphql");
    /// let schema = config
    ///     .configure(Schema::build(Query, EmptyMutation, EmptySubscription))
    ///     .finish();
    /// ```
    pub fn configure<Q, M, S>(
        &self,
        mut builder: async_graphql::SchemaBuilder<Q, M, S>,
    ) -> async_graphql::SchemaBuilder<Q, M, S> {
        if self.max_depth > 0 {
            builder = builder.limit_depth(self.max_depth);
        }

        if self.max_complexity > 0 {
            builder = builder.limit_complexity(self.max_complexity);
        }

        if self.max_directives > 0 {
            builder = builder.limit_directives(self.max_directives);
        }

        let recursive_depth = if self.max_recursive_depth > 0 {
            self.max_recursive_depth
        } else {
            DEFAULT_MAX_RECURSIVE_DEPTH
        };
        builder = builder.limit_recursive_depth(recursive_depth);

        if !self.enable_introspection {
            builder = builder.disable_introspection();
        }

        if !self.enable_validation {
            builder = builder
                .validation_mode(async_graphql::ValidationMode::Fast)
                .disable_suggestions();
        }

        if self.enable_tracing {
            builder = builder.extension(async_graphql::extensions::ApolloTracing);
        }

        builder
    }
}

/// Equivalent to `GraphQLConfig::new("/graphql")` — production-safe defaults.
impl Default for GraphQLConfig {
    fn default() -> Self {
        Self::new("/graphql")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        // M5: the default constructor must be production-safe.
        let config = GraphQLConfig::new("/graphql");
        assert_eq!(config.endpoint, "/graphql");
        assert!(!config.enable_playground);
        assert!(!config.enable_graphiql);
        assert!(!config.enable_introspection);
        assert!(!config.enable_schema_docs);
        assert_eq!(config.max_depth, DEFAULT_MAX_DEPTH);
        assert_eq!(config.max_complexity, DEFAULT_MAX_COMPLEXITY);
        assert_eq!(config.max_directives, DEFAULT_MAX_DIRECTIVES);
        assert_eq!(config.max_recursive_depth, DEFAULT_MAX_RECURSIVE_DEPTH);
    }

    #[test]
    fn test_new_matches_production_and_default() {
        let new = GraphQLConfig::new("/graphql");
        let prod = GraphQLConfig::production("/graphql");
        let default = GraphQLConfig::default();
        for other in [&prod, &default] {
            assert_eq!(new.enable_playground, other.enable_playground);
            assert_eq!(new.enable_graphiql, other.enable_graphiql);
            assert_eq!(new.enable_schema_docs, other.enable_schema_docs);
            assert_eq!(new.enable_introspection, other.enable_introspection);
            assert_eq!(new.max_depth, other.max_depth);
            assert_eq!(new.max_complexity, other.max_complexity);
            assert_eq!(new.max_directives, other.max_directives);
            assert_eq!(new.max_recursive_depth, other.max_recursive_depth);
        }
    }

    #[test]
    fn test_production_config() {
        let config = GraphQLConfig::production("/api/graphql");
        assert_eq!(config.endpoint, "/api/graphql");
        assert!(!config.enable_playground);
        assert!(!config.enable_graphiql);
        assert!(!config.enable_introspection);
        assert!(!config.enable_schema_docs);
    }

    #[test]
    fn test_production_config_enforces_dos_protection() {
        // `production()` disables introspection but must also set non-zero
        // max_depth/max_complexity, otherwise the DoS-protection half of
        // the "production" preset is inert (0 == unlimited, see `configure()`).
        let config = GraphQLConfig::production("/graphql");
        assert!(config.max_depth > 0, "production() must cap query depth");
        assert!(
            config.max_complexity > 0,
            "production() must cap query complexity"
        );
    }

    #[test]
    fn test_development_config() {
        let config = GraphQLConfig::development("/dev/graphql");
        assert_eq!(config.endpoint, "/dev/graphql");
        assert!(config.enable_playground);
        assert!(config.enable_graphiql);
        assert!(config.enable_schema_docs);
        assert!(config.enable_introspection);
        assert!(config.enable_tracing);
    }

    #[test]
    fn test_builder_pattern() {
        let config = GraphQLConfig::new("/graphql")
            .with_playground(false)
            .with_graphiql(true)
            .with_schema_docs(true)
            .with_max_depth(10)
            .with_max_complexity(100);

        assert!(!config.enable_playground);
        assert!(config.enable_graphiql);
        assert!(config.enable_schema_docs);
        assert_eq!(config.max_depth, 10);
        assert_eq!(config.max_complexity, 100);
    }

    #[test]
    fn test_custom_endpoints() {
        let config = GraphQLConfig::new("/api")
            .with_playground_endpoint("/api/play")
            .with_graphiql_endpoint("/api/iql")
            .with_schema_docs_endpoint("/api/docs");

        assert_eq!(config.playground_endpoint, "/api/play");
        assert_eq!(config.graphiql_endpoint, "/api/iql");
        assert_eq!(config.schema_docs_endpoint, "/api/docs");
    }

    // -------------------------------------------------------------------
    // GraphQLConfig::configure — regression tests.
    //
    // Prior to this fix, `configure` did not exist and nothing in the
    // crate ever called `.disable_introspection()`/`.limit_depth()` on
    // the async-graphql `SchemaBuilder`. A schema built from
    // `GraphQLConfig::production()` was therefore fully introspectable
    // and depth-unlimited despite the config's own docs claiming
    // otherwise. These tests fail against that prior behavior.
    // -------------------------------------------------------------------

    struct IntrospectionTestQuery;

    #[async_graphql::Object]
    impl IntrospectionTestQuery {
        async fn hello(&self) -> &str {
            "hi"
        }
    }

    #[test]
    fn test_configure_disables_introspection_for_production() {
        use async_graphql::{EmptyMutation, EmptySubscription, Schema};

        let config = GraphQLConfig::production("/graphql");
        let schema = config
            .configure(Schema::build(
                IntrospectionTestQuery,
                EmptyMutation,
                EmptySubscription,
            ))
            .finish();

        let response = tokio_test::block_on(schema.execute("{ __schema { types { name } } }"));

        // With introspection disabled, async-graphql resolves `__schema` to
        // `null` rather than erroring — so the regression check is that the
        // introspection data itself is absent, not that the query errors.
        let data = response.data.into_json().unwrap();
        assert!(
            data["__schema"].is_null(),
            "introspection query should return no schema data when GraphQLConfig::production() \
             is applied via configure(), but it returned: {:?}",
            data
        );
    }

    #[test]
    fn test_configure_denies_introspection_by_default() {
        use async_graphql::{EmptyMutation, EmptySubscription, Schema};

        let schema = GraphQLConfig::new("/graphql")
            .configure(Schema::build(
                IntrospectionTestQuery,
                EmptyMutation,
                EmptySubscription,
            ))
            .finish();
        let response = tokio_test::block_on(schema.execute("{ __schema { types { name } } }"));
        let data = response.data.into_json().unwrap();
        assert!(
            data["__schema"].is_null(),
            "GraphQLConfig::new() must not expose introspection: {data:?}"
        );
    }

    #[test]
    fn test_configure_allows_introspection_in_development() {
        use async_graphql::{EmptyMutation, EmptySubscription, Schema};

        let config = GraphQLConfig::development("/graphql");
        let schema = config
            .configure(Schema::build(
                IntrospectionTestQuery,
                EmptyMutation,
                EmptySubscription,
            ))
            .finish();

        let response = tokio_test::block_on(schema.execute("{ __schema { types { name } } }"));

        assert!(
            response.errors.is_empty(),
            "development config should allow introspection: {:?}",
            response.errors
        );

        let data = response.data.into_json().unwrap();
        assert!(
            !data["__schema"].is_null(),
            "development config should return real introspection data"
        );
    }

    struct Nested3;

    #[async_graphql::Object]
    impl Nested3 {
        async fn value(&self) -> i32 {
            3
        }
    }

    struct Nested2;

    #[async_graphql::Object]
    impl Nested2 {
        async fn nested3(&self) -> Nested3 {
            Nested3
        }
    }

    struct DepthTestQuery;

    #[async_graphql::Object]
    impl DepthTestQuery {
        async fn nested2(&self) -> Nested2 {
            Nested2
        }
    }

    #[test]
    fn test_configure_enforces_max_depth() {
        use async_graphql::{EmptyMutation, EmptySubscription, Schema};

        let config = GraphQLConfig::new("/graphql").with_max_depth(1);
        let schema = config
            .configure(Schema::build(
                DepthTestQuery,
                EmptyMutation,
                EmptySubscription,
            ))
            .finish();

        // Query depth here is 3: nested2 -> nested3 -> value
        let response = tokio_test::block_on(schema.execute("{ nested2 { nested3 { value } } }"));

        assert!(
            !response.errors.is_empty(),
            "query exceeding max_depth should be rejected when configured via configure(), \
             but it succeeded: {:?}",
            response.data
        );
    }

    #[test]
    fn test_configure_is_noop_when_limits_unset() {
        use async_graphql::{EmptyMutation, EmptySubscription, Schema};

        let config = GraphQLConfig::new("/graphql").with_max_depth(0); // explicit opt-out
        let schema = config
            .configure(Schema::build(
                DepthTestQuery,
                EmptyMutation,
                EmptySubscription,
            ))
            .finish();

        let response = tokio_test::block_on(schema.execute("{ nested2 { nested3 { value } } }"));

        assert!(
            response.errors.is_empty(),
            "unlimited depth config should not reject a depth-3 query: {:?}",
            response.errors
        );
    }

    struct CostQuery;

    #[async_graphql::Object]
    impl CostQuery {
        async fn value(&self) -> i32 {
            1
        }
    }

    #[test]
    fn test_default_config_rejects_alias_amplification() {
        use async_graphql::{EmptyMutation, EmptySubscription, Schema};

        // 1001 aliased copies of one field exceed DEFAULT_MAX_COMPLEXITY.
        let schema = GraphQLConfig::new("/graphql")
            .configure(Schema::build(CostQuery, EmptyMutation, EmptySubscription))
            .finish();
        let mut query = String::from("{");
        for i in 0..=DEFAULT_MAX_COMPLEXITY {
            query.push_str(&format!(" a{i}: value"));
        }
        query.push('}');
        let response = tokio_test::block_on(schema.execute(query));
        assert!(
            !response.errors.is_empty(),
            "alias amplification beyond the default complexity cap must be rejected"
        );

        let ok = tokio_test::block_on(schema.execute("{ a: value b: value }"));
        assert!(ok.errors.is_empty(), "{:?}", ok.errors);
    }

    #[test]
    fn test_default_config_rejects_excess_directives() {
        use async_graphql::{EmptyMutation, EmptySubscription, Schema};

        let schema = GraphQLConfig::new("/graphql")
            .configure(Schema::build(CostQuery, EmptyMutation, EmptySubscription))
            .finish();
        let directives = " @include(if: true)".repeat(DEFAULT_MAX_DIRECTIVES + 1);
        let response = tokio_test::block_on(schema.execute(format!("{{ value{directives} }}")));
        assert!(
            !response.errors.is_empty(),
            "more than DEFAULT_MAX_DIRECTIVES directives on a field must be rejected"
        );
    }
}
