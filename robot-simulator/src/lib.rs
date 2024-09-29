// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone)]
// is an enum the most appropriate data structure to use here?
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x,
            y,
            d
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        match self.d {
            Direction::North => self.d = Direction::East,
            Direction::East => self.d = Direction::South,
            Direction::South => self.d = Direction::West,
            Direction::West => self.d = Direction::North
        }
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        match self.d {
            Direction::North => self.d = Direction::West,
            Direction::East => self.d = Direction::North,
            Direction::South => self.d = Direction::East,
            Direction::West => self.d = Direction::South
        }
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.d {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1
        }
        self
    }

    // can still optimise this. do we need the to use robot or can we use mut self, self or &self somehow?
    // should we be cloning here? this seems memory inefficient
    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        instructions.chars().for_each(|c| {
            match c {
                'R' => robot = robot.clone().turn_right(),
                'L' => robot = robot.clone().turn_left(),
                _ => robot = robot.clone().advance()
            };
        });
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
