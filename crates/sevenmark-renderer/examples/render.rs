use sevenmark_parser::core::parse_document;
use sevenmark_renderer::render;
use std::fs;
use std::time::Instant;

const SEVENMARK_CSS: &str = include_str!("../assets/sevenmark.css");

fn main() {
    let input_content = fs::read_to_string("ToParse.txt").expect("ToParse.txt file not found");
    let document_len = input_content.len();

    println!("Input ({} bytes):\n{}\n", document_len, "=".repeat(50));

    // Parse
    let parse_start = Instant::now();
    let ast = parse_document(&input_content);
    let parse_duration = parse_start.elapsed();
    println!("Parsed {} elements in {:?}", ast.len(), parse_duration);

    // Render
    let render_start = Instant::now();
    let html = render(&ast);
    let render_duration = render_start.elapsed();
    println!("Rendered to {} bytes in {:?}", html.len(), render_duration);

    // Save HTML
    fs::write("RenderResult.html", &html).ok();
    println!("\nResult saved to RenderResult.html");

    // Also save wrapped HTML for easy viewing
    let wrapped_html = format!(
        r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SevenMark Render Result</title>
    <!-- Pretendard Font -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable.min.css">
    <!-- KaTeX -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css">
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js"></script>
    <!-- Prism.js -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/themes/prism.min.css">
    <script defer src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/prism.min.js"></script>
    <script defer src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>
    <style>
        body {{
            font-family: "Pretendard Variable", Pretendard, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
            line-height: 1.6;
            color: var(--sm-text);
            background: var(--sm-bg);
        }}
        {css}
    </style>
</head>
<body>
<article>
{html}
</article>
<script>
    document.addEventListener("DOMContentLoaded", function() {{
        // KaTeX auto-render
        if (typeof renderMathInElement !== 'undefined') {{
            document.querySelectorAll('.sm-tex').forEach(el => {{
                katex.render(el.textContent, el, {{ throwOnError: false, displayMode: el.classList.contains('sm-tex-block') }});
            }});
        }}
        // Prism highlight
        if (typeof Prism !== 'undefined') {{
            Prism.highlightAll();
        }}
    }});
</script>
</body>
</html>"#,
        css = SEVENMARK_CSS
    );
    fs::write("RenderResult_full.html", &wrapped_html).ok();
    println!("Full HTML saved to RenderResult_full.html");

    println!(
        "\nPerformance: Parse {:.2} KB/s, Render {:.2} KB/s",
        document_len as f64 / 1024.0 / parse_duration.as_secs_f64(),
        document_len as f64 / 1024.0 / render_duration.as_secs_f64()
    );
}
