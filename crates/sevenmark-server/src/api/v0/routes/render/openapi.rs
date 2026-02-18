use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::render_document::render_document,
        super::render_discussion::render_discussion,
    ),
    components(
        schemas(
            super::render_document::RenderDocumentRequest,
            super::render_document::RenderedDocument,
            super::render_discussion::RenderDiscussionRequest,
            super::render_discussion::RenderedDiscussion,
        )
    ),
    tags(
        (name = "Render", description = "Document rendering endpoints")
    )
)]
pub struct RenderApiDoc;
