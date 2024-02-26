# erniebot-rs

非官方的ernie rust SDK, 目前支持对话(chat)，文本嵌入(embedding)以及文生图(text2image)三个模块。

## 安装  
  
在`Cargo.toml`文件中添加以下内容：  
  
```toml  
[dependencies]  
erniebot-rs = {git = "https://github.com/chenwanqq/erniebot-rs"}


## 鉴权

使用前，将AK，SK导入到环境变量中：
```bash
    export QIANFAN_AK=***
    export QIANFAN_SK=***
```

## chat

目前默认支持的模型有：
* ErnieBotTurbo,
* ErnieBot
* Ernie40

这些模型可以用这样的方式来调用：

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

其它模型，用new_with_custom_endpoint来调用，字段名称是千帆API的最后一部分，以llama_2_70b为例，API地址为https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/llama_2_70b, 则调用方式如下：

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

支持invoke(同步非流式),ainvoke(异步非流式),stream(同步流式),astream(异步流式)四种调用方式。

例如，astream调用方法为：

```rust
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

注意，由于各个具体模型对参数的要求不同，所以本SDK并未在本地进行参数校验，而是将参数传递给服务端进行校验。因此，如果参数不符合要求，服务端会返回错误信息。

## embedding

支持目前(2024/02/26)千帆平台的四种模型：
* EmbeddingV1,
* BgeLargeZh,
* BgeLargeEn,
* Tao8k

调用方式类似于chat，支持invoke和ainvoke两种模式：

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

## text2image

支持默认的StableDiffusionXL，也支持自定义模型（文心一格）

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

* 完善文档
* chat模型比较多，所以option可能还不够完整，还需要进行补充
* 更多的测试
* Fuyu-8B图像理解模型还未支持