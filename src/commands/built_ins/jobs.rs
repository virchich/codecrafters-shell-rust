use crate::commands::command::Command;
use crate::repl::jobs_store;
use std::io::Write;

pub fn jobs(_command: &Command, writer: &mut dyn Write) {
    let jobs_snapshot = jobs_store::snapshot();

    for job in jobs_snapshot.iter() {
        writeln!(writer, "[{}]+  {:<24}{}", job.id, job.status, job.command).unwrap();
    }
}
