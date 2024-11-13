use anyhow::anyhow;
use pest::Parser;
use regex_parser::*;

#[cfg(test)]
mod regex_parser_unit_tests {
    use super::*;

    #[test]
    fn test_rsymb_on_correct() -> anyhow::Result<()> {
        let correct_inputs = ["a", "b", ".", "1", "т", "а", "н", "я"];
        for input in correct_inputs {
            let parsed_data = RegexGrammar::parse(Rule::rsymb, input)?
                .next()
                .ok_or_else(|| anyhow!("no needed pair"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_rsymb_on_incorrect() -> anyhow::Result<()> {
        let incorrect_inputs = ["|", "(", ")", "*", "+", "?", ""];
        for input in incorrect_inputs {
            let parsed_result = RegexGrammar::parse(Rule::rsymb, input);
            assert!(
                parsed_result.is_err(),
                "Expected error for input: {}",
                input
            );
        }
        Ok(())
    }

    #[test]
    fn test_prime_on_correct() -> anyhow::Result<()> {
        let correct_inputs = [
            "a", "b", ".", ",", "т", "а", "н", "я", "(a)", "(a|b)", "(a*)", "(a|(b*))",
        ];
        for input in correct_inputs {
            let parsed_data = RegexGrammar::parse(Rule::prime, input)?
                .next()
                .ok_or_else(|| anyhow!("no needed pair"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_prime_on_incorrect() -> anyhow::Result<()> {
        let incorrect_inputs = ["|a", "*b", "?.", "+,", "(т", ")а", "(н+(z)", ""];
        for input in incorrect_inputs {
            let parsed_result = RegexGrammar::parse(Rule::prime, input);
            assert!(
                parsed_result.is_err(),
                "Expected error for input: {}",
                input
            );
        }
        Ok(())
    }

    #[test]
    fn test_rfact_on_correct() -> anyhow::Result<()> {
        let correct_inputs = ["a", "b+", ".*", ",?", "(a|t)", "(ab)*", "(a*)?", "(n+|b)+"];
        for input in correct_inputs {
            let parsed_data = RegexGrammar::parse(Rule::rfact, input)?
                .next()
                .ok_or_else(|| anyhow!("no needed pair"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_rfact_on_incorrect() -> anyhow::Result<()> {
        let incorrect_inputs = ["|a", "*b", "?.", "+,", "(т", ")а", "(н+(z)", ""];
        for input in incorrect_inputs {
            let parsed_result = RegexGrammar::parse(Rule::rfact, input);
            assert!(
                parsed_result.is_err(),
                "Expected error for input: {}",
                input
            );
        }
        Ok(())
    }

    #[test]
    fn test_rterm_on_correct() -> anyhow::Result<()> {
        let correct_inputs = [
            "ab", "b+a", "bab*", "a*b?", "(a|t)", "(ab)b", "(a*)?", "(n+|b)+", "a(b+)",
        ];
        for input in correct_inputs {
            let parsed_data = RegexGrammar::parse(Rule::rterm, input)?
                .next()
                .ok_or_else(|| anyhow!("no needed pair"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_rterm_on_incorrect() -> anyhow::Result<()> {
        let incorrect_inputs = [
            "|a", "*b", "?.", "+,", "(т", ")а", "(н+(z)", "(a|())", "(ab+",
        ];
        for input in incorrect_inputs {
            let parsed_result = RegexGrammar::parse(Rule::rterm, input);
            assert!(
                parsed_result.is_err(),
                "Expected error for input: {}",
                input
            );
        }
        Ok(())
    }

    #[test]
    fn test_rexpr_on_correct() -> anyhow::Result<()> {
        let correct_inputs = ["ab", "b|a", "ba|b*", "(a*)|(b?)"];
        for input in correct_inputs {
            let parsed_data = RegexGrammar::parse(Rule::rexpr, input)?
                .next()
                .ok_or_else(|| anyhow!("no needed pair"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_rexpr_on_incorrect() -> anyhow::Result<()> {
        let incorrect_inputs = [
            "|a", "*b", "?.", "+,", "(т", ")а", "(н+(z)", "(a|())", "(ab+",
        ];
        for input in incorrect_inputs {
            let parsed_result = RegexGrammar::parse(Rule::rexpr, input);
            assert!(
                parsed_result.is_err(),
                "Expected error for input: {}",
                input
            );
        }
        Ok(())
    }

    #[test]
    fn test_reg_on_correct() -> anyhow::Result<()> {
        let correct_inputs = [
            "(a|b)*c",
            "(x|y)(1|2)",
            "x'*",
            "(ab|c)*",
            "(a?)a",
            "(ab)?d+",
        ];
        for input in correct_inputs {
            let parsed_data = RegexGrammar::parse(Rule::reg, input)?
                .next()
                .ok_or_else(|| anyhow!("no needed pair"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_reg_on_incorrect() -> anyhow::Result<()> {
        let incorrect_inputs = [
            "", "(", ")", "*", "+", "?", "|", "a||b", "a|", "a**", "a++", "a??", "(a|b|c", "|a|b",
            "a|b+(", "a?*", "ab(+)",
        ];
        for input in incorrect_inputs {
            let parsed_result = RegexGrammar::parse(Rule::reg, input);
            assert!(
                parsed_result.is_err(),
                "Expected error for input: {}",
                input
            );
        }
        Ok(())
    }
}
