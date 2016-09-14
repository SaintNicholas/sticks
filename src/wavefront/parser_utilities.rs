use nom::{space, digit, eof, not_line_ending};
use std::str;
use std::str::FromStr;

named!(pub parse_f64<f64>,
    chain!(
        neg: opt!(tag!("-")) ~
        a: map_res!(digit, str::from_utf8) ~
        opt!(tag!(".")) ~
        c: opt!(map_res!(digit, str::from_utf8)),

        ||{
               let mut float_string: String = a.to_string();
               if let Some(i) = c {
                  float_string = float_string + "." + &i;
               }
               let mut value = f64::from_str(&float_string[..]).unwrap();
               if let Some(j) = neg {
                   value = -value
               }
               value}
    )
);

named!(pub parse_int<isize>,
    chain!(
        neg: opt!(tag!("-")) ~
        a: map_res!(digit, str::from_utf8),

        ||{
               let mut int_string: String = a.to_string();
               let mut value = isize::from_str(&int_string[..]).unwrap();
               if let Some(j) = neg {
                   value = -value
               }
               value}
    )
);

named!(pub parse_ignored_line,
    chain!(
        alt!(parse_blank_line | parse_comment),

        || { &b""[..] }
    )
);

named!(pub parse_blank_line,
    chain!(
        many0!(space) ~
        alt!(eof | parse_eol),
        
        || { &b""[..] }
    )
);

named!(pub parse_comment,
    chain!(
        many0!(space) ~
        tag!("#") ~
        not_line_ending ~
        alt!(eof | parse_eol),
        
        || { &b""[..] }
    )
);

named!(pub parse_eol,
    alt!(tag!("\n") | tag!("\r\n") | tag!("\u{2028}") | tag!("\u{2029}"))
);

named!(pub not_space,
    is_not!(" \t\r\n")
);
