use std::env;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::process;

fn main() {
    match env::args_os().nth(1) {
        Some(file_path) => {
            let input = parse_input(file_path);
            println!("{}", part1(&input));
            println!("{}", part2(&input));
        }
        None => {
            eprintln!("expected 1 argument, but got none");
            process::exit(1);
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Rational {
    numerator: i64,
    denominator: i64,
}

#[derive(Debug, Clone, Copy, Hash)]
struct Matrix2x2(Rational, Rational, Rational, Rational);

type Input = Vec<(Matrix2x2, [i64; 2])>;

impl Rational {
    fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("Zero is an invalid denominator");
        }
        Self {
            numerator,
            denominator,
        }
        .simplify()
    }

    fn from_int(numerator: i64) -> Self {
        Self {
            numerator,
            denominator: 1,
        }
    }

    fn simplify(self) -> Self {
        let mut a = self.numerator;
        let mut b = self.denominator;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        let mut simplified = Self {
            numerator: -self.numerator / a,
            denominator: -self.denominator / a,
        };
        if simplified.denominator < 0 {
            simplified.numerator *= -1;
            simplified.denominator *= -1;
        }
        simplified
    }

    fn is_int(&self) -> bool {
        self.denominator == 1
    }

    fn to_int(self) -> Option<i64> {
        if self.is_int() {
            Some(self.numerator)
        } else {
            None
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.numerator, self.denominator)
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.numerator == 0 {
            panic!("Cannot divide by zero")
        }
        Self::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl Matrix2x2 {
    fn from_ints(row1: [i64; 2], row2: [i64; 2]) -> Self {
        Self(
            Rational::from_int(row1[0]),
            Rational::from_int(row1[1]),
            Rational::from_int(row2[0]),
            Rational::from_int(row2[1]),
        )
    }

    fn invert(self) -> Option<Self> {
        let determinant = self.0 * self.3 - self.1 * self.2;
        if determinant.numerator == 0 {
            None
        } else {
            Some(Self(
                self.3 / determinant,
                -self.1 / determinant,
                -self.2 / determinant,
                self.0 / determinant,
            ))
        }
    }
}

impl fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "⎡{} {}⎤\n⎣{} {}⎦", self.0, self.1, self.2, self.3)
    }
}

impl Mul<[Rational; 2]> for Matrix2x2 {
    type Output = [Rational; 2];

    fn mul(self, rhs: [Rational; 2]) -> Self::Output {
        [
            self.0 * rhs[0] + self.1 * rhs[1],
            self.2 * rhs[0] + self.3 * rhs[1],
        ]
    }
}

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path)
        .unwrap()
        .split("\n\n")
        .map(|para| {
            let mut values = para
                .lines()
                .flat_map(|line| line.split(": ").nth(1).unwrap().split(", "))
                .map(|component| {
                    component
                        .replace('=', "+")
                        .split('+')
                        .nth(1)
                        .unwrap()
                        .parse()
                        .unwrap()
                });
            let a = values.next().unwrap();
            let c = values.next().unwrap();
            let b = values.next().unwrap();
            let d = values.next().unwrap();
            let prize_x = values.next().unwrap();
            let prize_y = values.next().unwrap();
            (
                Matrix2x2::from_ints(
                    //
                    [a, b],
                    [c, d],
                ),
                [prize_x, prize_y],
            )
        })
        .collect()
}

fn part1(input: &Input) -> i64 {
    input
        .iter()
        .flat_map(|(coefficients, constants)| {
            coefficients.invert().and_then(|inverted| {
                let result = inverted
                    * [
                        Rational::from_int(constants[0]),
                        Rational::from_int(constants[1]),
                    ];
                result[0]
                    .to_int()
                    .and_then(|a| result[1].to_int().map(|b| a * 3 + b))
            })
        })
        .sum()
}

fn part2(input: &Input) -> i64 {
    input
        .iter()
        .flat_map(|(coefficients, constants)| {
            coefficients.invert().and_then(|inverted| {
                let result = inverted
                    * [
                        Rational::from_int(constants[0] + 10000000000000),
                        Rational::from_int(constants[1] + 10000000000000),
                    ];
                result[0]
                    .to_int()
                    .and_then(|a| result[1].to_int().map(|b| a * 3 + b))
            })
        })
        .sum()
}
