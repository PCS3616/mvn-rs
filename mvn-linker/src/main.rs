use indoc::indoc;

use mvn_linker::processor::process;

fn main() {
    let main_program = indoc! {"
        1000 0000 ; < _ADD_TWO
        1000 0001 ; < FOO
        0000 0002 ; > RESULT
        0000 0004 ;         JP  MAIN
        0002 0000 ; RESULT  $   /2
        0004 3003 ; MAIN    LV  /3
        1006 A000 ;         SC  ADD_TWO
        1008 9001 ;         MM  FOO
        000A C000 ;         HM  /0
        000C 0000 ; # MAIN
    "};
    let subroutine_program = indoc! {"
        6100 0102 ; > _ADD_TWO
        5000 0000 ; < RESULT
        4100 0002 ; TWO         K   /2
        4102 0000 ; ADD_TWO     $   /2
        6104 4100 ;             AD  TWO
        5106 9000 ;             MM  RESULT
        6108 B102 ;             RS  ADD_TWO
    "};

    let programs = vec![main_program, subroutine_program];
    let processor = process(programs);

    println!("{:#?}", processor);
}
