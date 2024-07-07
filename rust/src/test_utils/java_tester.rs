use std::io::{BufRead, BufReader, Write, LineWriter};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use serde::Serialize;
use serde_json::Value;
use crate::test_utils::serde_bytes;

#[derive(Serialize)]
struct TestCase {
    class: String,
    version: u16,
    json: Value,
    #[cfg_attr(test, serde(with="serde_bytes"))]
    serialized: Vec<u8>,
}

pub(crate) struct JavaTester {
    child: Child,
    child_stdin: LineWriter<ChildStdin>,
    child_stdout: BufReader<ChildStdout>,
}

impl JavaTester {
    pub(crate) fn new() -> Self {
        let mut child = Command::new("../java-tester/build/install/java-tester/bin/java-tester")
            .args(vec![":java-tester:run"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to run java_tester");

        let stdin = LineWriter::new(child.stdin.take().unwrap());
        let mut stdout = BufReader::new(child.stdout.take().unwrap());
        let mut line: String = String::new();
        while line.trim() != "Java tester started" {
            line.clear();
            stdout.read_line(&mut line).expect("failed to read from java_tester stdout");
            print!("{}", line);
        }

        JavaTester { child, child_stdin: stdin, child_stdout: stdout }
    }

    pub(crate) fn test(&mut self, class: &str, version: u16, json: Value, serialized: &[u8]) {
        let case = TestCase { class: class.to_string(), version, json, serialized: serialized.to_vec() };
        let mut case_str = serde_json::to_string(&case).unwrap();
        case_str.push('\n');
        let case_bytes = case_str.as_bytes();

        self.child_stdin.write_all(case_bytes).unwrap();
        self.child_stdin.flush().unwrap();
        let mut line: String = String::new();
        self.child_stdout.read_line(&mut line).unwrap();
        let result = serde_json::from_str::<Value>(&line).unwrap();
        let success = result.get("success").unwrap().as_bool().unwrap();
        if !success {
            let message = result.get("message").unwrap().as_str()
                .or(result.get("exception").unwrap().as_str())
                .unwrap();
            assert!(success, "{}", format!("{}", message));
        }
    }
}

impl Drop for JavaTester {
    fn drop(&mut self) {
        let _ = self.child.kill()
            .inspect_err(|e| eprintln!("failed to kill java tester: {e}"));
    }
}
