pub trait Robot {
    fn run(&self) {
        println!("Robot is running");
    }
}

pub struct ExRobot {
    pub name: String,
}

impl Robot for ExRobot {
    // Inherits the default implementation of the run method from the Robot trait
    fn run(&self) {
        println!("ExRobot {} is running", self.name);
    }
}

pub fn run_robot<R: Robot>(robot: &R) {
    robot.run();
}
