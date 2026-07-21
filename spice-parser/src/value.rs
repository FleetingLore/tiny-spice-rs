//! SPICE value expression parser.
//!
//! Handles:
//! - `{param_name}` → `Expression::Identifier`
//! - `10k`, `4.7u`, `100n`, etc. → `Expression::Literal`

use tiny_spice::bracket_expression::Expression;

/// Extract a bracket expression or literal value from SPICE text.
pub fn extract_expression(text: &str) -> Option<Expression> {
    if !text.starts_with('{') {
        let val = extract_value(text);
        if let Some(n) = val {
            return Some(Expression::Literal(n));
        } else {
            eprintln!("*ERROR* expected numerical literal in expression");
            return None;
        }
    }

    eprintln!("*WARN* only expressions that are a single identifier are supported");

    let expr_str: Vec<_> = text.chars().collect();
    let mut i: usize = 0;

    if expr_str[i] == '{' {
        i += 1
    } else {
        eprintln!("*ERROR* - expected '{{'");
        return None;
    }

    while expr_str[i].is_whitespace() {
        i += 1;
        if i > expr_str.len() {
            return None;
        }
    }

    let mut ident_chars: Vec<char> = vec![];
    while expr_str[i].is_ascii_alphanumeric() {
        ident_chars.push(expr_str[i]);
        i += 1;
        if i > expr_str.len() {
            return None;
        }
    }

    let ident: String = ident_chars.into_iter().collect();

    while expr_str[i].is_whitespace() {
        i += 1;
        if i > expr_str.len() {
            return None;
        }
    }

    if expr_str[i] != '}' {
        eprintln!("*ERROR* - expected '}}'");
        return None;
    }

    Some(Expression::Identifier(ident))
}

/// Extract an element identifier from SPICE.
pub fn extract_identifier(text: &str) -> String {
    text.to_string()
}

// ---------------------------------------------------------------------------
// Engineering notation value parser
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum ValueState {
    Start,
    Int,
    Frac,
    ExpStart,
    Exp,
    Unit,
}

/// Parse a SPICE-style numeric value with engineering suffix.
///
/// Supported: `k`(1e3) `m`(1e-3) `u`(1e-6) `n`(1e-9) `p`(1e-12)
///
/// Examples: `10`, `10.0`, `10k`, `4.7u`, `100n`, `1e-6`, `10mA`
pub fn extract_value(text: &str) -> Option<f64> {
    let mut value: Option<f64> = None;
    let mut float_str = String::new();
    let mut c: char;
    let mut state = ValueState::Start;
    let mut nxt;
    let mut eng_mult: f64 = 1.0;

    let mut text_iter = text.chars();

    fn eval(txt: &str, mult: f64) -> Option<f64> {
        Some(txt.parse::<f64>().unwrap() * mult)
    }

    'things: loop {
        if let Some(c_) = text_iter.next() {
            c = c_;
        } else {
            break 'things;
        }
        match state {
            ValueState::Start => match c {
                '+' | '-' => {
                    float_str.push(c);
                    nxt = ValueState::Int
                }
                '0'..='9' => {
                    float_str.push(c);
                    nxt = ValueState::Int
                }
                _ => break 'things,
            },

            ValueState::Int => match c {
                '0'..='9' => {
                    float_str.push(c);
                    nxt = ValueState::Int
                }
                '.' => {
                    float_str.push(c);
                    nxt = ValueState::Frac
                }
                'e' => {
                    float_str.push(c);
                    nxt = ValueState::ExpStart
                }
                'k' => {
                    eng_mult = 1e3;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                'm' => {
                    eng_mult = 1e-3;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                'u' => {
                    eng_mult = 1e-6;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                'n' => {
                    eng_mult = 1e-9;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                'p' => {
                    eng_mult = 1e-12;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                _ => break 'things,
            },

            ValueState::Frac => match c {
                '0'..='9' => {
                    float_str.push(c);
                    nxt = ValueState::Frac
                }
                'e' => {
                    float_str.push(c);
                    nxt = ValueState::ExpStart
                }
                'k' => {
                    eng_mult = 1e3;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                'm' => {
                    eng_mult = 1e-3;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                'u' => {
                    eng_mult = 1e-6;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                'n' => {
                    eng_mult = 1e-9;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                'p' => {
                    eng_mult = 1e-12;
                    value = eval(&float_str, eng_mult);
                    nxt = ValueState::Unit
                }
                _ => break 'things,
            },

            ValueState::ExpStart => match c {
                '+' | '-' => {
                    float_str.push(c);
                    nxt = ValueState::Exp
                }
                '0'..='9' => {
                    float_str.push(c);
                    nxt = ValueState::Exp
                }
                _ => break 'things,
            },

            ValueState::Exp => match c {
                '0'..='9' => {
                    float_str.push(c);
                    nxt = ValueState::Exp
                }
                _ => break 'things,
            },

            ValueState::Unit => break 'things,
        }
        state = nxt;
    }

    match state {
        ValueState::Int | ValueState::Frac | ValueState::Exp => value = eval(&float_str, eng_mult),
        _ => {}
    }

    value
}
