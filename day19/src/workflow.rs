use anyhow::{anyhow, Context, Error, Result};
use std::{collections::HashMap, str::FromStr};

use crate::part::Part;

type Range = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparison {
    Less,
    Greater,
}

impl FromStr for Comparison {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Self::Less),
            ">" => Ok(Self::Greater),
            _ => Err(anyhow!("Unknown comparison {s}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleAction {
    Accept,
    Reject,
    Move(String),
}

impl FromStr for RuleAction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::Move(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleApply {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    apply_to: RuleApply,
    comp: Comparison,
    value: usize,
    action: RuleAction,
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        let to_compare = match self.apply_to {
            RuleApply::X => part.x,
            RuleApply::M => part.m,
            RuleApply::A => part.a,
            RuleApply::S => part.s,
        };

        match self.comp {
            Comparison::Less => to_compare < self.value,
            Comparison::Greater => to_compare > self.value,
        }
    }

    fn split_ranges(
        &self,
        x: &mut Range,
        m: &mut Range,
        a: &mut Range,
        s: &mut Range,
    ) -> (Range, Range, Range, Range) {
        let (mut new_x, mut new_m, mut new_a, mut new_s) = (*x, *m, *a, *s);
        match self.apply_to {
            RuleApply::X => new_x = self.split_helper(x),
            RuleApply::M => new_m = self.split_helper(m),
            RuleApply::A => new_a = self.split_helper(a),
            RuleApply::S => new_s = self.split_helper(s),
        }
        (new_x, new_m, new_a, new_s)
    }

    fn split_helper(&self, range: &mut Range) -> Range {
        let send: Range;
        match self.comp {
            Comparison::Less => {
                send = (range.0, self.value - 1);
                *range = (self.value, range.1);
            }
            Comparison::Greater => {
                send = (self.value + 1, range.1);
                *range = (range.0, self.value);
            }
        }
        send
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((cond, action)) = s.split_once(':') else {
            return Err(anyhow!(
                "Empty rules must be represented as workflow fallbacks"
            ));
        };
        if cond.len() < 3 {
            return Err(anyhow!("Invalid condition {cond}"));
        }
        let action = RuleAction::from_str(action)?;
        let comp = Comparison::from_str(&cond[1..2])?;
        let value = usize::from_str(&cond[2..])?;
        let apply_to = match &cond[0..1] {
            "x" => RuleApply::X,
            "m" => RuleApply::M,
            "a" => RuleApply::A,
            "s" => RuleApply::S,
            cat => return Err(anyhow!("Unknown category for rule condition: {cat}")),
        };
        Ok(Self {
            apply_to,
            comp,
            value,
            action,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow {
    pub name: String,
    rules: Vec<Rule>,
    fallback: RuleAction,
}

impl Workflow {
    pub fn apply(&self, part: &Part) -> RuleAction {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.action.clone();
            }
        }
        self.fallback.clone()
    }

    pub fn eval_range(
        &self,
        workflows: &HashMap<String, Self>,
        mut x: Range,
        mut m: Range,
        mut a: Range,
        mut s: Range,
    ) -> usize {
        let mut combos = 0;

        for rule in &self.rules {
            let (send_x, send_m, send_a, send_s) =
                rule.split_ranges(&mut x, &mut m, &mut a, &mut s);
            match rule.action {
                RuleAction::Accept => {
                    let nc = (send_x.1 - send_x.0 + 1)
                        * (send_m.1 - send_m.0 + 1)
                        * (send_a.1 - send_a.0 + 1)
                        * (send_s.1 - send_s.0 + 1);
                    // println!("\tAdding {nc}");
                    combos += nc;
                }
                RuleAction::Reject => (),
                RuleAction::Move(ref name) => {
                    combos += workflows
                        .get(name)
                        .unwrap()
                        .eval_range(workflows, send_x, send_m, send_a, send_s)
                }
            }
        }
        match self.fallback {
            RuleAction::Accept => {
                let nc = (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1);
                combos += nc
            }
            RuleAction::Reject => (),
            RuleAction::Move(ref name) => {
                combos += workflows
                    .get(name)
                    .unwrap()
                    .eval_range(workflows, x, m, a, s)
            }
        }
        combos
    }
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((name, rules)) = s.trim_end_matches('}').split_once('{') else {
            return Err(anyhow!("Couldn't find name in workflow: {s}"));
        };
        let fallback = RuleAction::from_str(
            rules
                .split(',')
                .last()
                .context("Cannot create an empty workflow")?,
        )?;
        let rules = rules
            .split(',')
            .take_while(|r| r.contains(':'))
            .map(Rule::from_str)
            .collect::<Result<Vec<Rule>, Error>>()?;

        Ok(Self {
            name: name.to_owned(),
            rules,
            fallback,
        })
    }
}
