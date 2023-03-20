




pub enum Token {


    #[regex(r"0b_*[01][_01]*", |lex| {
        parse_int::parse::<i64>(lex.slice())
    }, priority = 3)]
    IntegerBase2(i64),


}