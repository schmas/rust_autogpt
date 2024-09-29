use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;

// Extend ai function to encourage certain specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg = format!(
        "FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function {}.
    Print out what the function will return.",
        ai_function_str, func_input
    );

    // Return Message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Perform call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_msg = extend_ai_function(function_pass, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // get LLM response
    let llm_response = call_gpt(vec![extended_msg.clone()]).await;

    // Handle success
    match llm_response {
        Ok(res) => res,
        Err(_) => call_gpt(vec![extended_msg])
            .await
            .expect("Failed twice to call OpenAI"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let message = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        assert_eq!(message.role, "system");
    }

    #[tokio::test]
    #[ignore]
    async fn tests_ai_task_request() {
        let ai_func_param = "Build me a webserver for making stock price api request".to_string();
        let agent_position = "Managing Agent";
        let agent_operation = "Defining use requirement";

        let result = ai_task_request(
            ai_func_param,
            agent_position,
            agent_operation,
            convert_user_input_to_goal,
        )
        .await;

        dbg!(&result);

        assert!(result.len() > 0);
    }
}
