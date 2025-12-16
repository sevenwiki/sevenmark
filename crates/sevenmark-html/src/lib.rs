pub fn render(ast: &[SevenMarkElement]) -> String {
    let mut ctx = RenderContext::new();
    render_with_context(ast, &mut ctx)
}
