use nom::{
    IResult,
    Parser,
    character::complete::{ alpha1, char, one_of, multispace0 },
    combinator::{ map_res, map },
    sequence::{ tuple, delimited },
    branch::alt,
    multi::{ many0, separated_list0},
    error::ParseError,
};

/*

/// Load 16 bit immediate into BC
01: LDBCnn =>
    c <- (read_byte pc)
    b <- (read_byte pc + 1)
    pc <- pc + 2
    (tick 3)
end
// if ... then ... else ... end

intruction: number: word => statements end

statements: statement+

statement:
   | assignment
   | funcall
   | condition

assignement: word -> expression

expression:
   | identifier
   | binop
   | funcall

binop: expression op expression

funcall: (word expression+)

condition: if expression then statement else statement end
*/

// FIXME add whitespaces where needed

enum BinOp {
    Add, Sub, RShift, LShift
}
enum Expression {
    Identifier(String),
    BinaryOperation(Box<(Expression, BinOp, Expression)>),
    FunctionCall(String, Vec<Expression>)
}

struct Instruction {}

fn whitespaces<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(f: F) -> impl Parser<&'a str, O, E> {
    delimited(multispace0, f, multispace0)
}
fn identifier(input: &str) -> IResult<&str, Expression> {
    map(alpha1, |i: &str| Expression::Identifier(i.to_string()))(input)
}
fn binary_operator(input: &str) -> IResult<&str, BinOp> {
    map_res(one_of("+-><"), |c| match c {
        '+' => Ok(BinOp::Add),
        '-' => Ok(BinOp::Sub),
        '>' => Ok(BinOp::RShift),
        '<' => Ok(BinOp::LShift),
        o => Err(format!("Unknonw operator {}", o))
    })(input)
}
fn expression(input: &str) -> IResult<&str, Expression> {
    alt((identifier,
        funcall,
        binary_operation))(input)
}
fn binary_operation(input: &str) -> IResult<&str, Expression> {
    map(
        tuple((expression, whitespaces(binary_operator), expression)),
        |(e1, op, e2)| Expression::BinaryOperation(Box::new((e1, op, e2)))
    )(input)
}
fn funcall(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            char('('),
            tuple((
                whitespaces(identifier),
                separated_list0(multispace0, expression))),
            char(')')),
        |(n, e)| { match n {
            Expression::Identifier(n) => Expression::FunctionCall(n, e),
            _ => panic!("This shouldn't have happened, there's a bug somewhere in the code")
        } }
    )(input)
}

// fn parse(input: &str) -> IResult<&str, Instruction> {
// }
