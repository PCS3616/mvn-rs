pub fn asm2mvn(_asm: &str) -> &str {
    todo!("Not implemented yet")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn should_returns_mvn_given_asm() {
        let input = indoc! {"
            VAL_A   K   /0001
            VAL_B   K   /0002
            RESLT   K   /0000

                    @   /0100
            MAIN    LD  VAL_A
                    AD  VAL_B
                    MM  RESLT
        "};

        let expected = indoc! {"
            00000001
            00020002
            00040000
            01008000
            01024002
            01049004
        "};

        assert_eq!(asm2mvn(input), expected);
    }
}
