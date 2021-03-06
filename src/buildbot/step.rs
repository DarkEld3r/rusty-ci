use std::fmt::{Display, Error, Formatter};

/// This describes a step executed by a builder in buildbot
/// This is used by the Builder struct to construct the Builder config file
#[derive(Clone, Debug)]
pub enum Step {
    /// Represents a git clone operation
    GitClone {
        url: String, // The repo to clone
    },

    /// Represents a git clone operation targetting gitlab.
    /// The reason we need to know specifically about if the clone
    /// is from gitlab is because this also handles merge requests
    GitLabClone {
        url: String, // The repo to clone
    },

    /// Represents a command line command
    Command {
        command: String,         // The command to run
        workdir: Option<String>, // The optional workdir
    },
}

/// Implementation of step struct
impl Step {
    /// Construct a command line step
    pub fn command<S: Display>(command: S, workdir: Option<S>) -> Self {
        Step::Command {
            command: command.to_string(),
            workdir: workdir.and_then(|s| Some(s.to_string())),
        }
    }

    /// Construct a git clone step
    pub fn git_clone<S: Display>(url: S) -> Self {
        Step::GitClone {
            url: url
                .to_string()
                .trim()
                .trim_start_matches('\"')
                .trim_end_matches('\"')
                .to_string(),
            // branch: branch.to_string(),
        }
    }

    /// Construct a gitlab clone step
    pub fn gitlab_clone<S: Display>(url: S) -> Self {
        Step::GitLabClone {
            url: url
                .to_string()
                .trim()
                .trim_start_matches('\"')
                .trim_end_matches('\"')
                .to_string(),
            // branch: branch.to_string(),
        }
    }
}

/// Convert Step to String / Allow string formatting for Step
impl Display for Step {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            // Used by buildbot to get the updated repository
            Step::GitClone { url } => write!(f,
                "steps.Git(repourl=\"{}\", mode=\"full\", branch=\"master\", method=\"clobber\", shallow=False, submodules=True)", url),

            // Used by buildbot to get the updated repository
            Step::GitLabClone { url } => write!(f,
                "steps.GitLab(repourl=\"{}\", mode=\"full\", branch=\"master\", method=\"clobber\", shallow=False, submodules=True)", url),

            // Command with provided work directory
            Step::Command {command, workdir: Some(workdir)} => write!(
                f, "steps.ShellCommand(command={command:?}, workdir=\"{workdir}\")",
                command = command
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>(),
                workdir = workdir
            ),

            // Command without provided work directory
            Step::Command {command, workdir: None} => write!(
                f, "steps.ShellCommand(command={:?})",
                command.split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>()
            ),
        }
    }
}
