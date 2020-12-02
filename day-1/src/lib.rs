use std::num::ParseIntError;
use std::str::FromStr;

pub struct ExpenseReport {
    expenses: Vec<usize>,
}

impl ExpenseReport {
    pub fn check(&self) -> usize {
        for (i, a) in self.expenses.iter().enumerate() {
            for n in i + 1..self.expenses.len() {
                let b = self.expenses[n];

                if a + b == 2020 {
                    return a * b;
                }
            }
        }

        0
    }

    pub fn check3(&self) -> usize {
        for (i, a) in self.expenses.iter().enumerate() {
            for n in i + 1..self.expenses.len() {
                let b = self.expenses[n];
                for j in n + 1..self.expenses.len() {
                    let c = self.expenses[j];

                    if a + b + c == 2020 {
                        return a * b * c;
                    }
                }
            }
        }

        0
    }
}

impl FromStr for ExpenseReport {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<ExpenseReport, ParseIntError> {
        let mut expenses = Vec::new();
        for line in s.trim().split('\n') {
            expenses.push(line.parse()?);
        }

        Ok(ExpenseReport { expenses })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = r#"1721
979
366
299
675
1456
"#;

    #[test]
    fn input_test() {
        let report: ExpenseReport = INPUT.parse().unwrap();

        assert_eq!(report.expenses, vec![1721, 979, 366, 299, 675, 1456]);
    }

    #[test]
    fn example() {
        let report: ExpenseReport = INPUT.parse().unwrap();

        assert_eq!(report.check(), 514579);
    }
}
