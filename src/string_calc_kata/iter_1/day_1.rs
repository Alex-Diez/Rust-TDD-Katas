pub struct Calculator<'a> {
    line: &'a str,
    operands: Vec<char>,
}

impl <'a> Calculator<'a> {

    pub fn new(line: &'a str) -> Calculator {
        Calculator {
            line: line,
            operands: vec!['+', '-']
         }
    }

    pub fn evaluate(&self) -> u32 {
        let mut chars = self.line.chars();
        let first_arg = chars.by_ref()
                .take_while(
                    |c| !self.operands.contains(c)
                )
                .map(
                    |c| c.to_digit(10).unwrap()
                )
                .fold(
                    0, |acum, i| acum*10 + i
                );
        println!("first_arg - {:?}", first_arg);
        let sign = chars.by_ref().next();
        println!("sign - {:?}", sign);
        let second_arg = chars.by_ref().skip_while(|c| self.operands.contains(c)).map(|c| c.to_digit(10).unwrap()).fold(0, |acum, i| acum*10 + i);
        println!("second_arg - {:?}", second_arg);
        match sign {
            Some('+') => first_arg + second_arg,
            Some('-') => first_arg - second_arg,
            Some(_) => first_arg,
            None => 0
        }
    }
}
