use sevenmark_parser::core::parse_document;
use sevenmark_renderer::render;
use sevenmark_server::connection::database_conn::establish_connection;
use sevenmark_transform::process_sevenmark;
use std::fs;
use std::time::Instant;

const SEVENMARK_CSS: &str = include_str!("../../sevenmark-renderer/assets/sevenmark.css");

#[tokio::main]
async fn main() {
    let input_content = fs::read_to_string("ToParse.txt").expect("ToParse.txt file not found");
    let document_len = input_content.len();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("Input ({} bytes)\n{}\n", document_len, "=".repeat(50));

    // Database connection
    let db = establish_connection().await;
    println!("Database connected\n");

    let start_time = Instant::now();

    // 1. Parse
    let parse_start = Instant::now();
    let ast = parse_document(&input_content);
    let parse_duration = parse_start.elapsed();
    println!("Parsed {} elements in {:?}", ast.len(), parse_duration);

    // 2. Process (preprocessing with DB)
    let process_start = Instant::now();
    let processed = match process_sevenmark(ast, &db).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error processing: {}", e);
            std::process::exit(1);
        }
    };
    let process_duration = process_start.elapsed();
    println!(
        "Processed {} elements in {:?}",
        processed.ast.len(),
        process_duration
    );

    // 3. Render
    let render_start = Instant::now();
    let html = render(&processed.ast);
    let render_duration = render_start.elapsed();
    println!("Rendered {} bytes in {:?}", html.len(), render_duration);

    let total_duration = start_time.elapsed();

    // Save raw HTML
    fs::write("RenderResult.html", &html).ok();

    // Save full HTML with CSS
    let full_html = format!(
        r#"<!DOCTYPE html>
<html lang="ja" data-theme="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SevenMark Render Result</title>
    <!-- Pretendard Font -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable.min.css">
    <!-- KaTeX -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css">
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
    <!-- Prism.js (Dark theme) -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/themes/prism-tomorrow.min.css">
    <script defer src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/prism.min.js"></script>
    <script defer src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>
    <style>
        body {{
            font-family: "Pretendard Variable", Pretendard, -apple-system, BlinkMacSystemFont, sans-serif;
            max-width: 900px;
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
        // KaTeX
        document.querySelectorAll('.sm-tex').forEach(el => {{
            if (typeof katex !== 'undefined') {{
                katex.render(el.textContent, el, {{
                    throwOnError: false,
                    displayMode: el.classList.contains('sm-tex-block')
                }});
            }}
        }});
        // Prism
        if (typeof Prism !== 'undefined') {{
            Prism.highlightAll();
        }}
    }});
</script>
</body>
</html>"#,
        css = SEVENMARK_CSS
    );

    fs::write("RenderResult_full.html", &full_html).ok();

    println!("\n=== Results ===");
    println!("Categories: {:?}", processed.categories);
    if let Some(redirect) = &processed.redirect {
        println!("Redirect: {:?}:{}", redirect.namespace, redirect.title);
    }
    println!("\nSaved:");
    println!("  - RenderResult.html (raw)");
    println!("  - RenderResult_full.html (full)");

    println!(
        "\nPerformance: Total {:?} ({:.2} KB/s)",
        total_duration,
        document_len as f64 / 1024.0 / total_duration.as_secs_f64()
    );
}
