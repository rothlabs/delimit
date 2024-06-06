pub mod text;
pub mod html;

#[cfg(test)]
mod tests {
    use super::text::*;

    #[test]
    fn list_empty() {
        let list = list();
        assert_eq!(list.string(), "");
    }
}
