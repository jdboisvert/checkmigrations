use crate::checkmigrations::utils::lib::has_duplicates;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicates() {
        let input = vec!["0001", "0002", "0003", "0004", "0005"];
        let has_dupes = has_duplicates(&input);
        assert_eq!(has_dupes, false, "There should be no duplicates");
    }

    #[test]
    fn with_duplicates() {
        let input = vec!["0001", "0002", "0002", "0004", "0005"];
        let has_dupes = has_duplicates(&input);
        assert_eq!(has_dupes, true, "There should be duplicates");
    }

    #[test]
    fn empty_input() {
        let input: Vec<i32> = vec![];
        let has_dupes = has_duplicates(&input);
        assert_eq!(has_dupes, false, "An empty input should have no duplicates");
    }

    #[test]
    fn single_element() {
        let input = vec!["0001"];
        let has_dupes = has_duplicates(&input);
        assert_eq!(
            has_dupes, false,
            "A single element input should have no duplicates"
        );
    }
}
