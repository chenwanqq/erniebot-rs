# erniebot-rs

Unofficial Ernie Rust SDK, currently supporting three modules: chat, text embedding (embedding), and text-to-image generation (text2image).

# Installation

Add the following to your Cargo.toml file:

```toml
[dependencies]
erniebot-rs = {git = "https://github.com/chenwanqq/erniebot-rs"}
```

## Authentication
Before using, import AK and SK into environment variables:

```bash
export QIANFAN_AK=***  
export QIANFAN_SK=***
```
## Chat

Currently supported models by default include:

* ErnieBotTurbo
* ErnieBot
* Ernie40

These models can be invoked in the following manner:

```rust
fn test_invoke() {  
    let chat = ChatEndpoint::new(ChatModel::ErnieBotTurbo).unwrap();  
    let messages = vec![  
        Message {  
            role: Role::User,  
            content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using invoke method".to_string(),  
            name: None,  
        },  
    ];  
    let options = vec![  
        ChatOpt::Temperature(0.5),  
        ChatOpt::TopP(0.5),  
        ChatOpt::TopK(50),  
    ];  
    let response = chat.invoke(messages, options).unwrap();  
    let result = response.get_chat_result().unwrap();  
    println!("{}", result);  
}
```

For other models, use new_with_custom_endpoint to invoke. The field name is the last part of the Qianfan API. Taking llama_2_70b as an example, the API address is https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/llama_2_70b. The invocation method is as follows:

```rust
fn test_custom_endpoint() {  
    let chat = ChatEndpoint::new_with_custom_endpoint("llama_2_70b").unwrap();  
    let messages = vec![  
        Message {  
            role: Role::User,  
            content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using a custom endpoint".to_string(),  
            name: None,  
        },  
    ];  
    let options = Vec::new();  
    let response = chat.invoke(messages, options).unwrap();  
    let result = response.get_chat_result().unwrap();  
    println!("{}", result);  
}
```

Supports four invocation methods: invoke (synchronous non-streaming), ainvoke (asynchronous non-streaming), stream (synchronous streaming), and astream (asynchronous streaming).

For example, the astream invocation method is as follows:

``` rust
fn test_astream() {  
    let chat = ChatEndpoint::new(ChatModel::ErnieBotTurbo).unwrap();  
    let messages = vec![  
        Message {  
            role: Role::User,  
            content: "hello, I'm a developer. I'm developing a rust SDK for qianfan LLM. If you get this message, that means I successfully send you this message using async stream method. Now reply to me a message as long as possible so that I can test if this function doing well".to_string(),  
            name: None,  
        },  
    ];  
    let options = Vec::new();  
    let rt = Runtime::new().unwrap();  
    rt.block_on(async move {  
        let mut stream_response = chat.astream(messages, options).await.unwrap();  
        while let Some(response) = stream_response.next().await {  
            let result = response.get_chat_result().unwrap();  
            print!("{}", result);  
            //flush  
            std::io::stdout().flush().unwrap();  
        }  
    });  
    println!();  
}
```

Please note that due to varying parameter requirements for each specific model, this SDK does not perform local parameter validation but instead passes the parameters to the server for validation. Therefore, if the parameters do not meet the requirements, the server will return an error message.

## Embedding

Supports the four models currently (as of 2024/02/26) available on the Qianfan platform:

* EmbeddingV1
* BgeLargeZh
* BgeLargeEn
* Tao8k

The invocation is similar to chat, supporting both invoke and ainvoke modes:

```rust
fn test_async_embedding() {  
    let embedding = EmbeddingEndpoint::new(EmbeddingModel::EmbeddingV1).unwrap();  
    let input = vec![  
        "你好".to_string(),  
        "你叫什么名字".to_string(),  
        "你是谁".to_string(),  
    ];  
    let rt = Runtime::new().unwrap();  
    let embedding_response = rt.block_on(embedding.ainvoke(input, None)).unwrap();  
    let embedding_results = embedding_response.get_embedding_results().unwrap();  
    println!("{},{}", embedding_results.len(), embedding_results[0].len());  
}
```

## Text2Image

Supports the default StableDiffusionXL and also allows for custom models (such as Wenxin Yige).

```rust
fn main() {  
    let text2image = Text2ImageEndpoint::new(Text2ImageModel::StableDiffusionXL).unwrap();  
    let prompt = "A beautiful sunset over the ocean".to_string();  
    let mut options = Vec::new();  
    options.push(Text2ImageOpt::Style(Style::DigitalArt));  
    options.push(Text2ImageOpt::Size(Size::S1024x768));  
    let text2image_response = text2image.invoke(prompt, options).unwrap();  
    let image_results = text2image_response.get_image_results().unwrap();  
    let mut index = 0;  
    for image_string in image_results {  
        let image = base64_to_image(image_string).unwrap();  
        let filepath = format!("./tmp/image_{}.png", index);  
        image.save(filepath).unwrap();  
        index += 1;  
    }  
}
```
## TODO

* Docs
* Chat models are numerous, so options may not be complete and require further supplementation.
* More testing is needed.
* The Fuyu-8B image understanding model is not yet supported.