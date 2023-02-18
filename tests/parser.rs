#[cfg(test)]
mod parse_calc_tests {
    use stable_diffusion_prompt_parser::parser::{cst::PromptCST, parser::ParseResult};

    #[test]
    fn parse_sample() {
        let content = "((a)";
        let result = ParseResult::parse(content);
        let pairs = result.unwrap().pairs;
        // println!("{:#?}", pairs);
        let cst = PromptCST::parse(pairs.clone());
        println!("{}", serde_json::to_string_pretty(&cst).unwrap());
    }

    #[test]
    fn parse_sample1() {
        let content = include_str!("./data/sample1.txt");
        let prompts: Vec<&str> = content
            .split("---")
            .map(|prompt| prompt.trim())
            .filter(|prompt| prompt.len() > 0)
            .collect();

        prompts.iter().for_each(|prompt| {
            let result = ParseResult::parse(prompt);
            if !result.is_ok() {
                println!("{}", prompt);
            }

            assert!(result.is_ok());
        });
    }
}
