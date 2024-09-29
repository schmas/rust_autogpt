use proc_macro::function_to_string;

#[allow(dead_code)]
const OUTPUT: &str = "";

#[function_to_string]
fn some_function_to_ai_llm(_whatever_param: &str) {
    /// This is an awesome function
    /// We shall give it to an AI to guess and output
    /// In a structured manner
    print!("{}", OUTPUT)
}

#[test]
fn test_proc_macro_001() {
    let x: &str = some_function_to_ai_llm("whatever_param");
    dbg!(x);
    assert_eq!(
        x,
        "fn some_function_to_ai_llm(_whatever_param : & str)\n{\n    #[doc = \" This is an awesome function\"]\n    #[doc = \" We shall give it to an AI to guess and output\"]\n    #[doc = \" In a structured manner\"] print! (\"{}\", OUTPUT)\n}"
    );
}
