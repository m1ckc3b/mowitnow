#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
enum Orientation {
    N,
    E,
    W,
    S,
}

#[derive(Debug)]
struct Position(Point, Orientation);

#[derive(Debug)]
struct Mower<'a> {
    position: Position,
    commands: &'a str,
    limits: Point,
}

impl<'a> Mower<'a> {
    fn new(limits: Point, position: Position, commands: &'a str) -> Self {
        Self {
            limits,
            position,
            commands,
        }
    }

    fn get_position(&self) -> &Point {
        &self.position.0
    }

    fn get_final_position(&self) -> (isize, isize, &str) {
        let x = self.position.0.x;
        let y = self.position.0.y;
        let orientation = match self.get_orientation() {
            Orientation::N => "N",
            Orientation::E => "E",
            Orientation::W => "W",
            Orientation::S => "S",
        };
        (x, y, orientation)
    }

    fn set_position(&mut self, x: isize, y: isize) {
        self.position.0.x = x;
        self.position.0.y = y;
    }

    fn get_orientation(&self) -> &Orientation {
        &self.position.1
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        self.position.1 = orientation;
    }

    fn mow(&mut self) {
        // split command
        for command in self.commands.chars() {
            match command {
                'G' | 'L' => {
                    // Turn Left
                    let new_position = match self.position.1 {
                        Orientation::N => Orientation::W,
                        Orientation::E => Orientation::N,
                        Orientation::W => Orientation::S,
                        Orientation::S => Orientation::E,
                    };
                    self.set_orientation(new_position);
                }
                'D' | 'R' => {
                    // Turn Right
                    let new_position = match self.position.1 {
                        Orientation::N => Orientation::E,
                        Orientation::E => Orientation::S,
                        Orientation::W => Orientation::N,
                        Orientation::S => Orientation::W,
                    };
                    self.set_orientation(new_position);
                }
                _ => {
                    // Advance
                    let position = self.get_position();
                    match self.position.1 {
                        Orientation::N => self.set_position(position.x, position.y + 1),
                        Orientation::E => self.set_position(position.x + 1, position.y),
                        Orientation::W => self.set_position(position.x - 1, position.y),
                        Orientation::S => self.set_position(position.x, position.y - 1),
                    }
                } 
            }
        }
    }
}

fn main() {
    println!("Mower is mowing....")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_display_13n() {
        let limits = Point { x: 5, y: 5 };
        let point = Point { x: 1, y: 2 };
        let orientation = Orientation::N;
        let position = Position(point, orientation);
        let commands = "GAGAGAGAA";

        let mut mower = Mower::new(limits, position, commands);
        mower.mow();
        let final_position = mower.get_final_position();
        assert_eq!(final_position, (1,3,"N"));
    }

    #[test]
    fn should_display_51e() {
        let limits = Point { x: 5, y: 5 };
        let point = Point { x: 3, y: 3 };
        let orientation = Orientation::E;
        let position = Position(point, orientation);
        let commands = "AADAADADDA";

        let mut mower = Mower::new(limits, position, commands);
        mower.mow();
        let final_position = mower.get_final_position();
        assert_eq!(final_position, (5,1,"E"));
    }

    #[test]
    fn should_display_13n_english_version() {
        let limits = Point { x: 5, y: 5 };
        let point = Point { x: 1, y: 2 };
        let orientation = Orientation::N;
        let position = Position(point, orientation);
        let commands = "LALALALAA";

        let mut mower = Mower::new(limits, position, commands);
        mower.mow();
        let final_position = mower.get_final_position();
        assert_eq!(final_position, (1,3,"N"));
    }

    #[test]
    fn should_display_51e_english_version() {
        let limits = Point { x: 5, y: 5 };
        let point = Point { x: 3, y: 3 };
        let orientation = Orientation::E;
        let position = Position(point, orientation);
        let commands = "AARAARARRA";

        let mut mower = Mower::new(limits, position, commands);
        mower.mow();
        let final_position = mower.get_final_position();
        assert_eq!(final_position, (5,1,"E"));
    }

    #[test]
    fn should_display_13n_mix_fr_en_commands() {
        let limits = Point { x: 5, y: 5 };
        let point = Point { x: 1, y: 2 };
        let orientation = Orientation::N;
        let position = Position(point, orientation);
        let commands = "LAGALAGAA";

        let mut mower = Mower::new(limits, position, commands);
        mower.mow();
        let final_position = mower.get_final_position();
        assert_eq!(final_position, (1,3,"N"));
    }

    #[test]
    fn should_display_51e_mix_fr_en_commands() {
        let limits = Point { x: 5, y: 5 };
        let point = Point { x: 3, y: 3 };
        let orientation = Orientation::E;
        let position = Position(point, orientation);
        let commands = "AADAARADRA";

        let mut mower = Mower::new(limits, position, commands);
        mower.mow();
        let final_position = mower.get_final_position();
        assert_eq!(final_position, (5,1,"E"));
    }
}
