use std::io::Write;

use erniebot_rs::chat::{ChatEndpoint, ChatModel, ChatOpt, Message, Role};
use tokio::runtime::Runtime;
use tokio_stream::StreamExt;

fn test_invoke() {
    let chat = ChatEndpoint::new(ChatModel::ErnieBotTurbo).unwrap();
    let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using invoke method".to_string(),
                ..Default::default()
            },
        ];
    let options = vec![
        ChatOpt::Temperature(0.5),
        ChatOpt::TopP(0.5),
        ChatOpt::TopK(50),
    ];
    let response = chat.invoke(&messages, &options).unwrap();
    let result = response.get_chat_result().unwrap();
    println!("{}", result);
}

fn test_stream() {
    let chat = ChatEndpoint::new(ChatModel::ErnieBotTurbo).unwrap();
    let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using stream method".to_string(),
                ..Default::default()
            },
        ];
    let options = vec![
        ChatOpt::Temperature(0.5),
        ChatOpt::TopP(0.5),
        ChatOpt::TopK(50),
    ];
    let response = chat.stream(&messages, &options).unwrap();
    let result_by_chunk = response.get_results().unwrap();
    println!("{:?}", result_by_chunk);
    let whole_result = response.get_whole_result().unwrap();
    println!("{}", whole_result);
}

fn test_ainvoke() {
    let chat = ChatEndpoint::new(ChatModel::ErnieBotTurbo).unwrap();
    let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using ainvoke method".to_string(),
                ..Default::default()
            },
        ];
    let options = Vec::new();
    let rt = Runtime::new().unwrap();
    let response = rt.block_on(chat.ainvoke(&messages, &options)).unwrap();
    let result = response.get_chat_result().unwrap();
    println!("{}", result);
}

fn test_astream() {
    let chat = ChatEndpoint::new(ChatModel::ErnieBotTurbo).unwrap();
    let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using async stream method. Now reply to me a message as long as possible so that I can test if this function doing well".to_string(),
                ..Default::default()
            },
        ];
    let options = Vec::new();
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let mut stream_response = chat.astream(&messages, &options).await.unwrap();
        while let Some(response) = stream_response.next().await {
            let result = response.get_chat_result().unwrap();
            print!("{}", result);
            //flush
            std::io::stdout().flush().unwrap();
        }
    });
    println!();
}

fn test_custom_endpoint() {
    let chat = ChatEndpoint::new_with_custom_endpoint("llama_2_70b").unwrap();
    let messages = vec![
            Message {
                role: Role::User,
                content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using a custom endpoint".to_string(),
                ..Default::default()
            },
        ];
    let options = Vec::new();
    let response = chat.invoke(&messages, &options).unwrap();
    let result = response.get_chat_result().unwrap();
    println!("{}", result);
}

fn main() {
    test_invoke();
    test_stream();
    test_ainvoke();
    test_astream();
    test_custom_endpoint();
}
