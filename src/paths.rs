#[derive(Debug, PartialEq, Eq)]
pub struct Paths(Vec<PathElement>);

impl Paths {
    pub fn new(entry: &str) -> Paths {
        Paths(entry.split('/').map(PathElement::new).collect())
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
    fn new(entry: &str) -> PathElement {
        match entry.trim() {
            element if element.starts_with('{') && element.ends_with('}') => {
                let len = element.len() - 1;
                PathElement::Var((element[1..len]).to_string())
            }
            element => PathElement::Const(element.to_string()),
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
    use super::*;

    #[test]
    fn result() {
        let px = Paths::new("/main/{var1}/in/{var2}");
        let (vars, paths) = px.get_data();
        assert_eq!(
            vars.map(|x| x.to_string()).collect::<Vec<String>>(),
            vec!["var1".to_string(), "var2".to_string()]
        );
        assert_eq!(
            paths.map(|v| v.to_string()).collect::<Vec<String>>(),
            vec![
                "\"main\"".to_string(),
                "var1".to_string(),
                "\"in\"".to_string(),
                "var2".to_string(),
            ]
        );
    }
}
