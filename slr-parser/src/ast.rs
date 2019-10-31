#[derive(Debug, PartialEq)]
pub(crate) enum Ast {
    Add(Box<Ast>, Box<Ast>),
    Calculation(Box<Ast>),
    Multiply(Box<Ast>, Box<Ast>),
    Number(usize),
}

pub(crate) fn ast_create_addition(ast_stack: &mut Vec<Ast>) {
    let left_summand = ast_stack.pop().unwrap();
    let right_summand = ast_stack.pop().unwrap();

    ast_stack.push(Ast::Add(Box::new(left_summand), Box::new(right_summand)));
}

pub(crate) fn ast_create_calculation(ast_stack: &mut Vec<Ast>) {
    let last_element = ast_stack.pop().unwrap();

    ast_stack.push(Ast::Calculation(Box::new(last_element)));
}

pub(crate) fn ast_create_multiplication(ast_stack: &mut Vec<Ast>) {
    let left_multiplier = ast_stack.pop().unwrap();
    let right_multiplier = ast_stack.pop().unwrap();

    ast_stack.push(Ast::Multiply(
        Box::new(left_multiplier),
        Box::new(right_multiplier),
    ));
}

pub(crate) fn ast_create_number(ast_stack: &mut Vec<Ast>) {
    ast_stack.push(Ast::Number(0));
}
