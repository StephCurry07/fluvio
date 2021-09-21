use std::sync::Arc;
use structopt::StructOpt;

mod create;
mod delete;
mod list;

use crate::error::ConnectorError as ClusterCliError;
use fluvio_extension_common::Terminal;
use fluvio_extension_common::COMMAND_TEMPLATE;

use create::CreateManagedConnectorOpt;
use delete::DeleteManagedConnectorOpt;
use list::ListManagedConnectorsOpt;

#[derive(Debug, StructOpt)]
pub enum ManagedConnectorCmd {
    /// Create a new Managed Connector
    #[structopt(
        name = "create",
        template = COMMAND_TEMPLATE,
    )]
    Create(CreateManagedConnectorOpt),

    /// Delete a Managed Connector
    #[structopt(
        name = "delete",
        template = COMMAND_TEMPLATE,
    )]
    Delete(DeleteManagedConnectorOpt),

    /// List all Managed Connectors
    #[structopt(
        name = "list",
        template = COMMAND_TEMPLATE,
    )]
    List(ListManagedConnectorsOpt),
}

impl ManagedConnectorCmd {
    pub async fn process<O: Terminal>(self, out: Arc<O>) -> Result<(), ClusterCliError> {
        let fluvio = fluvio::Fluvio::connect().await?;
        match self {
            Self::Create(create) => {
                create.process(&fluvio).await?;
            }
            Self::Delete(delete) => {
                delete.process(&fluvio).await?;
            }
            Self::List(list) => {
                list.process(out, &fluvio).await?;
            }
        }
        Ok(())
    }
}