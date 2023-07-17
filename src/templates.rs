pub fn create_page(title: &str, html_body_inner_html: &str) -> String {
    // note: we'll get a compiler error here until the tailwind build occurs.
    // Make sure you use `make build` in the Makefile to get both to happen
    // together
    let tailwind = include_str!("./tailwind.generated.css");
    format!(
        r#"
    <html>
        <head>
            <meta name="viewport" content="width=device-width, initial-scale=1.0"></meta>
            <title>{title}</title>
            <style>
                {tailwind}
            </style>
        </head>
        <body>
            {body_html}
            <script src="https://unpkg.com/htmx.org@1.9.3" integrity="sha384-lVb3Rd/Ca0AxaoZg5sACe8FJKF0tnUgR2Kd7ehUOG5GCcROv5uBIZsOqovBAcWua" crossorigin="anonymous"></script>
        </body>
    </html>
    "#,
        tailwind = tailwind,
        title = title,
        body_html = html_body_inner_html
    )
}
