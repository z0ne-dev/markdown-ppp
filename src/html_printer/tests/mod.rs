#![cfg(test)]
use rstest::rstest;

#[rstest]
#[case("Hello, world!", "<p>Hello, world!</p>")]
#[case("Hello, **world**!", "<p>Hello, <b>world</b>!</p>")]
#[case("Hello, *world*!", "<p>Hello, <em>world</em>!</p>")]
#[case("Hello, __world__!", "<p>Hello, <b>world</b>!</p>")]
#[case("Hello, _world_!", "<p>Hello, <em>world</em>!</p>")]
#[case("Hello, ~~world~~!", "<p>Hello, <s>world</s>!</p>")]
#[case(
    "1. Item 1\n2. Item 2",
    "<ol start=\"1\"><li><p>Item 1</p></li><li><p>Item 2</p></li></ol>"
)]
#[case(
    "* Item 1\n* Item 2",
    "<ul class=\"markdown-list-kind-star\"><li><p>Item 1</p></li><li><p>Item 2</p></li></ul>"
)]
#[case("`code`", "<p><code>code</code></p>")]
#[case("```rust\nfn main() {}\n```", "<pre><code>fn main() {}</code></pre>")]
fn render_to_html(#[case] input: &str, #[case] expected: &str) {
    let config = crate::html_printer::config::Config::default();
    let ast = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();
    println!("{:?} => {:#?}", input, ast);
    let result = crate::html_printer::render_html(&ast, config);
    assert_eq!(expected, result);
}
