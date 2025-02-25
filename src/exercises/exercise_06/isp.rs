pub trait Runner {
    fn run(&self);
}

pub trait Swimmer {
    fn swim(&self);
}

pub trait Cyclist {
    fn cycle(&self);
}

pub trait TeamSport {
    fn play_as_team(&self);
}

/// Triathlete struct
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::isp::*;
///
/// let triathlete = Triathlete;
/// assert_eq!(triathlete.run(), ());
/// assert_eq!(triathlete.swim(), ());
/// assert_eq!(triathlete.cycle(), ());
/// ```
pub struct Triathlete;

impl Runner for Triathlete {
    fn run(&self) {
        println!("Triathlete is running");
    }
}

impl Swimmer for Triathlete {
    fn swim(&self) {
        println!("Triathlete is swimming");
    }
}

impl Cyclist for Triathlete {
    fn cycle(&self) {
        println!("Triathlete is cycling");
    }
}

/// RunnerOnly struct
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::isp::*;
///
/// let runner_only = RunnerOnly;
/// assert_eq!(runner_only.run(), ());
/// ```
pub struct RunnerOnly;

impl Runner for RunnerOnly {
    fn run(&self) {
        println!("Runner is running");
    }
}

/// SoccerPlayer struct
///
/// # Example
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_06::isp::*;
///
/// let soccer_player = SoccerPlayer;
/// assert_eq!(soccer_player.play_as_team(), ());
/// ```
pub struct SoccerPlayer;

impl TeamSport for SoccerPlayer {
    fn play_as_team(&self) {
        println!("Soccer player is playing as part of a team");
    }
}
