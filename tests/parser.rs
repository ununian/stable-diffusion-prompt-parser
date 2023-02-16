#[cfg(test)]
mod parse_calc_tests {
    use stable_diffusion_prompt_parser::parser::parser::ParseResult;

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
