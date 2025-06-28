# SolAgent
SolAgent SDK: The AI Agent Development Kit for Solana.

## SolAgent Framework Architecture

```
┌───────────────────────────────────────────────────┐
│                    User Interface                 │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  │
│  │    CLI     │  │    Web     │  │    API     │  │
│  └────────────┘  └────────────┘  └────────────┘  │
└───────────────────────────────────────────────────┘
           ↑ (Commands/Requests)
┌───────────────────────────────────────────────────┐
│                 SolAgent (Config)                 │
│  (Central entry point with SolAgentConfig:       │
│   name, instructions, model, tools)              │
└───────────────────────────────────────────────────┘
           ↑ (Task decomposition)
┌───────────────────────────────────────────────────┐
│                 Agent Controller                  │
│  (Task scheduling, multi-agent, LLM coordination) │
└───────────────────────────────────────────────────┘
           ↑ (Planning and reasoning)
┌───────────────────────────────────────────────────┐
│              Planning & Reasoning                 │
│  ┌────────────┐  ┌────────────┐                  │
│  │ LLMPlanner │  │ RuleEngine │                  │
│  └────────────┘  └────────────┘                  │
└───────────────────────────────────────────────────┘
           ↑ (Workflow orchestration)
┌───────────────────────────────────────────────────┐
│                 Workflow Engine                   │
│  (Petgraph DAG, checkpoints, parallel execution) │
└───────────────────────────────────────────────────┘
           ↑ (Tool execution)
┌───────────────────────────────────────────────────┐
│                   Tool System                     │
│  ┌────────────┐                                  │
│  │ ToolRegistry  (50+ tools: account, defi, nft,  │
│  └────────────┘  staking, transaction, misc)      │
└───────────────────────────────────────────────────┘
           ↑ (Context storage/retrieval)
┌───────────────────────────────────────────────────┐
│                  Memory System                    │
│  ┌────────────┐  ┌────────────┐                  │
│  │ ShortTerm  │  │ LongTerm   │ (RwLock, SQLite) │
│  └────────────┘  └────────────┘                  │
└───────────────────────────────────────────────────┘
           ↑ (Prompts and API calls)
┌───────────────────────────────────────────────────┐
│                 LLM Integration                   │
│  ┌────────────┐  ┌────────────┐                  │
│  │ Prompt     │  │ LLMClient  │ (Grok3, Gemini,  │
│  └────────────┘  └────────────┘  OpenAI)         │
└───────────────────────────────────────────────────┘
           ↑ (Permission checks)
┌───────────────────────────────────────────────────┐
│              Security & Permission                │
│  ┌────────────┐  ┌────────────┐                  │
│  │    RBAC    │  │    ABAC    │ (Casbin)         │
│  └────────────┘  └────────────┘                  │
└───────────────────────────────────────────────────┘
           ↑ (Chain data access)
┌───────────────────────────────────────────────────┐
│              Solana RPC & Indexer                │
│  ┌────────────┐  ┌────────────┐                  │
│  │ SolanaRPC  │  │ IndexerClient │ (Multi-RPC,   │
│  └────────────┘  └────────────┘  Helius)         │
└───────────────────────────────────────────────────┘
           ↑ (Logs and metrics)
┌───────────────────────────────────────────────────┐
│                  Observability                    │
│  ┌────────────┐  ┌────────────┐                  │
│  │  Logging   │  │ Monitoring │ (Tracing, Prometheus) │
│  └────────────┘  └────────────┘                  │
└───────────────────────────────────────────────────┘
```


## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=solagentlabs/solagent-rs&type=Date)](https://www.star-history.com/#solagentlabs/solagent-rs&Date)

## License
Apache-2.0