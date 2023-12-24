use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Result};
use part::Part;
use workflow::{RuleAction, Workflow};

mod part;
mod workflow;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    println!("Part one: {}", part_one(input)?);
    println!("Part two: {}", part_two(input)?);

    Ok(())
}

fn part_one(input: &str) -> Result<usize> {
    let (wkflw, parts) = input
        .split_once("\n\n")
        .context("Missing divider for workflows and parts")?;
    let workflows = wkflw
        .lines()
        .map(Workflow::from_str)
        .collect::<Result<Vec<Workflow>>>()?;
    let starter = workflows.iter().find(|w| &w.name == "in").unwrap();

    let parts = parts
        .lines()
        .map(Part::from_str)
        .collect::<Result<Vec<Part>>>()?;

    Ok(parts
        .into_iter()
        .filter(|p| execute(p, &workflows, starter))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum())
}

fn part_two(input: &str) -> Result<usize> {
    let (wkflw, _parts) = input
        .split_once("\n\n")
        .context("Missing divider for workflows and parts")?;
    let mut workflow_map = HashMap::new();
    for l in wkflw.lines() {
        let workflow = Workflow::from_str(l)?;
        workflow_map.insert(workflow.name.clone(), workflow);
    }

    let starter = workflow_map.get("in").unwrap();
    let range = (1, 4000);

    Ok(starter.eval_range(&workflow_map, range, range, range, range))
}

fn execute(part: &Part, workflows: &[Workflow], starter: &Workflow) -> bool {
    let mut action = starter.apply(part);
    // print!("{} -> ", starter.name);
    while let RuleAction::Move(ref wkfw) = action {
        // print!("{wkfw} -> ");
        let next = workflows
            .iter()
            .find(|w| &w.name == wkfw)
            .expect("Couldn't find workflow {wkfw}");
        action = next.apply(part);
    }
    // println!("{action:?}");
    action == RuleAction::Accept
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(INPUT).unwrap(), 19114);
    }
    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(INPUT).unwrap(), 167409079868000);
    }
}
