use crate::commands::command::Command;
use crate::repl::jobs_store;
use std::io::Write;

pub fn jobs(_command: &Command, writer: &mut dyn Write) {
    let jobs_snapshot = jobs_store::snapshot();

    for (i, job) in jobs_snapshot.iter().enumerate() {
        let last_background_job = i + 1 == jobs_snapshot.len();
        let second_to_last_background_job = i + 1 == jobs_snapshot.len() - 1;

        let job_order: char = if last_background_job {
            '+'
        } else if second_to_last_background_job {
            '-'
        } else {
            ' '
        };

        writeln!(
            writer,
            "[{}]{}  {:<24}{}",
            job.id, job_order, job.status, job.command
        )
            .unwrap();
    }
}
