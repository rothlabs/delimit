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
<script type="importmap">
{"imports":{"init":"/client.js"}}
</script>
</head>
<body>
Delimit
<canvas id="canvas">
</canvas>
<script src="/boot.js" type="module">
</script>
</body>
</html>"#;

// #[test]
// fn index_page() {
//     assert_eq!(EXPECTED_INDEX, index().unwrap());
// }
