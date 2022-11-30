pub mod algorithms {}

pub mod scanner {
    enum InputSource {
        Stdin,
        FromFile(Vec<String>),
    }

    pub struct Scanner {
        buffer: Vec<String>,
        input_source: InputSource,
    }

    impl Scanner {
        #[allow(dead_code)]
        pub fn new() -> Self {
            Self {
                buffer: vec![],
                input_source: InputSource::Stdin,
            }
        }

        #[allow(dead_code)]
        pub fn new_file(filename: &str) -> Self {
            let file = std::fs::read_to_string(filename).unwrap();
            let mut lines: Vec<String> = file.lines().map(|line| String::from(line)).collect();
            lines.reverse();
            Self {
                buffer: vec![],
                input_source: InputSource::FromFile(lines),
            }
        }

        #[allow(dead_code)]
        pub fn i64(&mut self) -> i64 {
            self.next::<i64>()
        }

        #[allow(dead_code)]
        pub fn i32(&mut self) -> i32 {
            self.next::<i32>()
        }

        #[allow(dead_code)]
        pub fn usize(&mut self) -> usize {
            self.next::<usize>()
        }

        #[allow(dead_code)]
        pub fn f64(&mut self) -> f64 {
            self.next::<f64>()
        }

        #[allow(dead_code)]
        pub fn f32(&mut self) -> f32 {
            self.next::<f32>()
        }

        #[allow(dead_code)]
        pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.next::<T>()).collect()
        }

        fn parse_next_line(&mut self) -> bool {
            let mut input = String::new();
            match &mut self.input_source {
                InputSource::Stdin => {
                    if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                        return false;
                    }
                }
                InputSource::FromFile(lines) => match lines.pop() {
                    Some(line) => input = line,
                    None => return false,
                },
            }

            self.buffer = input.split_whitespace().rev().map(String::from).collect();
            return true;
        }

        fn next<T: std::str::FromStr>(&mut self) -> T {
            loop {
                if let Some(token) = self.buffer.pop() {
                    return token.parse().ok().expect("Failed parse");
                }

                self.parse_next_line();
            }
        }

        #[allow(dead_code)]
        pub fn has_more_elements(&mut self) -> bool {
            loop {
                if !self.buffer.is_empty() {
                    return true;
                }
                if !self.parse_next_line() {
                    return false;
                }
            }
        }

        #[allow(dead_code)]
        pub fn string(&mut self) -> Vec<u8> {
            self.next::<String>().into_bytes()
        }
    }
}
