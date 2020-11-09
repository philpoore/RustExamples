extern crate nom;

use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple
};

#[derive(Debug,PartialEq)]
pub struct Color {
  pub red:   u8,
  pub green: u8,
  pub blue:  u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}
  
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
      take_while_m_n(2, 2, is_hex_digit),
      from_hex
    )(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
  
    Ok((input, Color { red, green, blue }))
}

fn main() {
    let color = hex_color("#2F14DF").unwrap().1;
    println!("Color = {:?}", color);
}

#[test]
fn test_color() {
    let color = hex_color("#2F14DF").unwrap().1;
    assert_eq!(color, Color{ red: 47, green: 20, blue: 223 });
}

#[test]
fn test_colors() {
    let red = hex_color("#FF0000").unwrap().1;
    assert_eq!(red, Color{ red: 255, green: 0, blue: 0 });
    
    let green = hex_color("#00FF00").unwrap().1;
    assert_eq!(green, Color{ red: 0, green: 255, blue: 0 });

    let blue = hex_color("#0000FF").unwrap().1;
    assert_eq!(blue, Color{ red: 0, green: 0, blue: 255 });
}

#[test]
fn test_lowercase() {
    let color = hex_color("#2f14df").unwrap().1;
    assert_eq!(color, Color{ red: 47, green: 20, blue: 223 });

    let red = hex_color("#ff0000").unwrap().1;
    assert_eq!(red, Color{ red: 255, green: 0, blue: 0 });
    
    let green = hex_color("#00ff00").unwrap().1;
    assert_eq!(green, Color{ red: 0, green: 255, blue: 0 });

    let blue = hex_color("#0000ff").unwrap().1;
    assert_eq!(blue, Color{ red: 0, green: 0, blue: 255 });
}

#[test]
fn test_error_on_input() {
    let color1 = hex_color("foo-bar");
    assert!(color1.is_err());
    let color2 = hex_color("#ff00ff");
    assert!(color2.is_ok());
}