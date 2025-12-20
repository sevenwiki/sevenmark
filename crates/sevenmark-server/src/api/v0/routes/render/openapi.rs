use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::render::render_endpoint,
    ),
    components(
        schemas(
            super::render::RenderDocumentRequest,
            super::render::RenderedDocument,
        )
    ),
    tags(
        (name = "Render", description = "Document rendering endpoints")
    )
)]
pub struct RenderApiDoc;