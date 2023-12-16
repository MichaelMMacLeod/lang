use crate::{
    compound::Compound,
    rule::{apply_rule, Rule},
    storage::{Storage, StorageKey, Term},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Env {
    // Each key must point to a Rule
    rules: Vec<Rule>,
}

// (env <rule> ...) -> <env>
pub fn compile_env(storage: &Storage, env: StorageKey) -> Option<Env> {
    match storage.get(env).unwrap() {
        Term::Compound(c) => {
            if let Term::Symbol(s) = storage.get(*c.keys().get(0)?)? {
                if s.data() == "env" {
                    let rules: Vec<Rule> = c.keys()[1..]
                        .iter()
                        .filter_map(|k| match storage.get(*k).unwrap() {
                            Term::Rule(r) => Some(r.clone()),
                            _ => None,
                        })
                        .collect();
                    (rules.len() == c.keys().len()).then(|| Env { rules })
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn apply_matching_rule(
    env: &Env,
    storage: &mut Storage,
    term: StorageKey,
) -> Option<StorageKey> {
    env.rules
        .iter()
        .filter_map(|rule| apply_rule(rule, storage, term))
        .next()
        .or_else(|| match storage.get(term).unwrap() {
            Term::Compound(c) => {
                let c = c.clone();
                let mut success = false;
                let mut new_keys: Vec<StorageKey> = Vec::new();
                for k in c.keys() {
                    if !success {
                        if let Some(result) = apply_matching_rule(env, storage, *k) {
                            success = true;
                            new_keys.push(result);
                        } else {
                            new_keys.push(*k);
                        }
                    } else {
                        new_keys.push(*k);
                    }
                }
                if success {
                    Some(storage.insert(Term::Compound(Compound::new(new_keys))))
                } else {
                    None
                }
            }
            _ => None,
        })
}

#[cfg(test)]
mod test {
    use crate::{parser::read, rule::compile_rule};

    use super::*;

    #[test]
    fn apply_matching_rule1() {
        let mut s = Storage::new();

        let rule1 = {
            let r = read(&mut s, "(for x -> 10)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule2 = {
            let r = read(&mut s, "(for (+ 10 10) -> 20)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule3 = {
            let r = read(&mut s, "(for (+ 20 10) -> 30)").unwrap();
            compile_rule(&mut s, r)
        };

        let env = Env {
            rules: vec![rule1, rule2, rule3],
        };

        let t = read(&mut s, "(+ (+ x x) x)").unwrap();
        s.println(t);
        let t = apply_matching_rule(&env, &mut s, t).unwrap();
        s.println(t);
        let t = apply_matching_rule(&env, &mut s, t).unwrap();
        s.println(t);
        let t = apply_matching_rule(&env, &mut s, t).unwrap();
        s.println(t);
        let t = apply_matching_rule(&env, &mut s, t).unwrap();
        s.println(t);
        let t = apply_matching_rule(&env, &mut s, t).unwrap();
        s.println(t);
    }
}
