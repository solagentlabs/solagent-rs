use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use solagent_plugin_solana::close_empty_token_accounts;
use crate::close_empty_account::tool::{CloseEmptyTokenAccountsArgs, CloseEmptyTokenAccountsOutput, CloseEmptyTokenAccountsError, CloseEmptyTokenAccounts};

impl Tool for CloseEmptyTokenAccounts {
    const NAME: &'static str = "close_empty_token_accounts";

    type Error = CloseEmptyTokenAccountsError;
    type Args = CloseEmptyTokenAccountsArgs;
    type Output = CloseEmptyTokenAccountsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "close_empty_token_accounts".to_string(),
            description: r#"
            Close empty SPL Token accounts associated with your wallet to reclaim rent. 
            This action will close both regular SPL Token accounts and Token-2022 accounts that have zero balance. 

            examples: [
                [
                    {
                        input: {},
                        output: {
                            status: "success",
                            signature:
                                "3KmPyiZvJQk8CfBVVaz8nf3c2crb6iqjQVDqNxknnusyb1FTFpXqD8zVSCBAd1X3rUcD8WiG1bdSjFbeHsmcYGXY",
                            accountsClosed: 10,
                        },
                        explanation: "Closed 10 empty token accounts successfully.",
                    },
                ],
                [
                    {
                        input: {},
                        output: {
                            status: "success",
                            signature: "",
                            accountsClosed: 0,
                        },
                        explanation: "No empty token accounts were found to close.",
                    },
                ],
            ]

"#.to_string(),
            parameters: serde_json::json!(null),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = close_empty_token_accounts(&self.agent)
            .await
            .expect("close_empty_token_accounts");

        Ok(CloseEmptyTokenAccountsOutput { data })
    }
}