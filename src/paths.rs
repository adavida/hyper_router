#[derive(Debug, PartialEq, Eq)]
struct Paths(Vec<PathElement>);

impl Paths {
    fn new(entry: &str) -> Paths {
        Paths(entry.split('/').filter_map(PathElement::new).collect())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PathElement {
    Const(String),
    Var(String),
}

impl PathElement {
    fn new(entry: &str) -> Option<PathElement> {
        match entry.trim() {
            "" => None,
            a if a.starts_with("{") && a.ends_with("}") => {
                let len = a.len() - 1;
                Some(PathElement::Var((&a[1..len]).to_string()))
            }
            a => Some(PathElement::Const(a.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::paths::*;

    #[test]
    fn base_test() {
        let result = Paths::new("/main/{var}/in/{var2}");
        assert_eq!(
            result,
            Paths(vec!(
                PathElement::Const("main".to_string()),
                PathElement::Var("var".to_string()),
                PathElement::Const("in".to_string()),
                PathElement::Var("var2".to_string()),
            )),
        );
    }
}
