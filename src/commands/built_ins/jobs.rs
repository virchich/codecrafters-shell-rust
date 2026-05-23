use crate::state::jobs_store;
use crate::state::jobs_store::JobStatus;
use crate::syntax::command_invocation::CommandInvocation;
use std::io::Write;

pub fn jobs(_command: &CommandInvocation, writer: &mut dyn Write, _writer_err: &mut dyn Write) {
    let jobs_snapshot = jobs_store::list_and_reap();

    print_jobs(&jobs_snapshot, writer, |_| true);
}

pub fn print_done_jobs(writer: &mut dyn Write) {
    let jobs_snapshot = jobs_store::list_and_reap();

    print_jobs(&jobs_snapshot, writer, |job| job.status == "Done");
}

fn print_jobs(
    jobs_snapshot: &[JobStatus],
    writer: &mut dyn Write,
    should_print: impl Fn(&JobStatus) -> bool,
) {
    for (index, job) in jobs_snapshot.iter().enumerate() {
        if should_print(job) {
            print_job(job, job_order(index, jobs_snapshot.len()), writer);
        }
    }
}

fn print_job(job: &JobStatus, job_order: char, writer: &mut dyn Write) {
    writeln!(
        writer,
        "[{}]{}  {:<24}{}",
        job.id, job_order, job.status, job.command
    )
    .unwrap();
}

fn job_order(index: usize, job_count: usize) -> char {
    if index + 1 == job_count {
        '+'
    } else if index + 1 == job_count.saturating_sub(1) {
        '-'
    } else {
        ' '
    }
}

#[cfg(test)]
mod tests {
    use super::{job_order, print_jobs};
    use crate::state::jobs_store::JobStatus;

    #[test]
    fn marks_last_two_jobs_with_shell_markers() {
        assert_eq!(job_order(0, 3), ' ');
        assert_eq!(job_order(1, 3), '-');
        assert_eq!(job_order(2, 3), '+');
    }

    #[test]
    fn prints_only_jobs_that_match_filter() {
        let jobs = vec![
            JobStatus {
                id: 1,
                command: "sleep 1".to_string(),
                status: "Running".to_string(),
            },
            JobStatus {
                id: 2,
                command: "echo done".to_string(),
                status: "Done".to_string(),
            },
        ];
        let mut output = Vec::new();

        print_jobs(&jobs, &mut output, |job| job.status == "Done");

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "[2]+  Done                    echo done\n"
        );
    }
}
