//! This module implements an extremely naive string-based component framework.
//! As a general rule, components should sanitize all inputs lazily, just
//! before they are interpolated into an HTML string, with very few exceptions:
//!
//! - Any `children` prop will not be sanitized
//! - The return value of any other component will be presumed to be safe

use super::models::{Sanitize, User};
use ammonia::clean;

pub struct CreatePageProps<'a> {
    /// Dangerous inner html!
    title: &'a str,
    children: &'a str,
}

impl CreatePageProps<'_> {
    pub fn new<'a>(title: &'a str, children: &'a str) -> CreatePageProps<'a> {
        CreatePageProps { title, children }
    }
}

pub fn create_page(props: CreatePageProps) -> String {
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
        title = props.title,
        body_html = props.children
    )
}

pub fn unauthenticated_actions() -> &'static str {
    r##"
        <button
            hx-get="/login" hx-swap="outerHTML"
            class="bg-blue-100 rounded p-2 m-2 w-24"
        >Login</button>
        <button
            hx-get="/register" hx-swap="outerHTML"
            hx-target="#auth-widget"
            class="bg-green-100 rounded p-2 m-2 w-24"
        >Register</button>
    "##
}

pub struct HxFormProps {
    pub children: String,
    pub hx_post: &'static str,
    pub hx_target: &'static str
}

pub fn hx_form(props: HxFormProps) -> String {
    format!(
        r#"
        <form
            class="flex flex-col"
            hx-post={hx_post}
            hx-target={hx_target}
        >{children}
            <button>Submit</button>
        </form>
        "#,
        children = props.children,
        hx_post = clean(props.hx_post),
        hx_target = clean(props.hx_target)
    )
}

pub struct FormFieldProps {
    pub input_type: String,
    pub name: String,
    pub label_text: String,
}
pub fn form_field(props: FormFieldProps) -> String {
    format!(
    r#"
        <label>{label_text}</label>
        <input class="rounded" type="{type}" name="{name}" />
    "#,
    label_text = clean(&props.label_text),
    name = clean(&props.name),
    type = props.input_type
    )
}

pub struct AuthWidgetProps {
    pub user: Option<User>,
}
pub fn auth_widget(props: AuthWidgetProps) -> String {
    let greeting = if let Some(ref u) = props.user {
        format!("<p>Hi, {}</p>", u.username.sanitize())
    } else {
        "".to_string()
    };
    let auth_buttons = if props.user.is_some() {
        r##"<button
            class="bg-blue-100 rounded p-2 m-2 w-24"
            hx-post="/logout"
            hx-target="#auth-widget"
            >Log Out</button>"##
    } else {
        unauthenticated_actions()
    };
    format!(
        r#"
            <div class="flex flex-col" id="auth-widget">
                {greeting}
                {auth_buttons}
            </div>
        "#,
        greeting = greeting
    )
}
