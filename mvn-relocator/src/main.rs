use indoc::indoc;

use mvn_relocator::processor::process;
use mvn_relocator::writer::print;

fn main() {
    let program = indoc! {"
        0000 0004 ;         JP  MAIN
        0002 0000 ; RESULT  $   /1
        0004 3003 ; MAIN    LV  /3
        2006 A110 ;         SC  ADD_TWO
        000A C000 ;         HM  /0
        000C 0000 ; # MAIN
        000E 0000 ;         &   /100
        410E 0002 ; TWO     K   /2
        4110 0000 ; ADD_TWO $   /1
        6112 410E ;         AD  TWO
        4114 9002 ;         MM  RESULT
        6116 B110 ;         RS  ADD_TWO
    "};
    let processor_output = process(program, 0x10);
    print(processor_output);
}
