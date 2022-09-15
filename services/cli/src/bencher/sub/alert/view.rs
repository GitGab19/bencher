use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_json::ResourceId;
use uuid::Uuid;

use crate::{
    bencher::{backend::Backend, sub::SubCmd, wide::Wide},
    cli::alert::CliAlertView,
    CliError,
};

#[derive(Debug)]
pub struct View {
    pub project: ResourceId,
    pub alert: Uuid,
    pub backend: Backend,
}

impl TryFrom<CliAlertView> for View {
    type Error = CliError;

    fn try_from(view: CliAlertView) -> Result<Self, Self::Error> {
        let CliAlertView {
            project,
            alert,
            backend,
        } = view;
        Ok(Self {
            project,
            alert,
            backend: backend.try_into()?,
        })
    }
}

#[async_trait]
impl SubCmd for View {
    async fn exec(&self, _wide: &Wide) -> Result<(), CliError> {
        self.backend
            .get(&format!(
                "/v0/projects/{}/alerts/{}",
                self.project, self.alert
            ))
            .await?;
        Ok(())
    }
}
