use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LangNVar(usize);

impl From<LangNVar> for usize {
    fn from(value: LangNVar) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LangNPlusOneVar(usize);

impl LangNPlusOneVar {
    pub fn new(v: usize) -> Self {
        Self(v)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Scope(usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Varmap {
    generated_vars: HashSet<LangNVar>,
    source_to_generated_map: HashMap<LangNPlusOneVar, LangNVar>,
    scope_stack: Vec<Vec<(LangNVar, Option<LangNPlusOneVar>)>>,
}

impl Varmap {
    pub fn enter_scope(&mut self) -> Scope {
        let scope = Scope(self.scope_stack.len());
        self.scope_stack.push(vec![]);
        scope
    }

    pub fn exit_scope(&mut self, scope: Scope) {
        self.assert_scope_invariants(&scope);
        let scoped_vars = self.scope_stack.pop().unwrap();
        for (var, np1) in scoped_vars {
            assert!(self.generated_vars.contains(&var));
            self.generated_vars.remove(&var);
            if let Some(np1_var) = np1 {
                assert!(self.source_to_generated_map.contains_key(&np1_var));
                self.source_to_generated_map.remove(&np1_var);
            }
        }
    }

    pub fn generate_var(&mut self, scope: &Scope) -> LangNVar {
        self.generate_var_np1(scope, None)
    }

    pub fn get_source_variable(&mut self, src: LangNPlusOneVar, scope: &Scope) -> LangNVar {
        if self.source_to_generated_map.contains_key(&src) {
            *self.source_to_generated_map.get(&src).unwrap()
        } else {
            self.generate_var_from_source(src, scope)
        }
    }

    fn generate_var_np1(&mut self, scope: &Scope, np1: Option<LangNPlusOneVar>) -> LangNVar {
        self.assert_scope_invariants(&scope);
        let mut var = LangNVar(self.generated_vars.len());
        while self.generated_vars.contains(&var) {
            var.0 += 1;
        }
        self.generated_vars.insert(var);
        self.scope_stack
            .last_mut()
            .expect("attempt to create var outside of top level scope")
            .push((var, np1));
        var
    }

    fn generate_var_from_source(&mut self, src: LangNPlusOneVar, scope: &Scope) -> LangNVar {
        assert!(!self.source_to_generated_map.contains_key(&src));
        let var = self.generate_var_np1(scope, Some(src));
        self.source_to_generated_map.insert(src, var);
        var
    }

    fn assert_scope_invariants(&self, scope: &Scope) {
        assert_eq!(
            self.scope_stack
                .len()
                .checked_sub(1)
                .expect("not inside a scope"),
            scope.0,
            "{}",
            "only the most recent scope may be operated on",
        );
    }
}

impl Default for Varmap {
    fn default() -> Self {
        Self {
            generated_vars: Default::default(),
            source_to_generated_map: Default::default(),
            scope_stack: Vec::default(),
        }
    }
}
