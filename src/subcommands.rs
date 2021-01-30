use std::env;
use std::io::{stdin, stdout, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

use chrono::{Local, TimeZone};
use clap::ArgMatches;

use super::display;
use super::{
    Issue, IssueFilter, IssueHash, IssuePriority, IssueStatus, IssueStorage, IssueType,
    IssuesHandler,
};

pub fn init(_args: &ArgMatches, working_dir: &PathBuf) {
    match IssueStorage::init(&working_dir) {
        Err(e) => {
            eprintln!("Error initializing issue tracker: {}", e);
            process::exit(-1);
        }
        Ok(_) => process::exit(0),
    }
}

pub fn list(args: &ArgMatches, working_dir: &PathBuf) {
    let (_, handler) = load_issues(working_dir);

    // Filter
    let mut filter = IssueFilter::new();
    // hash
    if let Some(v) = hash_parser(args) {
        filter.set_hash_match(v);
    };
    // type
    if let Some(v) = type_parser(args) {
        filter.set_type_match(v);
    };
    // parent
    if let Some(v) = args.value_of("parent") {
        let hash = match IssueHash::from_str(v) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error parsing issue parent hash: {}, {}", v, e);
                process::exit(-1);
            }
        };
        filter.set_parent_match(hash);
    }
    // author
    if let Some(v) = args.value_of("author") {
        if let Err(e) = filter.set_autor_match(v) {
            eprintln!("Error setting author matching pattern: {}, {}", v, e);
            process::exit(-1);
        }
    }
    // from creation date
    if let Some(v) = args.value_of("from_creation") {
        let ts = datetime_parser(v);
        filter.set_creation_date_from_match(ts);
    }
    // to creation date
    if let Some(v) = args.value_of("to_creation") {
        let ts = datetime_parser(v);
        filter.set_creation_date_to_match(ts);
    }
    // title
    if let Some(v) = args.value_of("title") {
        if let Err(e) = filter.set_title_match(v) {
            eprintln!("Error setting title matching pattern: {}, {}", v, e);
            process::exit(-1);
        }
    }
    // description
    if let Some(v) = args.value_of("description") {
        let pattern = v.to_string();
        if let Err(e) = filter.set_description_match(&pattern) {
            eprintln!("Error setting description matching pattern: {}, {}", v, e);
            process::exit(-1);
        }
    }
    // priority
    if let Some(v) = priority_parser(args) {
        filter.set_priority_match(v);
    };
    // status
    if let Some(v) = status_parser(args) {
        filter.set_status_match(v);
    };
    // assigned to
    if let Some(v) = args.value_of("assigned_to") {
        if let Err(e) = filter.set_assigned_to_match(v) {
            eprintln!("Error setting assigned to matching pattern: {}, {}", v, e);
            process::exit(-1);
        }
    };
    // from start date
    if let Some(v) = args.value_of("from_start") {
        let ts = datetime_parser(v);
        filter.set_start_date_from_match(ts);
    }
    // to start date
    if let Some(v) = args.value_of("to_start") {
        let ts = datetime_parser(v);
        filter.set_start_date_to_match(ts);
    }
    // from due date
    if let Some(v) = args.value_of("from_due") {
        let ts = datetime_parser(v);
        filter.set_due_date_from_match(ts);
    }
    // to due date
    if let Some(v) = args.value_of("to_due") {
        let ts = datetime_parser(v);
        filter.set_due_date_to_match(ts);
    }
    // from progress
    if let Some(v) = args.value_of("from_progress") {
        let progress = match v.parse::<u8>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error parsing given progress as integer: {}, {}", v, e);
                process::exit(-1);
            }
        };
        if let Err(e) = filter.set_progress_from_match(progress) {
            eprintln!("Error setting lower progress limit: {}", e);
            process::exit(-1);
        }
    };
    // to progress
    if let Some(v) = args.value_of("to_progress") {
        let progress = match v.parse::<u8>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error parsing given progress as integer: {}, {}", v, e);
                process::exit(-1);
            }
        };
        if let Err(e) = filter.set_progress_to_match(progress) {
            eprintln!("Error setting upper progress limit: {}", e);
            process::exit(-1);
        }
    };

    let issues = handler.issues_filtered(&filter);

    if issues.len() == 1 {
        println!("Found 1 issue");
    } else {
        println!("Found {} issues", issues.len());
    }
    for (_, issue) in issues {
        display::issue_long(issue);
    }
}

pub fn open(args: &ArgMatches, working_dir: &PathBuf) {
    let (storage, mut handler) = load_issues(working_dir);

    // set issue author
    let author = if let Some(v) = args.value_of("author") {
        v.to_owned()
    } else if let Some(ref v) = *storage.config().user() {
        v.to_owned()
    } else {
        match user::get_user_name() {
            Ok(user) => user,
            Err(e) => {
                println!("Error retrieving user: {}", e);
                process::exit(-1);
            }
        }
    };

    // set issue title
    let title = match args.value_of("title") {
        Some(v) => v,
        None => {
            eprintln!("No issue title provided");
            process::exit(-1);
        }
    };
    let mut issue = Issue::new(&author, title);

    // set issue type
    issue_type_parser(&mut issue, args);
    // set issue parent
    issue_parent_parser(&mut issue, args, &storage);
    // set issue priority
    issue_priority_parser(&mut issue, args);
    // set issue status
    issue_status_parser(&mut issue, args);
    // set assigned to
    issue_assigned_to_parser(&mut issue, args);
    // set issue start date
    issue_start_date_parser(&mut issue, args);
    // set issue due date, date format
    issue_due_date_parser(&mut issue, args);
    // set issue progress
    issue_progress_parser(&mut issue, args);
    // set issue description
    issue_description_edit_parser(&mut issue, args);

    handler.insert_issue(issue);
    store_issues(&handler, &storage);
}

pub fn edit(args: &ArgMatches, working_dir: &PathBuf) {
    let (storage, mut handler) = load_issues(working_dir);

    let mut issue = issue_hash_parser(&handler, args);

    // set issue author
    if let Some(v) = args.value_of("author") {
        issue.set_author(v.to_string());
    }

    // set issue title
    if let Some(v) = args.value_of("title") {
        issue.set_title(v.to_string());
    }

    // set issue type
    issue_type_parser(&mut issue, args);
    // set issue parent
    issue_parent_parser(&mut issue, args, &storage);
    // set issue priority
    issue_priority_parser(&mut issue, args);
    // set issue status
    issue_status_parser(&mut issue, args);
    // set assigned to
    issue_assigned_to_parser(&mut issue, args);
    // set issue start date
    issue_start_date_parser(&mut issue, args);
    // set issue due date, date format
    issue_due_date_parser(&mut issue, args);
    // set issue progress
    issue_progress_parser(&mut issue, args);
    // set issue description
    issue_description_edit_parser(&mut issue, args);

    handler.insert_issue(issue);
    store_issues(&handler, &storage);
}

pub fn close(args: &ArgMatches, working_dir: &PathBuf) {
    let (storage, mut handler) = load_issues(working_dir);

    let mut issue = issue_hash_parser(&handler, args);

    issue.set_status(IssueStatus::Closed);
    let _ = issue.set_progress(100);

    handler.insert_issue(issue);
    store_issues(&handler, &storage);
}

pub fn remove(args: &ArgMatches, working_dir: &PathBuf) {
    let (storage, handler) = load_issues(working_dir);

    let hash = hash_parser(args).unwrap();

    let dependencies = handler.find_dependend_issues(&hash);

    if dependencies.len() == 1 {
        println!("Following issue is about to be deleted:");
    } else {
        println!("Following issues are about to be deleted:");
    }
    for hash in &dependencies {
        let issue = handler.issue(hash).unwrap();
        display::issue_short(issue);
    }

    loop {
        let mut buf = String::new();
        print!("Continue? (y/[n]) ");
        let _ = stdout().flush();
        stdin().read_line(&mut buf).unwrap();
        buf = buf.trim().to_lowercase();
        match buf.as_ref() {
            "y" => break,
            "n" | "" => {
                println!("Aborted");
                process::exit(0);
            }
            _ => continue,
        }
    }

    // use reverse iterator so children are deleted before parent which helps
    // preventing children without parent on error
    let dep_iter = dependencies.iter();
    for hash in dep_iter.rev() {
        match storage.remove_issue(hash) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error removing issue with hash: {}, {}", hash, e);
                process::exit(-1);
            }
        }
    }
}

pub fn config(_args: &ArgMatches, working_dir: &PathBuf) {
    let storage = match IssueStorage::find_from_path(working_dir) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed finding issue tracker meta files: {}", e);
            process::exit(-1);
        }
    };

    display::config(storage.config());
}

fn load_issues(working_dir: &PathBuf) -> (IssueStorage, IssuesHandler) {
    let storage = match IssueStorage::find_from_path(working_dir) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed finding issue tracker meta files: {}", e);
            process::exit(-1);
        }
    };

    let mut handler = IssuesHandler::new();
    match handler.read_issues(&storage) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Reading issues failed due to: {}", e);
            process::exit(-1);
        }
    }

    (storage, handler)
}

fn store_issues(handler: &IssuesHandler, storage: &IssueStorage) {
    match handler.write_issues(&storage) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error writing issues: {}", e);
            process::exit(-1);
        }
    }
}

fn hash_parser(args: &ArgMatches) -> Option<IssueHash> {
    args.value_of("hash").map(|v| match IssueHash::from_str(v) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing issue hash: {}, {}", v, e);
            process::exit(-1);
        }
    })
}

fn type_parser(args: &ArgMatches) -> Option<IssueType> {
    // TODO: implement handling multiple types as argument
    args.value_of("type").map(|v| match IssueType::from_str(v) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing issue type: {}, {}", v, e);
            process::exit(-1);
        }
    })
}

fn datetime_parser(dt: &str) -> i64 {
    match Local.datetime_from_str(dt, "%Y-%m-%d %H:%M") {
        Ok(v) => v.timestamp(),
        Err(e) => {
            eprintln!("Error parsing date time: {}, {}", dt, e);
            process::exit(-1);
        }
    }
}

fn priority_parser(args: &ArgMatches) -> Option<IssuePriority> {
    // TODO: implement handling multiple priorities as argument
    args.value_of("priority")
        .map(|v| match IssuePriority::from_str(v) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error parsing issue priority: {}, {}", v, e);
                process::exit(-1);
            }
        })
}

fn status_parser(args: &ArgMatches) -> Option<IssueStatus> {
    // TODO: implement handling multiple statuses as argument
    args.value_of("status")
        .map(|v| match IssueStatus::from_str(v) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error parsing issue status: {}, {}", v, e);
                process::exit(-1);
            }
        })
}

fn issue_hash_parser(handler: &IssuesHandler, args: &ArgMatches) -> Issue {
    let hash = hash_parser(args).unwrap();

    match handler.issue(&hash) {
        Some(v) => v.to_owned(),
        None => {
            eprintln!("Issue with hash {} doesn't exist", hash);
            process::exit(-1);
        }
    }
}

fn issue_type_parser(issue: &mut Issue, args: &ArgMatches) {
    if let Some(typ) = type_parser(args) {
        issue.set_typ(typ);
    }
}

fn issue_parent_parser(issue: &mut Issue, args: &ArgMatches, storage: &IssueStorage) {
    if let Some(v) = args.value_of("parent") {
        let parent = match IssueHash::from_str(v) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error parsing issue parent hash: {}, {}", v, e);
                process::exit(-1);
            }
        };
        if !storage.issue_exists(&parent) {
            eprintln!("Error parent issue hash {} doesn't exist", parent);
            process::exit(-1);
        }
        issue.set_parent(Some(parent));
    }
}

fn issue_priority_parser(issue: &mut Issue, args: &ArgMatches) {
    if let Some(priority) = priority_parser(args) {
        issue.set_priority(priority);
    }
}

fn issue_status_parser(issue: &mut Issue, args: &ArgMatches) {
    if let Some(status) = status_parser(args) {
        issue.set_status(status);
    }
}

fn issue_assigned_to_parser(issue: &mut Issue, args: &ArgMatches) {
    if let Some(v) = args.value_of("assigned_to") {
        let assigend_to = v.to_string();
        issue.set_assigned_to(Some(assigend_to));
    }
}

fn issue_start_date_parser(issue: &mut Issue, args: &ArgMatches) {
    if let Some(v) = args.value_of("start_date") {
        let ts = datetime_parser(v);
        issue.set_start_date(ts);
    }
}

fn issue_due_date_parser(issue: &mut Issue, args: &ArgMatches) {
    if let Some(v) = args.value_of("due_date") {
        let ts = datetime_parser(v);
        issue.set_due_date(Some(ts));
    }
}

fn issue_progress_parser(issue: &mut Issue, args: &ArgMatches) {
    if let Some(v) = args.value_of("progress") {
        let progress = match v.parse::<u8>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error parsing given progress as integer: {}, {}", v, e);
                process::exit(-1);
            }
        };
        match issue.set_progress(progress) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error setting progress: {}, {}", progress, e);
                process::exit(-1);
            }
        }
    }
}

fn issue_description_edit_parser(issue: &mut Issue, args: &ArgMatches) {
    // create temporary txt file, open corresponding editor, set description
    if args.is_present("description_edit") {
        // create temporary file for description
        let mut tmp_file = match tempfile::Builder::new().suffix(".txt").tempfile() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error creating temporary description file: {}", e);
                process::exit(-1);
            }
        };

        // write current issue description to temporary file
        if let Some(ref description) = *issue.description() {
            match tmp_file.write_all(description.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error writing issue desciption to temporary file: {}", e);
                    let _ = tmp_file.close();
                    process::exit(-1);
                }
            }
            match tmp_file.flush() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error syncing desciption data to filesystem: {}", e);
                    let _ = tmp_file.close();
                    process::exit(-1);
                }
            }
        }

        // try to find suitable editor from environment for description editing
        let editor = env::var("VISUAL").ok();
        let editor = editor.or_else(|| env::var("EDITOR").ok());
        match editor {
            Some(v) => {
                let path = tmp_file.path().to_owned();
                let ret = process::Command::new(v).arg(path).status();

                if let Err(exit_status) = ret {
                    eprintln!("Editor doesn't exit cleanly. Exit status: {}", exit_status);
                    process::exit(-1);
                }
            }
            None => {
                eprintln!("No suitable editor found");
                let _ = tmp_file.close();
                process::exit(-1);
            }
        }

        // set cursor to start to read whole string not
        match tmp_file.seek(SeekFrom::Start(0)) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error resetting file cursor: {}", e);
                let _ = tmp_file.close();
                process::exit(-1);
            }
        }

        let mut description = String::new();
        match tmp_file.read_to_string(&mut description) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to extract description from temporary file: {}", e);
                let _ = tmp_file.close();
                process::exit(-1);
            }
        }
        let description = description.trim().to_string();
        issue.set_description(Some(description));

        let _ = tmp_file.close();
    }
}
