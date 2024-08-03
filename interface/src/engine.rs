use std::{
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Child, ChildStdout, Command, Stdio}, time::Duration,
};

use types::{bitboard::BitBoard, player::Player};
pub struct Engine {
    child: Child,
    buf: BufReader<ChildStdout>,
}

impl Write for Engine {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.child
            .stdin
            .as_mut()
            .expect("unable to get engine stdin")
            .write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.child
            .stdin
            .as_mut()
            .expect("unable to get engine stdin")
            .flush()
    }
}

impl Engine {
    pub fn from_path(path: &Path) -> Self {
        let mut child = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Invalid engine");
        let buf = BufReader::new(
            child
                .stdout
                .take()
                .expect("couldn't retrieve engine's stdout"),
        );
        Self { child, buf }
    }

    pub fn read_line(&mut self) -> String {
        let mut line = String::new();
        self.buf.read_line(&mut line).expect("unable to read line");
        let line = line.trim().to_owned();
        // println!("--> {line}");
        line
    }

    /// Determines if the provided executable supports the Connect-4 Interface (C4I)
    pub fn is_c4i(&mut self) -> bool {
        self.write_all("c4i\n".as_bytes()).expect("couldn't write");
        self.flush().expect("couldn't flush da toilet");
        loop {
            let line = self.read_line();
            if line.eq_ignore_ascii_case("c4iok") {
                return true;
            }
        }
    }

    pub fn startpos(&mut self) {
        // println!("startpos");
        self.write_all("position startpos\n".as_bytes()).expect("couldn't write position");
        self.write_all("setoption turn yellow\n".as_bytes()).expect("couldn't write turn");
        self.flush().expect("couldn't flush");
    }

    pub fn set_position(&mut self, turn: Player, red: BitBoard, yellow: BitBoard) {
        // println!("setpos");
        let colour = match turn {
            Player::Red => "red",
            Player::Yellow => "yellow"
        };
        self.write_all(format!("setoption turn {colour}\n").as_bytes()).expect("couldn't write turn");
        self.write_all(format!("position custom {} {}\n", red.0, yellow.0).as_bytes()).expect("couldn't write position");
        self.flush().expect("couldn't flush");
    }

    pub fn get_best(&mut self, time: Duration) -> usize {
        self.write_all(format!("go time {}\n", time.as_micros()).as_bytes()).expect("couldn't write");
        self.flush().expect("couldn't flush");
        loop {
            let line = self.read_line();
            // println!("EWE {line}");
            if line.starts_with("bestmove") {
                let mut parts = line.split_whitespace();
                let col = parts.nth(1).expect("engine api error");
                let col = col.parse::<usize>().expect("couldn't convert move column");
                return col;
            }
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.child.kill().expect("Could not kill engine process");
    }
}
