// 用途：展示如何使用 SolAgent 实现链上数据的 RAG（检索增强生成），结合 LLM 查询 Solana 交易历史。  

use solagent::{
    tool_system::ToolRegistry,
    memory_system::{ShortTermMemory, LongTermMemory},
    llm_integration::LLMClient,
    solana_integration::SolanaRPC,
};

#[tokio::main]
async fn main() {
    let rpc = SolanaRPC::new("https://api.devnet.solana.com");
    let memory = LongTermMemory::new("solagent.db");
    let llm = LLMClient::new("grok", "https://api.x.ai");
    let registry = ToolRegistry::new();
    registry.register_tool("get_transaction", GetTransactionTool::new(rpc));

    let query = "Find my recent SOL transactions";
    let context = memory.retrieve("transaction_history");
    let prompt = format!("Query: {}\nContext: {}", query, context);
    let response = llm.generate(&prompt).await.unwrap();
    println!("RAG Response: {}", response);
}