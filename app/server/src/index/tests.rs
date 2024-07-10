use super::*;

const EXPECTED_INDEX: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<title>
Delimit
</title>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="author" content="Roth Labs LLC">
</head>
<body>
Delimit
</body>
</html>"#;

#[test]
fn index_page() {
    assert_eq!(EXPECTED_INDEX, index());
}
