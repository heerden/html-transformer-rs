use crate::models::transform::TransformCase;
use kuchikiki::{parse_html, traits::*};

pub fn transform_p_case(html: String, case: TransformCase) -> String {
    let document = parse_html().one(html);
    for p in document.select("p").unwrap() {
        for descendant in p.as_node().descendants() {
            // 1: Read
            if let Some(text) = descendant.as_text() {
                // 2: Transform
                let case_change = match case {
                    TransformCase::UpperCase => text.borrow().to_uppercase(),
                    TransformCase::LowerCase => text.borrow().to_lowercase(),
                };
                // 3: Replace, all in-place
                *text.borrow_mut() = case_change;
            }
        }
    }

    let binding = document.select_first("body").unwrap();
    let inner_body = binding.as_node();

    inner_body
        .children()
        .map(|child| child.to_string())
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod test_transform {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_transform_case() {
        // Special note, I write out the params as separate variables for readability

        // Test 1 Simple lower case
        let html = "<p>Hello world</p>".to_string();
        let case = TransformCase::LowerCase;
        let result = transform_p_case(html.clone(), case);
        assert_eq!(result, "<p>hello world</p>");

        // Test 2 Simple lower case
        let html = "<p>Hello world</p>".to_string();
        let case = TransformCase::UpperCase;
        let result = transform_p_case(html.clone(), case);
        assert_eq!(result, "<p>HELLO WORLD</p>");

        // Test 3 Multiple paragraph
        let html = "<div><p>First paragraph</p><span>Not a paragraph</span><p>Second
paragraph</p></div>"
            .to_string();
        let case = TransformCase::UpperCase;
        let result = transform_p_case(html.clone(), case);
        assert_eq!(
            result,
            "<div><p>FIRST PARAGRAPH</p><span>Not a paragraph</span><p>SECOND
PARAGRAPH</p></div>"
        );

        // Test 4 Nested elements
        let html =
            "<p>Text with <strong>bold</strong> and <em>italic</em> elements</p>".to_string();
        let case = TransformCase::UpperCase;
        let result = transform_p_case(html.clone(), case);
        assert_eq!(
            result,
            "<p>TEXT WITH <strong>BOLD</strong> AND <em>ITALIC</em> ELEMENTS</p>"
        );

        // Test 5 Mixed test
        let html = "<div><p>First paragraph element</p><span>Not a paragraph</span><p>This is the <strong>Second</strong> listed <em>Paragraph</em> element</p></div".to_string();
        let case = TransformCase::LowerCase;
        let result = transform_p_case(html.clone(), case);
        assert_eq!(
            result,
            "<div><p>first paragraph element</p><span>Not a paragraph</span><p>this is the <strong>second</strong> listed <em>paragraph</em> element</p></div>"
        );
    }
}
