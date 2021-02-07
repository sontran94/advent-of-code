fn main() -> anyhow::Result<()> {
    let input = include_str!("input2.txt")
        .lines()
        .map(|l| parser(l))
        .collect::<Result<Vec<bool>, _>>()?;

    dbg!(input);

    Ok(())
}

fn parser(s: &str) -> anyhow::Result<bool> {
    peg::parser! {
    grammar parser() for str {
        pub(crate) rule line() -> bool
            = l:letter1() t:letter2(l) [_]* {t}

        rule letter1() -> String
            = l:$(['a'..='z']) { l.to_string() }

        rule letter2(a: String) -> bool
            = (:a) { true  }


        }
    }
    Ok(parser::line(s)?)
}
