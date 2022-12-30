use indoc::indoc;

use mvn_assembler::parser::Parse;
use mvn_assembler::processor::address::AddressedProgram;
use mvn_assembler::writer::base::print;
use types::Program;

fn main() {
    let program = Program::parse(indoc! {"
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
    "}).unwrap().1;
    let addresses_program = AddressedProgram::process(program);
    print(&addresses_program);
}
