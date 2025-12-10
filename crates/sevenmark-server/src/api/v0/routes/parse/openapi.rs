use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::parse::parse_endpoint,
    ),
    components(
        schemas(
            super::parse::ParseDocumentRequest,
            sevenmark_transform::ProcessedDocument,
        )
    ),
    tags(
        (name = "Parse", description = "Document parsing endpoints")
    )
)]
pub struct ParseApiDoc;
