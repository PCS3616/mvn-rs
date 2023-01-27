use indoc::indoc;

use mvn_assembler::processor::process;
use mvn_assembler::writer::print;

fn main() {
    let program = indoc! {"
        < IMPORTED
        > RESERVE
        > TWO
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
    "};
    let validator_output = process(program);
    print(program, validator_output);
}
