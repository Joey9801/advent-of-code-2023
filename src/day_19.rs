use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WfId(usize);

#[derive(Clone, Copy, Debug)]
enum Property {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy, Debug)]
enum Test {
    /// The test that always passes,
    Void,

    /// The given property must be less than the given value to pass
    LessThan {
        property: Property,
        value: i64,
    },

    GreaterThan {
        property: Property,
        value: i64,
    },
}

impl Test {
    fn test(&self, object: &Object) -> bool {
        match self {
            Self::Void => true,
            Self::LessThan { property, value } => object[*property] < *value,
            Self::GreaterThan { property, value } => object[*property] > *value,
        }
    }

    /// Splits the given range into a range that passes this test and a range
    /// that fails this test.
    fn test_range(&self, object_range: ObjectRange) -> (Option<ObjectRange>, Option<ObjectRange>) {
        match self {
            Self::Void => (Some(object_range), None),
            Self::LessThan { property, value } => object_range.split_lt(*property, *value),
            Self::GreaterThan { property, value } => {
                let (b, a) = object_range.split_lt(*property, *value + 1);
                (a, b)
            }
        }
    }
}

impl FromStr for Test {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.is_ascii());

        let property = match &s[0..1] {
            "x" => Property::X,
            "m" => Property::M,
            "a" => Property::A,
            "s" => Property::S,
            other => panic!("Invalid property '{other}'"),
        };

        let value = s[2..].parse().unwrap();

        match &s[1..2] {
            "<" => Ok(Self::LessThan { property, value }),
            ">" => Ok(Self::GreaterThan { property, value }),
            _ => panic!("Invalid test '{s}'"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Destination {
    Reject,
    Accept,
    Workflow(WfId),
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    test: Test,
    destination: Destination,
}

impl Instruction {
    fn parse(s: &str, workflow_ids: &HashMap<String, WfId>) -> Self {
        // Parses strings like:
        //   "x<10:A" - if x < 10, destination accept
        //   "m>100:asdf" - if m > 100, destination workflow "asdf"
        //   "a>100:R" - if a > 100, destination reject
        //   "asdf" - destination workflow "asdf"

        if let Some((test, dest_name)) = s.split_once(':') {
            let destination = match dest_name {
                "R" => Destination::Reject,
                "A" => Destination::Accept,
                _ => Destination::Workflow(workflow_ids[dest_name]),
            };
            let test = test.parse().unwrap();
            Self { test, destination }
        } else {
            let destination = match s {
                "R" => Destination::Reject,
                "A" => Destination::Accept,
                _ => Destination::Workflow(workflow_ids[s]),
            };
            Self {
                test: Test::Void,
                destination,
            }
        }
    }
}

#[derive(Debug)]
struct Workflow(Vec<Instruction>);

impl Workflow {
    fn destination(&self, object: &Object) -> Destination {
        for instruction in &self.0 {
            if instruction.test.test(object) {
                return instruction.destination;
            }
        }
        panic!("No destination found for object {:?}", object);
    }

    fn range_destinations(
        &self,
        object_range: ObjectRange,
    ) -> impl Iterator<Item = (Destination, ObjectRange)> + '_ {
        self.0
            .iter()
            .scan(Some(object_range), |object_range, instruction| {
                if let Some(r) = object_range {
                    let (pass, fail) = instruction.test.test_range(*r);
                    *object_range = fail;
                    Some(pass.map(|r| (instruction.destination, r)))
                } else {
                    None
                }
            })
            .flatten()
    }
}

#[derive(Clone, Copy, Debug)]
struct Object {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Object {
    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Object {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parses strings like "{x=787,m=2655,a=1222,s=2876}"

        let mut object = Object {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        let parts = s.trim_start_matches('{').trim_end_matches('}').split(',');

        for part in parts {
            let (property, value) = part.split_once('=').unwrap();
            let value = value.parse().unwrap();
            match property {
                "x" => object.x = value,
                "m" => object.m = value,
                "a" => object.a = value,
                "s" => object.s = value,
                _ => panic!("Invalid property '{property}'"),
            }
        }

        Ok(object)
    }
}

impl Index<Property> for Object {
    type Output = i64;

    fn index(&self, property: Property) -> &Self::Output {
        match property {
            Property::X => &self.x,
            Property::M => &self.m,
            Property::A => &self.a,
            Property::S => &self.s,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ObjectRange {
    // Inclusive bounds
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl Index<Property> for ObjectRange {
    type Output = (i64, i64);

    fn index(&self, property: Property) -> &Self::Output {
        match property {
            Property::X => &self.x,
            Property::M => &self.m,
            Property::A => &self.a,
            Property::S => &self.s,
        }
    }
}

impl IndexMut<Property> for ObjectRange {
    fn index_mut(&mut self, property: Property) -> &mut Self::Output {
        match property {
            Property::X => &mut self.x,
            Property::M => &mut self.m,
            Property::A => &mut self.a,
            Property::S => &mut self.s,
        }
    }
}

impl ObjectRange {
    /// The total number of distinct objects in this range.
    fn len(&self) -> i64 {
        let mut len = 1;
        for property in &[Property::X, Property::M, Property::A, Property::S] {
            let (lower, upper) = self[*property];
            len *= upper - lower + 1;
        }
        len
    }

    /// Splits this range into two, one with the given property less than the
    /// given value, and one with the given property greater than or equal to
    /// the given value.
    fn split_lt(&self, property: Property, value: i64) -> (Option<Self>, Option<Self>) {
        let split_bounds = |(lower, upper), value| {
            if value <= lower {
                (None, Some((lower, upper)))
            } else if value > upper {
                (Some((lower, upper)), None)
            } else {
                (Some((lower, value - 1)), Some((value, upper)))
            }
        };

        let (a, b) = split_bounds(self[property], value);
        let a = a.map(|bounds| {
            let mut range = self.clone();
            range[property] = bounds;
            range
        });

        let b = b.map(|bounds| {
            let mut range = self.clone();
            range[property] = bounds;
            range
        });

        (a, b)
    }
}

#[derive(Debug)]
pub struct Input {
    start_workflow: WfId,
    workflows: Vec<Workflow>,
    objects: Vec<Object>,
}

impl Input {
    fn final_destination(&self, object: Object) -> Destination {
        let mut wf = self.start_workflow;
        loop {
            match self.workflows[wf.0].destination(&object) {
                Destination::Reject => return Destination::Reject,
                Destination::Accept => return Destination::Accept,
                Destination::Workflow(next_wf) => wf = next_wf,
            }
        }
    }

    fn range_destinations(&self, object_range: ObjectRange) -> Vec<ObjectRange> {
        let mut stack = vec![(self.start_workflow, object_range)];
        let mut accepted = Vec::new();

        while let Some((wf, object_range)) = stack.pop() {
            for (destinationm, object_range) in
                self.workflows[wf.0].range_destinations(object_range)
            {
                match destinationm {
                    Destination::Reject => {}
                    Destination::Accept => accepted.push(object_range),
                    Destination::Workflow(next_wf) => stack.push((next_wf, object_range)),
                }
            }
        }

        accepted
    }
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

pub fn parse(input: &str) -> Input {
    let (workflows, objects) = input.split_once("\n\n").unwrap();

    let mut workflow_ids = HashMap::new();
    for (i, line) in workflows.lines().enumerate() {
        let (name, _) = line.split_once('{').unwrap();
        workflow_ids.insert(name.to_owned(), WfId(i));
    }

    let start_workflow = workflow_ids["in"];

    let workflows = workflows
        .lines()
        .map(|line| {
            let (_, instructions) = line.split_once('{').unwrap();
            let instructions = instructions
                .trim_end_matches('}')
                .split(',')
                .map(|s| Instruction::parse(s, &workflow_ids))
                .collect();
            Workflow(instructions)
        })
        .collect();

    let objects = objects.lines().map(|line| line.parse().unwrap()).collect();

    Input {
        start_workflow,
        workflows,
        objects,
    }
}

pub fn solve_part_1(input: &Input) -> i64 {
    let mut sum = 0;
    for object in &input.objects {
        if input.final_destination(*object) == Destination::Accept {
            sum += object.sum();
        }
    }
    sum
}

pub fn solve_part_2(input: &Input) -> i64 {
    let range = ObjectRange {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    input
        .range_destinations(range)
        .iter()
        .map(|r| r.len())
        .sum()
}
