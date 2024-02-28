use erniebot_rs::chat::{ChatEndpoint, ChatModel, ChatOpt, Example, Function, Message, Role};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize, PartialEq)]
struct WeatherParameters {
    place: String,
}

fn get_weather_mock(params: WeatherParameters) -> String {
    format!("The weather in {} is sunny", params.place)
}

fn main() {
    let messages = vec![Message {
        role: Role::User,
        content: "What's the weather in Beijing?".to_string(),
        ..Default::default()
    }];
    let examples = vec![vec![
        Example {
            role: "user".to_string(),
            content: Some("What's the weather in Shanghai?".to_string()),
            ..Default::default()
        },
        Example {
            role: "assistant".to_string(),
            content: None,
            name: None,
            function_call: Some(erniebot_rs::chat::FunctionCall {
                name: "weather".to_string(),
                arguments: r#"{"place":"Shanghai"}"#.to_string(),
                thoughts: Some(
                    "I'm calling the weather function to get weather of Shanghai".to_string(),
                ),
            }),
        },
    ]];
    let weather_function = Function {
        name: "weather".to_string(),
        description: "Get the weather of a place".to_string(),
        parameters: schema_for!(WeatherParameters),
        examples: Some(examples),
        ..Default::default()
    };
    let functions = vec![weather_function];
    let options = vec![ChatOpt::Functions(functions)];
    let chat = ChatEndpoint::new(ChatModel::ErnieBot).unwrap();
    let response = chat.invoke(messages, options).unwrap();
    let text_response = response.get_chat_result().unwrap();
    let function_call = response.get_function_call().unwrap();
    println!("text_response: {}", text_response);
    println!("function_call: {:?}", function_call);
    let weather_params: WeatherParameters = serde_json::from_str(&function_call.arguments).unwrap();
    let weather_result = get_weather_mock(weather_params);
    println!("weather_result: {}", weather_result);
}
