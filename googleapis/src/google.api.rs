/// An indicator of the behavior of a given field (for example, that a field
/// is required in requests, or given as output but ignored as input).
/// This **does not** change the behavior in protocol buffers itself; it only
/// denotes the behavior and may affect how API tooling handles the field.
///
/// Note: This enum **may** receive new values in the future.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FieldBehavior {
    /// Conventional default for enums. Do not use this.
    Unspecified = 0,
    /// Specifically denotes a field as optional.
    /// While all fields in protocol buffers are optional, this may be specified
    /// for emphasis if appropriate.
    Optional = 1,
    /// Denotes a field as required.
    /// This indicates that the field **must** be provided as part of the request,
    /// and failure to do so will cause an error (usually `INVALID_ARGUMENT`).
    Required = 2,
    /// Denotes a field as output only.
    /// This indicates that the field is provided in responses, but including the
    /// field in a request does nothing (the server *must* ignore it and
    /// *must not* throw an error as a result of the field's presence).
    OutputOnly = 3,
    /// Denotes a field as input only.
    /// This indicates that the field is provided in requests, and the
    /// corresponding field is not included in output.
    InputOnly = 4,
    /// Denotes a field as immutable.
    /// This indicates that the field may be set once in a request to create a
    /// resource, but may not be changed thereafter.
    Immutable = 5,
    /// Denotes that a (repeated) field is an unordered list.
    /// This indicates that the service may provide the elements of the list
    /// in any arbitrary  order, rather than the order the user originally
    /// provided. Additionally, the list's order may or may not be stable.
    UnorderedList = 6,
    /// Denotes that this field returns a non-empty default value if not set.
    /// This indicates that if the user provides the empty value in a request,
    /// a non-empty value will be returned. The user will not be aware of what
    /// non-empty value to expect.
    NonEmptyDefault = 7,
}
/// A simple descriptor of a resource type.
///
/// ResourceDescriptor annotates a resource message (either by means of a
/// protobuf annotation or use in the service config), and associates the
/// resource's schema, the resource type, and the pattern of the resource name.
///
/// Example:
///
///     message Topic {
///       // Indicates this message defines a resource schema.
///       // Declares the resource type in the format of {service}/{kind}.
///       // For Kubernetes resources, the format is {api group}/{kind}.
///       option (google.api.resource) = {
///         type: "pubsub.googleapis.com/Topic"
///         pattern: "projects/{project}/topics/{topic}"
///       };
///     }
///
/// The ResourceDescriptor Yaml config will look like:
///
///     resources:
///     - type: "pubsub.googleapis.com/Topic"
///       pattern: "projects/{project}/topics/{topic}"
///
/// Sometimes, resources have multiple patterns, typically because they can
/// live under multiple parents.
///
/// Example:
///
///     message LogEntry {
///       option (google.api.resource) = {
///         type: "logging.googleapis.com/LogEntry"
///         pattern: "projects/{project}/logs/{log}"
///         pattern: "folders/{folder}/logs/{log}"
///         pattern: "organizations/{organization}/logs/{log}"
///         pattern: "billingAccounts/{billing_account}/logs/{log}"
///       };
///     }
///
/// The ResourceDescriptor Yaml config will look like:
///
///     resources:
///     - type: 'logging.googleapis.com/LogEntry'
///       pattern: "projects/{project}/logs/{log}"
///       pattern: "folders/{folder}/logs/{log}"
///       pattern: "organizations/{organization}/logs/{log}"
///       pattern: "billingAccounts/{billing_account}/logs/{log}"
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceDescriptor {
    /// The resource type. It must be in the format of
    /// {service_name}/{resource_type_kind}. The `resource_type_kind` must be
    /// singular and must not include version numbers.
    ///
    /// Example: `storage.googleapis.com/Bucket`
    ///
    /// The value of the resource_type_kind must follow the regular expression
    /// /\[A-Za-z][a-zA-Z0-9\]+/. It should start with an upper case character and
    /// should use PascalCase (UpperCamelCase). The maximum number of
    /// characters allowed for the `resource_type_kind` is 100.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional. The relative resource name pattern associated with this resource
    /// type. The DNS prefix of the full resource name shouldn't be specified here.
    ///
    /// The path pattern must follow the syntax, which aligns with HTTP binding
    /// syntax:
    ///
    ///     Template = Segment { "/" Segment } ;
    ///     Segment = LITERAL | Variable ;
    ///     Variable = "{" LITERAL "}" ;
    ///
    /// Examples:
    ///
    ///     - "projects/{project}/topics/{topic}"
    ///     - "projects/{project}/knowledgeBases/{knowledge_base}"
    ///
    /// The components in braces correspond to the IDs for each resource in the
    /// hierarchy. It is expected that, if multiple patterns are provided,
    /// the same component name (e.g. "project") refers to IDs of the same
    /// type of resource.
    #[prost(string, repeated, tag="2")]
    pub pattern: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The field on the resource that designates the resource name
    /// field. If omitted, this is assumed to be "name".
    #[prost(string, tag="3")]
    pub name_field: ::prost::alloc::string::String,
    /// Optional. The historical or future-looking state of the resource pattern.
    ///
    /// Example:
    ///
    ///     // The InspectTemplate message originally only supported resource
    ///     // names with organization, and project was added later.
    ///     message InspectTemplate {
    ///       option (google.api.resource) = {
    ///         type: "dlp.googleapis.com/InspectTemplate"
    ///         pattern:
    ///         "organizations/{organization}/inspectTemplates/{inspect_template}"
    ///         pattern: "projects/{project}/inspectTemplates/{inspect_template}"
    ///         history: ORIGINALLY_SINGLE_PATTERN
    ///       };
    ///     }
    #[prost(enumeration="resource_descriptor::History", tag="4")]
    pub history: i32,
    /// The plural name used in the resource name and permission names, such as
    /// 'projects' for the resource name of 'projects/{project}' and the permission
    /// name of 'cloudresourcemanager.googleapis.com/projects.get'. It is the same
    /// concept of the `plural` field in k8s CRD spec
    /// <https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions/>
    ///
    /// Note: The plural form is required even for singleton resources. See
    /// <https://aip.dev/156>
    #[prost(string, tag="5")]
    pub plural: ::prost::alloc::string::String,
    /// The same concept of the `singular` field in k8s CRD spec
    /// <https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions/>
    /// Such as "project" for the `resourcemanager.googleapis.com/Project` type.
    #[prost(string, tag="6")]
    pub singular: ::prost::alloc::string::String,
    /// Style flag(s) for this resource.
    /// These indicate that a resource is expected to conform to a given
    /// style. See the specific style flags for additional information.
    #[prost(enumeration="resource_descriptor::Style", repeated, tag="10")]
    pub style: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `ResourceDescriptor`.
pub mod resource_descriptor {
    /// A description of the historical or future-looking state of the
    /// resource pattern.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum History {
        /// The "unset" value.
        Unspecified = 0,
        /// The resource originally had one pattern and launched as such, and
        /// additional patterns were added later.
        OriginallySinglePattern = 1,
        /// The resource has one pattern, but the API owner expects to add more
        /// later. (This is the inverse of ORIGINALLY_SINGLE_PATTERN, and prevents
        /// that from being necessary once there are multiple patterns.)
        FutureMultiPattern = 2,
    }
    /// A flag representing a specific style that a resource claims to conform to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Style {
        /// The unspecified value. Do not use.
        Unspecified = 0,
        /// This resource is intended to be "declarative-friendly".
        ///
        /// Declarative-friendly resources must be more strictly consistent, and
        /// setting this to true communicates to tools that this resource should
        /// adhere to declarative-friendly expectations.
        ///
        /// Note: This is used by the API linter (linter.aip.dev) to enable
        /// additional checks.
        DeclarativeFriendly = 1,
    }
}
/// Defines a proto annotation that describes a string field that refers to
/// an API resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceReference {
    /// The resource type that the annotated field references.
    ///
    /// Example:
    ///
    ///     message Subscription {
    ///       string topic = 2 [(google.api.resource_reference) = {
    ///         type: "pubsub.googleapis.com/Topic"
    ///       }];
    ///     }
    ///
    /// Occasionally, a field may reference an arbitrary resource. In this case,
    /// APIs use the special value * in their resource reference.
    ///
    /// Example:
    ///
    ///     message GetIamPolicyRequest {
    ///       string resource = 2 [(google.api.resource_reference) = {
    ///         type: "*"
    ///       }];
    ///     }
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// The resource type of a child collection that the annotated field
    /// references. This is useful for annotating the `parent` field that
    /// doesn't have a fixed resource type.
    ///
    /// Example:
    ///
    ///     message ListLogEntriesRequest {
    ///       string parent = 1 [(google.api.resource_reference) = {
    ///         child_type: "logging.googleapis.com/LogEntry"
    ///       };
    ///     }
    #[prost(string, tag="2")]
    pub child_type: ::prost::alloc::string::String,
}
/// Specifies the routing information that should be sent along with the request
/// in the form of routing header.
/// **NOTE:** All service configuration rules follow the "last one wins" order.
///
/// The examples below will apply to an RPC which has the following request type:
///
/// Message Definition:
///
///     message Request {
///       // The name of the Table
///       // Values can be of the following formats:
///       // - `projects/<project>/tables/<table>`
///       // - `projects/<project>/instances/<instance>/tables/<table>`
///       // - `region/<region>/zones/<zone>/tables/<table>`
///       string table_name = 1;
///
///       // This value specifies routing for replication.
///       // It can be in the following formats:
///       // - `profiles/<profile_id>`
///       // - a legacy `profile_id` that can be any string
///       string app_profile_id = 2;
///     }
///
/// Example message:
///
///     {
///       table_name: projects/proj_foo/instances/instance_bar/table/table_baz,
///       app_profile_id: profiles/prof_qux
///     }
///
/// The routing header consists of one or multiple key-value pairs. Every key
/// and value must be percent-encoded, and joined together in the format of
/// `key1=value1&key2=value2`.
/// In the examples below I am skipping the percent-encoding for readablity.
///
/// Example 1
///
/// Extracting a field from the request to put into the routing header
/// unchanged, with the key equal to the field name.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // Take the `app_profile_id`.
///       routing_parameters {
///         field: "app_profile_id"
///       }
///     };
///
/// result:
///
///     x-goog-request-params: app_profile_id=profiles/prof_qux
///
/// Example 2
///
/// Extracting a field from the request to put into the routing header
/// unchanged, with the key different from the field name.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // Take the `app_profile_id`, but name it `routing_id` in the header.
///       routing_parameters {
///         field: "app_profile_id"
///         path_template: "{routing_id=**}"
///       }
///     };
///
/// result:
///
///     x-goog-request-params: routing_id=profiles/prof_qux
///
/// Example 3
///
/// Extracting a field from the request to put into the routing
/// header, while matching a path template syntax on the field's value.
///
/// NB: it is more useful to send nothing than to send garbage for the purpose
/// of dynamic routing, since garbage pollutes cache. Thus the matching.
///
/// Sub-example 3a
///
/// The field matches the template.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // Take the `table_name`, if it's well-formed (with project-based
///       // syntax).
///       routing_parameters {
///         field: "table_name"
///         path_template: "{table_name=projects/*/instances/*/**}"
///       }
///     };
///
/// result:
///
///     x-goog-request-params:
///     table_name=projects/proj_foo/instances/instance_bar/table/table_baz
///
/// Sub-example 3b
///
/// The field does not match the template.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // Take the `table_name`, if it's well-formed (with region-based
///       // syntax).
///       routing_parameters {
///         field: "table_name"
///         path_template: "{table_name=regions/*/zones/*/**}"
///       }
///     };
///
/// result:
///
///     <no routing header will be sent>
///
/// Sub-example 3c
///
/// Multiple alternative conflictingly named path templates are
/// specified. The one that matches is used to construct the header.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // Take the `table_name`, if it's well-formed, whether
///       // using the region- or projects-based syntax.
///
///       routing_parameters {
///         field: "table_name"
///         path_template: "{table_name=regions/*/zones/*/**}"
///       }
///       routing_parameters {
///         field: "table_name"
///         path_template: "{table_name=projects/*/instances/*/**}"
///       }
///     };
///
/// result:
///
///     x-goog-request-params:
///     table_name=projects/proj_foo/instances/instance_bar/table/table_baz
///
/// Example 4
///
/// Extracting a single routing header key-value pair by matching a
/// template syntax on (a part of) a single request field.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // Take just the project id from the `table_name` field.
///       routing_parameters {
///         field: "table_name"
///         path_template: "{routing_id=projects/*}/**"
///       }
///     };
///
/// result:
///
///     x-goog-request-params: routing_id=projects/proj_foo
///
/// Example 5
///
/// Extracting a single routing header key-value pair by matching
/// several conflictingly named path templates on (parts of) a single request
/// field. The last template to match "wins" the conflict.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // If the `table_name` does not have instances information,
///       // take just the project id for routing.
///       // Otherwise take project + instance.
///
///       routing_parameters {
///         field: "table_name"
///         path_template: "{routing_id=projects/*}/**"
///       }
///       routing_parameters {
///         field: "table_name"
///         path_template: "{routing_id=projects/*/instances/*}/**"
///       }
///     };
///
/// result:
///
///     x-goog-request-params:
///     routing_id=projects/proj_foo/instances/instance_bar
///
/// Example 6
///
/// Extracting multiple routing header key-value pairs by matching
/// several non-conflicting path templates on (parts of) a single request field.
///
/// Sub-example 6a
///
/// Make the templates strict, so that if the `table_name` does not
/// have an instance information, nothing is sent.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // The routing code needs two keys instead of one composite
///       // but works only for the tables with the "project-instance" name
///       // syntax.
///
///       routing_parameters {
///         field: "table_name"
///         path_template: "{project_id=projects/*}/instances/*/**"
///       }
///       routing_parameters {
///         field: "table_name"
///         path_template: "projects/*/{instance_id=instances/*}/**"
///       }
///     };
///
/// result:
///
///     x-goog-request-params:
///     project_id=projects/proj_foo&instance_id=instances/instance_bar
///
/// Sub-example 6b
///
/// Make the templates loose, so that if the `table_name` does not
/// have an instance information, just the project id part is sent.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // The routing code wants two keys instead of one composite
///       // but will work with just the `project_id` for tables without
///       // an instance in the `table_name`.
///
///       routing_parameters {
///         field: "table_name"
///         path_template: "{project_id=projects/*}/**"
///       }
///       routing_parameters {
///         field: "table_name"
///         path_template: "projects/*/{instance_id=instances/*}/**"
///       }
///     };
///
/// result (is the same as 6a for our example message because it has the instance
/// information):
///
///     x-goog-request-params:
///     project_id=projects/proj_foo&instance_id=instances/instance_bar
///
/// Example 7
///
/// Extracting multiple routing header key-value pairs by matching
/// several path templates on multiple request fields.
///
/// NB: note that here there is no way to specify sending nothing if one of the
/// fields does not match its template. E.g. if the `table_name` is in the wrong
/// format, the `project_id` will not be sent, but the `routing_id` will be.
/// The backend routing code has to be aware of that and be prepared to not
/// receive a full complement of keys if it expects multiple.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // The routing needs both `project_id` and `routing_id`
///       // (from the `app_profile_id` field) for routing.
///
///       routing_parameters {
///         field: "table_name"
///         path_template: "{project_id=projects/*}/**"
///       }
///       routing_parameters {
///         field: "app_profile_id"
///         path_template: "{routing_id=**}"
///       }
///     };
///
/// result:
///
///     x-goog-request-params:
///     project_id=projects/proj_foo&routing_id=profiles/prof_qux
///
/// Example 8
///
/// Extracting a single routing header key-value pair by matching
/// several conflictingly named path templates on several request fields. The
/// last template to match "wins" the conflict.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // The `routing_id` can be a project id or a region id depending on
///       // the table name format, but only if the `app_profile_id` is not set.
///       // If `app_profile_id` is set it should be used instead.
///
///       routing_parameters {
///         field: "table_name"
///         path_template: "{routing_id=projects/*}/**"
///       }
///       routing_parameters {
///          field: "table_name"
///          path_template: "{routing_id=regions/*}/**"
///       }
///       routing_parameters {
///         field: "app_profile_id"
///         path_template: "{routing_id=**}"
///       }
///     };
///
/// result:
///
///     x-goog-request-params: routing_id=profiles/prof_qux
///
/// Example 9
///
/// Bringing it all together.
///
/// annotation:
///
///     option (google.api.routing) = {
///       // For routing both `table_location` and a `routing_id` are needed.
///       //
///       // table_location can be either an instance id or a region+zone id.
///       //
///       // For `routing_id`, take the value of `app_profile_id`
///       // - If it's in the format `profiles/<profile_id>`, send
///       // just the `<profile_id>` part.
///       // - If it's any other literal, send it as is.
///       // If the `app_profile_id` is empty, and the `table_name` starts with
///       // the project_id, send that instead.
///
///       routing_parameters {
///         field: "table_name"
///         path_template: "projects/*/{table_location=instances/*}/tables/*"
///       }
///       routing_parameters {
///         field: "table_name"
///         path_template: "{table_location=regions/*/zones/*}/tables/*"
///       }
///       routing_parameters {
///         field: "table_name"
///         path_template: "{routing_id=projects/*}/**"
///       }
///       routing_parameters {
///         field: "app_profile_id"
///         path_template: "{routing_id=**}"
///       }
///       routing_parameters {
///         field: "app_profile_id"
///         path_template: "profiles/{routing_id=*}"
///       }
///     };
///
/// result:
///
///     x-goog-request-params:
///     table_location=instances/instance_bar&routing_id=prof_qux
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingRule {
    /// A collection of Routing Parameter specifications.
    /// **NOTE:** If multiple Routing Parameters describe the same key
    /// (via the `path_template` field or via the `field` field when
    /// `path_template` is not provided), "last one wins" rule
    /// determines which Parameter gets used.
    /// See the examples for more details.
    #[prost(message, repeated, tag="2")]
    pub routing_parameters: ::prost::alloc::vec::Vec<RoutingParameter>,
}
/// A projection from an input message to the GRPC or REST header.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingParameter {
    /// A request field to extract the header key-value pair from.
    #[prost(string, tag="1")]
    pub field: ::prost::alloc::string::String,
    /// A pattern matching the key-value field. Optional.
    /// If not specified, the whole field specified in the `field` field will be
    /// taken as value, and its name used as key. If specified, it MUST contain
    /// exactly one named segment (along with any number of unnamed segments) The
    /// pattern will be matched over the field specified in the `field` field, then
    /// if the match is successful:
    /// - the name of the single named segment will be used as a header name,
    /// - the match value of the segment will be used as a header value;
    /// if the match is NOT successful, nothing will be sent.
    ///
    /// Example:
    ///
    ///               -- This is a field in the request message
    ///              |   that the header value will be extracted from.
    ///              |
    ///              |                     -- This is the key name in the
    ///              |                    |   routing header.
    ///              V                    |
    ///     field: "table_name"           v
    ///     path_template: "projects/*/{table_location=instances/*}/tables/*"
    ///                                                ^            ^
    ///                                                |            |
    ///       In the {} brackets is the pattern that --             |
    ///       specifies what to extract from the                    |
    ///       field as a value to be sent.                          |
    ///                                                             |
    ///      The string in the field must match the whole pattern --
    ///      before brackets, inside brackets, after brackets.
    ///
    /// When looking at this specific example, we can see that:
    /// - A key-value pair with the key `table_location`
    ///   and the value matching `instances/*` should be added
    ///   to the x-goog-request-params routing header.
    /// - The value is extracted from the request message's `table_name` field
    ///   if it matches the full pattern specified:
    ///   `projects/*/instances/*/tables/*`.
    ///
    /// **NB:** If the `path_template` field is not provided, the key name is
    /// equal to the field name, and the whole field should be sent as a value.
    /// This makes the pattern for the field and the value functionally equivalent
    /// to `**`, and the configuration
    ///
    ///     {
    ///       field: "table_name"
    ///     }
    ///
    /// is a functionally equivalent shorthand to:
    ///
    ///     {
    ///       field: "table_name"
    ///       path_template: "{table_name=**}"
    ///     }
    ///
    /// See Example 1 for more details.
    #[prost(string, tag="2")]
    pub path_template: ::prost::alloc::string::String,
}
/// Defines the HTTP configuration for an API service. It contains a list of
/// \[HttpRule][google.api.HttpRule\], each specifying the mapping of an RPC method
/// to one or more HTTP REST API methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http {
    /// A list of HTTP configuration rules that apply to individual API methods.
    ///
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    #[prost(message, repeated, tag="1")]
    pub rules: ::prost::alloc::vec::Vec<HttpRule>,
    /// When set to true, URL path parameters will be fully URI-decoded except in
    /// cases of single segment matches in reserved expansion, where "%2F" will be
    /// left encoded.
    ///
    /// The default behavior is to not decode RFC 6570 reserved characters in multi
    /// segment matches.
    #[prost(bool, tag="2")]
    pub fully_decode_reserved_expansion: bool,
}
/// # gRPC Transcoding
///
/// gRPC Transcoding is a feature for mapping between a gRPC method and one or
/// more HTTP REST endpoints. It allows developers to build a single API service
/// that supports both gRPC APIs and REST APIs. Many systems, including [Google
/// APIs](<https://github.com/googleapis/googleapis>),
/// [Cloud Endpoints](<https://cloud.google.com/endpoints>), [gRPC
/// Gateway](<https://github.com/grpc-ecosystem/grpc-gateway>),
/// and \[Envoy\](<https://github.com/envoyproxy/envoy>) proxy support this feature
/// and use it for large scale production services.
///
/// `HttpRule` defines the schema of the gRPC/REST mapping. The mapping specifies
/// how different portions of the gRPC request message are mapped to the URL
/// path, URL query parameters, and HTTP request body. It also controls how the
/// gRPC response message is mapped to the HTTP response body. `HttpRule` is
/// typically specified as an `google.api.http` annotation on the gRPC method.
///
/// Each mapping specifies a URL path template and an HTTP method. The path
/// template may refer to one or more fields in the gRPC request message, as long
/// as each field is a non-repeated field with a primitive (non-message) type.
/// The path template controls how fields of the request message are mapped to
/// the URL path.
///
/// Example:
///
///     service Messaging {
///       rpc GetMessage(GetMessageRequest) returns (Message) {
///         option (google.api.http) = {
///             get: "/v1/{name=messages/*}"
///         };
///       }
///     }
///     message GetMessageRequest {
///       string name = 1; // Mapped to URL path.
///     }
///     message Message {
///       string text = 1; // The resource content.
///     }
///
/// This enables an HTTP REST to gRPC mapping as below:
///
/// HTTP | gRPC
/// -----|-----
/// `GET /v1/messages/123456`  | `GetMessage(name: "messages/123456")`
///
/// Any fields in the request message which are not bound by the path template
/// automatically become HTTP query parameters if there is no HTTP request body.
/// For example:
///
///     service Messaging {
///       rpc GetMessage(GetMessageRequest) returns (Message) {
///         option (google.api.http) = {
///             get:"/v1/messages/{message_id}"
///         };
///       }
///     }
///     message GetMessageRequest {
///       message SubMessage {
///         string subfield = 1;
///       }
///       string message_id = 1; // Mapped to URL path.
///       int64 revision = 2;    // Mapped to URL query parameter `revision`.
///       SubMessage sub = 3;    // Mapped to URL query parameter `sub.subfield`.
///     }
///
/// This enables a HTTP JSON to RPC mapping as below:
///
/// HTTP | gRPC
/// -----|-----
/// `GET /v1/messages/123456?revision=2&sub.subfield=foo` |
/// `GetMessage(message_id: "123456" revision: 2 sub: SubMessage(subfield:
/// "foo"))`
///
/// Note that fields which are mapped to URL query parameters must have a
/// primitive type or a repeated primitive type or a non-repeated message type.
/// In the case of a repeated type, the parameter can be repeated in the URL
/// as `...?param=A&param=B`. In the case of a message type, each field of the
/// message is mapped to a separate parameter, such as
/// `...?foo.a=A&foo.b=B&foo.c=C`.
///
/// For HTTP methods that allow a request body, the `body` field
/// specifies the mapping. Consider a REST update method on the
/// message resource collection:
///
///     service Messaging {
///       rpc UpdateMessage(UpdateMessageRequest) returns (Message) {
///         option (google.api.http) = {
///           patch: "/v1/messages/{message_id}"
///           body: "message"
///         };
///       }
///     }
///     message UpdateMessageRequest {
///       string message_id = 1; // mapped to the URL
///       Message message = 2;   // mapped to the body
///     }
///
/// The following HTTP JSON to RPC mapping is enabled, where the
/// representation of the JSON in the request body is determined by
/// protos JSON encoding:
///
/// HTTP | gRPC
/// -----|-----
/// `PATCH /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id:
/// "123456" message { text: "Hi!" })`
///
/// The special name `*` can be used in the body mapping to define that
/// every field not bound by the path template should be mapped to the
/// request body.  This enables the following alternative definition of
/// the update method:
///
///     service Messaging {
///       rpc UpdateMessage(Message) returns (Message) {
///         option (google.api.http) = {
///           patch: "/v1/messages/{message_id}"
///           body: "*"
///         };
///       }
///     }
///     message Message {
///       string message_id = 1;
///       string text = 2;
///     }
///
///
/// The following HTTP JSON to RPC mapping is enabled:
///
/// HTTP | gRPC
/// -----|-----
/// `PATCH /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id:
/// "123456" text: "Hi!")`
///
/// Note that when using `*` in the body mapping, it is not possible to
/// have HTTP parameters, as all fields not bound by the path end in
/// the body. This makes this option more rarely used in practice when
/// defining REST APIs. The common usage of `*` is in custom methods
/// which don't use the URL at all for transferring data.
///
/// It is possible to define multiple HTTP methods for one RPC by using
/// the `additional_bindings` option. Example:
///
///     service Messaging {
///       rpc GetMessage(GetMessageRequest) returns (Message) {
///         option (google.api.http) = {
///           get: "/v1/messages/{message_id}"
///           additional_bindings {
///             get: "/v1/users/{user_id}/messages/{message_id}"
///           }
///         };
///       }
///     }
///     message GetMessageRequest {
///       string message_id = 1;
///       string user_id = 2;
///     }
///
/// This enables the following two alternative HTTP JSON to RPC mappings:
///
/// HTTP | gRPC
/// -----|-----
/// `GET /v1/messages/123456` | `GetMessage(message_id: "123456")`
/// `GET /v1/users/me/messages/123456` | `GetMessage(user_id: "me" message_id:
/// "123456")`
///
/// ## Rules for HTTP mapping
///
/// 1. Leaf request fields (recursive expansion nested messages in the request
///    message) are classified into three categories:
///    - Fields referred by the path template. They are passed via the URL path.
///    - Fields referred by the \[HttpRule.body][google.api.HttpRule.body\]. They are passed via the HTTP
///      request body.
///    - All other fields are passed via the URL query parameters, and the
///      parameter name is the field path in the request message. A repeated
///      field can be represented as multiple query parameters under the same
///      name.
///  2. If \[HttpRule.body][google.api.HttpRule.body\] is "*", there is no URL query parameter, all fields
///     are passed via URL path and HTTP request body.
///  3. If \[HttpRule.body][google.api.HttpRule.body\] is omitted, there is no HTTP request body, all
///     fields are passed via URL path and URL query parameters.
///
/// ### Path template syntax
///
///     Template = "/" Segments [ Verb ] ;
///     Segments = Segment { "/" Segment } ;
///     Segment  = "*" | "**" | LITERAL | Variable ;
///     Variable = "{" FieldPath [ "=" Segments ] "}" ;
///     FieldPath = IDENT { "." IDENT } ;
///     Verb     = ":" LITERAL ;
///
/// The syntax `*` matches a single URL path segment. The syntax `**` matches
/// zero or more URL path segments, which must be the last part of the URL path
/// except the `Verb`.
///
/// The syntax `Variable` matches part of the URL path as specified by its
/// template. A variable template must not contain other variables. If a variable
/// matches a single path segment, its template may be omitted, e.g. `{var}`
/// is equivalent to `{var=*}`.
///
/// The syntax `LITERAL` matches literal text in the URL path. If the `LITERAL`
/// contains any reserved character, such characters should be percent-encoded
/// before the matching.
///
/// If a variable contains exactly one path segment, such as `"{var}"` or
/// `"{var=*}"`, when such a variable is expanded into a URL path on the client
/// side, all characters except `\[-_.~0-9a-zA-Z\]` are percent-encoded. The
/// server side does the reverse decoding. Such variables show up in the
/// [Discovery
/// Document](<https://developers.google.com/discovery/v1/reference/apis>) as
/// `{var}`.
///
/// If a variable contains multiple path segments, such as `"{var=foo/*}"`
/// or `"{var=**}"`, when such a variable is expanded into a URL path on the
/// client side, all characters except `\[-_.~/0-9a-zA-Z\]` are percent-encoded.
/// The server side does the reverse decoding, except "%2F" and "%2f" are left
/// unchanged. Such variables show up in the
/// [Discovery
/// Document](<https://developers.google.com/discovery/v1/reference/apis>) as
/// `{+var}`.
///
/// ## Using gRPC API Service Configuration
///
/// gRPC API Service Configuration (service config) is a configuration language
/// for configuring a gRPC service to become a user-facing product. The
/// service config is simply the YAML representation of the `google.api.Service`
/// proto message.
///
/// As an alternative to annotating your proto file, you can configure gRPC
/// transcoding in your service config YAML files. You do this by specifying a
/// `HttpRule` that maps the gRPC method to a REST endpoint, achieving the same
/// effect as the proto annotation. This can be particularly useful if you
/// have a proto that is reused in multiple services. Note that any transcoding
/// specified in the service config will override any matching transcoding
/// configuration in the proto.
///
/// Example:
///
///     http:
///       rules:
///         # Selects a gRPC method and applies HttpRule to it.
///         - selector: example.v1.Messaging.GetMessage
///           get: /v1/messages/{message_id}/{sub.subfield}
///
/// ## Special notes
///
/// When gRPC Transcoding is used to map a gRPC to JSON REST endpoints, the
/// proto to JSON conversion must follow the [proto3
/// specification](<https://developers.google.com/protocol-buffers/docs/proto3#json>).
///
/// While the single segment variable follows the semantics of
/// [RFC 6570](<https://tools.ietf.org/html/rfc6570>) Section 3.2.2 Simple String
/// Expansion, the multi segment variable **does not** follow RFC 6570 Section
/// 3.2.3 Reserved Expansion. The reason is that the Reserved Expansion
/// does not expand special characters like `?` and `#`, which would lead
/// to invalid URLs. As the result, gRPC Transcoding uses a custom encoding
/// for multi segment variables.
///
/// The path variables **must not** refer to any repeated or mapped field,
/// because client libraries are not capable of handling such variable expansion.
///
/// The path variables **must not** capture the leading "/" character. The reason
/// is that the most common use case "{var}" does not capture the leading "/"
/// character. For consistency, all path variables must share the same behavior.
///
/// Repeated message fields must not be mapped to URL query parameters, because
/// no client library can support such complicated mapping.
///
/// If an API needs to use a JSON array for request or response body, it can map
/// the request or response body to a repeated field. However, some gRPC
/// Transcoding implementations may not support this feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRule {
    /// Selects a method to which this rule applies.
    ///
    /// Refer to \[selector][google.api.DocumentationRule.selector\] for syntax details.
    #[prost(string, tag="1")]
    pub selector: ::prost::alloc::string::String,
    /// The name of the request field whose value is mapped to the HTTP request
    /// body, or `*` for mapping all request fields not captured by the path
    /// pattern to the HTTP body, or omitted for not having any HTTP request body.
    ///
    /// NOTE: the referred field must be present at the top-level of the request
    /// message type.
    #[prost(string, tag="7")]
    pub body: ::prost::alloc::string::String,
    /// Optional. The name of the response field whose value is mapped to the HTTP
    /// response body. When omitted, the entire response message will be used
    /// as the HTTP response body.
    ///
    /// NOTE: The referred field must be present at the top-level of the response
    /// message type.
    #[prost(string, tag="12")]
    pub response_body: ::prost::alloc::string::String,
    /// Additional HTTP bindings for the selector. Nested bindings must
    /// not contain an `additional_bindings` field themselves (that is,
    /// the nesting may only be one level deep).
    #[prost(message, repeated, tag="11")]
    pub additional_bindings: ::prost::alloc::vec::Vec<HttpRule>,
    /// Determines the URL pattern is matched by this rules. This pattern can be
    /// used with any of the {get|put|post|delete|patch} methods. A custom method
    /// can be defined using the 'custom' field.
    #[prost(oneof="http_rule::Pattern", tags="2, 3, 4, 5, 6, 8")]
    pub pattern: ::core::option::Option<http_rule::Pattern>,
}
/// Nested message and enum types in `HttpRule`.
pub mod http_rule {
    /// Determines the URL pattern is matched by this rules. This pattern can be
    /// used with any of the {get|put|post|delete|patch} methods. A custom method
    /// can be defined using the 'custom' field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pattern {
        /// Maps to HTTP GET. Used for listing and getting information about
        /// resources.
        #[prost(string, tag="2")]
        Get(::prost::alloc::string::String),
        /// Maps to HTTP PUT. Used for replacing a resource.
        #[prost(string, tag="3")]
        Put(::prost::alloc::string::String),
        /// Maps to HTTP POST. Used for creating a resource or performing an action.
        #[prost(string, tag="4")]
        Post(::prost::alloc::string::String),
        /// Maps to HTTP DELETE. Used for deleting a resource.
        #[prost(string, tag="5")]
        Delete(::prost::alloc::string::String),
        /// Maps to HTTP PATCH. Used for updating a resource.
        #[prost(string, tag="6")]
        Patch(::prost::alloc::string::String),
        /// The custom pattern is used for specifying an HTTP method that is not
        /// included in the `pattern` field, such as HEAD, or "*" to leave the
        /// HTTP method unspecified for this rule. The wild-card rule is useful
        /// for services that provide content to Web (HTML) clients.
        #[prost(message, tag="8")]
        Custom(super::CustomHttpPattern),
    }
}
/// A custom pattern is used for defining custom HTTP verb.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomHttpPattern {
    /// The name of this custom HTTP verb.
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    /// The path matched by this custom verb.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
