use indoc::indoc;

use mvn::parser::lines::Lines;
use mvn::processor::address::AddressedLines;
use mvn::writer::base::print;
use mvn::parser::util::Span;

fn main() {
    let program = Lines::parse(Span::new(indoc! {"
        < IMPORTED
        > RESERVE
        @ /10
                JP  MAIN
        TWO     K   /2 ; This is an inline comment
        & /200
        FOUR    K   /4
        RESERVE $   /4
        ; This is a comment
        MAIN    LD  TWO
        & /100
                AD  FOUR
                MM  RESERVE
                HM  /0
        # MAIN
    "})).unwrap().1;
    let addresses_program = AddressedLines::parse(program);
    print(&addresses_program);
}
