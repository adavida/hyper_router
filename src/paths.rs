#[derive(Debug, PartialEq, Eq)]
pub struct Paths(Vec<PathElement>);

impl Paths {
    pub fn new(entry: &str) -> Paths {
        Paths(entry.split('/').filter_map(PathElement::new).collect())
    }

    pub fn get_data(
        &self,
    ) -> (
        impl Iterator<Item = proc_macro2::TokenTree> + '_,
        impl Iterator<Item = proc_macro2::TokenTree> + '_,
    ) {
        (self.get_vars(), self.get_paths_pattern())
    }

    pub fn get_vars(&self) -> impl Iterator<Item = proc_macro2::TokenTree> + '_ {
        self.0.iter().filter_map(|r| match r {
            value @ PathElement::Var(_) => Some(value.to_token_tree()),
            _ => None,
        })
    }

    pub fn get_paths_pattern(&self) -> impl Iterator<Item = proc_macro2::TokenTree> + '_ {
        self.0.iter().map(|e| e.to_token_tree())
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
            a if a.starts_with('{') && a.ends_with('}') => {
                let len = a.len() - 1;
                Some(PathElement::Var((a[1..len]).to_string()))
            }
            a => Some(PathElement::Const(a.to_string())),
        }
    }

    fn to_token_tree(&self) -> proc_macro2::TokenTree {
        match &self {
            PathElement::Const(str) => {
                proc_macro2::TokenTree::from(proc_macro2::Literal::string(str))
            }
            PathElement::Var(str) => proc_macro2::TokenTree::from(quote::format_ident!("{}", &str)),
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
