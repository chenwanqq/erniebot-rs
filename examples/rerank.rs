use erniebot_rs::reranker::{RerankerEndpoint, RerankerModel};
use tokio::runtime::Runtime;

fn test_reranker() {
    let reranker = RerankerEndpoint::new(RerankerModel::BceRerankerBaseV1).unwrap();
    let query = "你好".to_string();
    let documents = vec![
        "你好".to_string(),
        "你叫什么名字".to_string(),
        "你是谁".to_string(),
    ];
    let reranker_response = reranker.invoke(query, documents, None, None).unwrap();
    let reranker_results = reranker_response.get_reranker_response().unwrap();
    let reranked_documents = reranker_results.into_iter().map(|x|x.document).collect::<Vec<String>>();
    println!("{},{:?}", reranked_documents.len(), reranked_documents);
}

fn test_async_reranker() {
    let reranker = RerankerEndpoint::new(RerankerModel::BceRerankerBaseV1).unwrap();
    let query = "你好".to_string();
    let documents = vec![
        "你好".to_string(),
        "你叫什么名字".to_string(),
        "你是谁".to_string(),
    ];
    let rt = Runtime::new().unwrap();
    let reranker_response = rt
        .block_on(reranker.ainvoke(query, documents, None, None))
        .unwrap();
    let reranker_results = reranker_response.get_reranker_response().unwrap();
    println!("{},{:?}", reranker_results.len(), reranker_results);
}

fn main() {
    test_reranker();
    test_async_reranker();
}
