use std::io;

fn main() {
    let mut calculator = RpnCalculator::Calculator::new();

    calculator.run();
}

mod RpnCalculator {
    use std::io;

    #[derive(Debug, Copy, Clone)]
    enum OperatorType { Plus, Minus, Times, Divide, Modulo }

    #[derive(Debug, Copy, Clone)]
    enum Token { Number (i32), Operator (OperatorType) }

    pub(crate) struct Calculator {
        stack: Vec<Token>,
    }

    impl OperatorType {
        fn to_string(&self) -> String {
            match &self {
                OperatorType::Plus => "+",
                OperatorType::Minus => "-",
                OperatorType::Times => "*",
                OperatorType::Divide => "/",
                OperatorType::Modulo => "%",
            }.to_string()
        }

        fn from(s: &str) -> Option<OperatorType> {
            // match s {
            //     "+" => Some ( OperatorType::Plus ),
            //     "-" => Some ( OperatorType::Minus ),
            //     "*" => Some ( OperatorType::Times ),
            //     "/" => Some ( OperatorType::Divide ),
            //     "%" => Some ( OperatorType::Modulo ),
            //     _ => None
            // }

            Some ( OperatorType::Plus )
        }
    }

    impl Token {
        fn clone(&self) -> Self { *self }
    }

    impl OperatorType {
        fn clone(&self) -> Self { *self }
    }

    impl Calculator {
        fn is_token_numeric(s: &str) -> bool {
            s.bytes().all(|c| c.is_ascii_digit())
        }

        fn push(&mut self, s: &str) {
            self.evaluate_stack();

            if Calculator::is_token_numeric(s) {
                self.stack.push(
                    Token::Number ( s.parse::<i32>().unwrap() )
                );
            } else {
                self.stack.push(
                    Token::Operator ( OperatorType::from(s).unwrap() )
                );
            }
        }

        fn evaluate_stack(&mut self) {
            if self.stack.len() < 3 { return; }

            let mut a = 0;
            let mut b = 0;
            let mut operator = OperatorType::Plus;

            if let Token::Number(n) = self.stack.get(0).unwrap() {
                a = *n;
            }
            if let Token::Number(n) = self.stack.get(1).unwrap() {
                b = *n;
            }

            if let Token::Operator(t) = self.stack.get(2).unwrap() {
                operator = *t;
            }

            let result: i32 = match operator {
                OperatorType::Plus => a + b,
                OperatorType::Minus => a - b,
                OperatorType::Times => a * b,
                OperatorType::Divide => a / b,
                OperatorType::Modulo => a % b,
            };

            self.stack.clear();
            self.stack.push(Token::Number(result));
        }

        pub fn run(&mut self) {
            loop {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(n) => {
                        for c in input.split(" ") {
                            println!("c: {}", c);
                            self.push(c);
                        }

                        self.evaluate_stack();

                        println!("Out: {:?}", &self.stack.get(0).unwrap());
                    }
                    Err(e) => {
                        println!("Error while reading from stdin: {}", e);
                    }
                }

            }
        }

        pub fn new() -> Calculator {
            Calculator {
                stack: Vec::new(),
            }
        }
    }
}
