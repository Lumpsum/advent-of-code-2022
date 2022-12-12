use std::{fmt::Display, str::FromStr};

use crate::solution::Solution;

pub struct DayFive {}

#[derive(Clone)]
pub struct ContainerStacks([Stack; 9]);

impl Display for ContainerStacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stack in &self.0 {
            writeln!(f, "{}", stack).ok();
        }
        Ok(())
    }
}

impl ContainerStacks {
    fn move_containers(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.container_amount {
            if let Some(container) = self.0[instruction.from_stack].0.pop() {
                self.0[instruction.to_stack].0.push(container);
            }
        }
    }

    fn improved_move_containers(&mut self, instruction: &Instruction) {
        let mut tmp_vec_containers: Vec<Container> = Vec::new();
        for _ in 0..instruction.container_amount {
            if let Some(container) = self.0[instruction.from_stack].0.pop() {
                tmp_vec_containers.push(container);
            }
        }

        tmp_vec_containers.reverse();
        self.0[instruction.to_stack].0.extend(tmp_vec_containers);
    }

    fn print_top_containers(&self) -> Option<String> {
        let mut s = String::new();
        for stack in &self.0 {
            if let Some(container) = stack.0.last() {
                s.push(container.0);
            }
        }
        Some(s)
    }
}

#[derive(Clone)]
struct Stack(Vec<Container>);

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for container in &self.0 {
            writeln!(f, "{}", container.0).ok();
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Container(char);

impl Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", &self.0)
    }
}

pub struct Instructions(Vec<Instruction>);

#[derive(Debug)]
struct Instruction {
    container_amount: usize,
    from_stack: usize,
    to_stack: usize
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_string = s.split_whitespace().collect::<Vec<&str>>();
        Ok(Instruction {
            container_amount: str::parse::<usize>(split_string[1]).unwrap(),
            from_stack: str::parse::<usize>(split_string[3]).unwrap() - 1,
            to_stack: str::parse::<usize>(split_string[5]).unwrap() - 1
        })
    }
}


impl Solution for DayFive {
    type Input<'a> = (ContainerStacks, Instructions);
    type Output = String;

    fn preprocess<'a>(data: &'a str) -> Self::Input<'a> {
        let data_lines = data.lines();
        let mut container_stack = ContainerStacks([
            Stack(Vec::new()),
            Stack(Vec::new()),
            Stack(Vec::new()),
            Stack(Vec::new()),
            Stack(Vec::new()),
            Stack(Vec::new()),
            Stack(Vec::new()),
            Stack(Vec::new()),
            Stack(Vec::new()),
        ]);

        let mut instructions = Instructions(Vec::new());

        for line in data_lines.rev().enumerate() {
            if line.0 >= 503 {
                for container in line.1.chars().collect::<Vec<char>>()[1..].into_iter().step_by(4).enumerate() {
                    if container.1 != &' ' {
                        container_stack.0[container.0].0.push(Container(*container.1));
                    }
                }
            } else if line.0 <= 500 {
                instructions.0.push(Instruction::from_str(line.1).unwrap());
            }
        }

        instructions.0.reverse();

        (container_stack, instructions)
    }

    fn puzzle_one<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        let mut container_stacks = input.0.clone();
        for instruction in input.1.0.iter() {
            container_stacks.move_containers(instruction)
        }
        container_stacks.print_top_containers()
    }

    fn puzzle_two<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        let mut container_stacks = input.0.clone();
        for instruction in input.1.0.iter() {
            container_stacks.improved_move_containers(instruction)
        }
        container_stacks.print_top_containers()
    }
}
