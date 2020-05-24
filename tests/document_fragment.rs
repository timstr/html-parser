use html_parser::{Dom, Result};
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_single_div_as_fragment() -> Result<()> {
    let markup = "<div/>";
    let dom = Dom::parse(markup)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_single_text_as_fragment() -> Result<()> {
    let markup = "hello";
    let dom = Dom::parse(markup)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_text_comment_element_as_fragment() -> Result<()> {
    let markup = "hello<!--world?--><div/>";
    let dom = Dom::parse(markup)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_error_when_body_is_used_in_fragment_root() {
    let markup = "<div></div><body></body>";
    assert!(Dom::parse(markup).is_err());
}
#[test]
fn it_error_when_head_is_used_in_fragment_root() {
    let markup = "<div></div><head></head>";
    assert!(Dom::parse(markup).is_err());
}
#[test]
fn it_error_when_html_is_used_in_fragment_root() {
    let markup = "<div></div><html></html>";
    assert!(Dom::parse(markup).is_err());
}
