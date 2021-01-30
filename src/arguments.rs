use clap::{App, arg_enum, clap_app};

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Types {
        Bug,
        Feature,
        Improvement,
        Task,
    }
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Priority {
        Low,
        Medium,
        High,
    }
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Status {
        Open,
        Closed,
        InProgress,
        Review,
        Rejected,
        Halted,
    }
}

pub fn init<'a, 'b>() -> App<'a, 'b> {
    clap_app!(init =>
        (about: "Initializes issue trackers meta files")
    )
}

pub fn list<'a, 'b>() -> App<'a, 'b> {
    clap_app!(list =>
        (about: "Displays issues")
        (@arg hash: -h --hash +takes_value "Filter listing by issue hash")
        (@arg type: -t --type +takes_value possible_values(&Types::variants()) "Filter listing by issue type")
        (@arg parent: -p --parent +takes_value "Filter listing by parent issue hash")
        (@arg author: -a --author +takes_value "Filter listing by issue author")
        (@arg from_creation: -c --from_creation +takes_value "Filter listing from given issue creation date")
        (@arg to_creation: -C --to_creation +takes_value "Filter listing to given issue creation date")
        (@arg title: -T --title +takes_value "Filter listing by issue title regex")
        (@arg description: -d --description +takes_value "Filter listing by issue description regex")
        (@arg priority: -i --priority +takes_value possible_values(&Priority::variants()) "Filter listing by issue priority")
        (@arg status: -s --status +takes_value possible_values(&Status::variants()) "Filter listing by issue status")
        (@arg assigned_to: -r --assigned +takes_value "Filter listing by issue assigned to regex")
        (@arg from_start: -b --from_start +takes_value "Filter listing from given issue start date")
        (@arg to_start: -B --to_start +takes_value "Filter listing to given issue start date")
        (@arg from_due: -e --from_due +takes_value "Filter listing from given due date")
        (@arg to_due: -E --to_due +takes_value "Filter listing to given due date")
        (@arg from_progress: -g --from_progress +takes_value "Filter listing from given issue progress")
        (@arg to_progress: -G --to_progress +takes_value "Filter listing to given issue progress")
    )
}

pub fn open<'a, 'b>() -> App<'a, 'b> {
    clap_app!(open =>
        (about: "Opens an issues")
        (@arg type: -t --type +takes_value possible_values(&Types::variants()) "Set issue type. Default is bug")
        (@arg parent: -p --parent +takes_value "Set issue parent hash")
        (@arg author: -a --author +takes_value "Set issue author. Default is the configured/current user")
        (@arg description_edit: -D --description_edit "Open editor for issue description editing")
        (@arg priority: -i --priority +takes_value possible_values(&Priority::variants()) "Set issue priority. Default is medium")
        (@arg status: -s --status +takes_value possible_values(&Status::variants()) "Set issue status. Default is open")
        (@arg assigned_to: -r --assigned +takes_value "Assign issue to given user")
        (@arg start_date: -b --start_date +takes_value "Set issue start date. Default is the current date time")
        (@arg due_date: -e --due_date +takes_value "Set issue due date")
        (@arg progress: -g --progress +takes_value "Set issue progress")
        (@arg title: +required "Set title of the issue")
    )
}

pub fn edit<'a, 'b>() -> App<'a, 'b> {
    clap_app!(edit =>
        (about: "Edit an existing issues")
        (@arg type: -t --type +takes_value possible_values(&Types::variants()) "Set issue type")
        (@arg parent: -p --parent +takes_value "Set issue parent hash")
        (@arg author: -a --author +takes_value "Set issue author. Default is the configured/current user")
        (@arg title: -T --title +takes_value "Set title of the issue")
        (@arg description_edit: -D --description_edit "Open editor for issue description editing")
        (@arg priority: -i --priority +takes_value possible_values(&Priority::variants()) "Set issue priority")
        (@arg status: -s --status +takes_value possible_values(&Status::variants()) "Set issue status")
        (@arg assigned_to: -r --assigned +takes_value "Assign issue to given user")
        (@arg start_date: -b --start_date +takes_value "Set issue start date. Default is the current date time")
        (@arg due_date: -e --due_date +takes_value "Set issue due date")
        (@arg progress: -g --progress +takes_value "Set issue progress")
        (@arg hash: +required "Hash of the issue to be edited")
    )
}

pub fn close<'a, 'b>() -> App<'a, 'b> {
    clap_app!(close =>
        (about: "Close an existing issue")
        (@arg hash: +required "Hash of the issue to be closed")
    )
}

pub fn remove<'a, 'b>() -> App<'a, 'b> {
    clap_app!(remove =>
        (about: "Remove an existing issue")
        (@arg hash: +required "Hash of the issue to be removed")
    )
}

pub fn config<'a, 'b>() -> App<'a, 'b> {
    clap_app!(config =>
        (about: "Show bugtracker configuration")
    )
}

// TODO: implement custom validator for optional hashes (parent)
// TODO: implement custom validator for optional description
// TODO: implement custom validator for optional string (assigned to)
// TODO: implement custom validator for optional date (due date)
// TODO: implement multiple argument
