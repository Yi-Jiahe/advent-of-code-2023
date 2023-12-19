use regex::Regex;

use std::collections::HashMap;

type MachinePart = [usize; 4];

#[derive(Debug)]
struct WorkflowRule<'a>(char, char, usize, &'a str);

fn parse_input(input: &str) -> (HashMap<&str, (Vec<WorkflowRule>, &str)>, Vec<MachinePart>) {
    let mut input_iterator = input.split("\n").map(|line| line.trim());

    let mut workflows: HashMap<&str, (Vec<WorkflowRule>, &str)> = HashMap::new();

    let workflow_re = Regex::new(r"(\w+)\{(.*),(\w+)\}").unwrap();
    let rule_re = Regex::new(r"(\w+)([><])(\d+):(\w+)").unwrap();

    loop {
        let workflow = input_iterator.next().unwrap();
        if workflow == "" {
            break;
        }

        let (_, [name, rules, dump]) = workflow_re
            .captures(workflow)
            .expect("Unable to parse workflow")
            .extract();
        let mut workflow_rules = Vec::new();
        for rule in rules.split(",") {
            let (_, [rating, operator, value, destination]) = rule_re
                .captures(rule)
                .expect("Unable to parse rule")
                .extract();
            workflow_rules.push(WorkflowRule(
                rating.parse::<char>().unwrap(),
                operator.parse::<char>().unwrap(),
                value.parse::<usize>().unwrap(),
                destination,
            ));
        }
        workflows.insert(name, (workflow_rules, dump));
    }

    let mut parts = Vec::new();

    let part_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    loop {
        if let Some(part) = input_iterator.next() {
            if part == "" {
                break;
            }
            let (_, ratings) = part_re
                .captures(part)
                .expect("Unable to parse part")
                .extract();
            // Hey it spells x-mas!
            let [x, m, a, s] = ratings.map(|r| r.parse::<usize>().unwrap());

            parts.push([x, m, a, s]);
        } else {
            break;
        }
    }

    (workflows, parts)
}

fn run_workflow<'a>(workflow: &'a (Vec<WorkflowRule>, &str), part: &MachinePart) -> &'a str {
    for WorkflowRule(rating, operator, value, destination) in &workflow.0 {
        let i = match rating {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        };
        let matched = match operator {
            '<' => part[i] < *value,
            '>' => part[i] > *value,
            _ => unreachable!(),
        };
        if matched {
            return destination;
        }
    }
    return workflow.1;
}

fn sort_part<'a>(
    part: &MachinePart,
    workflows: &'a HashMap<&str, (Vec<WorkflowRule>, &str)>,
) -> &'a str {
    let mut workflow_name = "in";

    loop {
        let workflow = workflows.get(&workflow_name).unwrap();

        workflow_name = run_workflow(workflow, &part);

        if workflow_name == "A" || workflow_name == "R" {
            return workflow_name;
        }
    }
}

fn sort_parts<'a>(
    workflows: &HashMap<&str, (Vec<WorkflowRule>, &str)>,
    parts: &'a Vec<MachinePart>,
) -> (Vec<&'a MachinePart>, Vec<&'a MachinePart>) {
    let (mut accepted, mut rejected) = (Vec::new(), Vec::new());

    for part in parts {
        match sort_part(part, workflows) {
            "A" => accepted.push(part),
            "R" => rejected.push(part),
            _ => unreachable!(),
        }
    }

    (accepted, rejected)
}

pub fn day_19_sum_accepted_part_ratings(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);

    let (accepted, _) = sort_parts(&workflows, &parts);

    let mut acc = 0;
    for part in accepted {
        for rating in part {
            acc = acc + rating;
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_19_sum_accepted_part_ratings() {
        assert_eq!(
            19114,
            day_19_sum_accepted_part_ratings(
                r#"px{a<2006:qkq,m>2090:A,rfg}
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
  {x=2127,m=1623,a=2188,s=1013}"#
            )
        )
    }
}
