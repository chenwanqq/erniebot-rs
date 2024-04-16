use erniebot_rs::embedding::{EmbeddingEndpoint, EmbeddingModel};
use tokio::runtime::Runtime;
fn test_embedding() {
    let embedding = EmbeddingEndpoint::new(EmbeddingModel::EmbeddingV1).unwrap();
    let input = vec![
        "你好".to_string(),
        "你叫什么名字".to_string(),
        "你是谁".to_string(),
    ];
    let embedding_response = embedding.invoke(&input, None).unwrap();
    let embedding_results = embedding_response.get_embedding_results().unwrap();
    println!("{},{}", embedding_results.len(), embedding_results[0].len());
}
fn test_async_embedding() {
    let embedding = EmbeddingEndpoint::new(EmbeddingModel::EmbeddingV1).unwrap();
    let input = vec![
        "你好".to_string(),
        "你叫什么名字".to_string(),
        "你是谁".to_string(),
    ];
    let rt = Runtime::new().unwrap();
    let embedding_response = rt.block_on(embedding.ainvoke(&input, None)).unwrap();
    let embedding_results = embedding_response.get_embedding_results().unwrap();
    println!("{},{}", embedding_results.len(), embedding_results[0].len());
}

fn main() {
    test_embedding();
    //sleep to avoid qps
    std::thread::sleep(std::time::Duration::from_secs(1));
    test_async_embedding();
}
