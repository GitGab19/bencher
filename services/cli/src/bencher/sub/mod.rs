use std::convert::TryFrom;

use async_trait::async_trait;

use crate::{parser::CliSub, CliError};

#[cfg(feature = "docs")]
mod docs;
mod mock;
mod organization;
mod project;
mod sub_cmd;
mod system;
mod user;

#[cfg(feature = "docs")]
use docs::Docs;
use mock::Mock;
pub use mock::MockError;
use organization::{member::Member, organization::Organization};
pub use project::run::{runner::output::Output, RunError};
use project::{
    alert::Alert, benchmark::Benchmark, branch::Branch, metric_kind::MetricKind, perf::Perf,
    project::Project, report::Report, run::Run, statistic::Statistic, testbed::Testbed,
    threshold::Threshold,
};
pub use sub_cmd::SubCmd;
use system::{auth::Auth, server::Server};
use user::resource::User;
use user::token::Token;

#[derive(Debug)]
pub enum Sub {
    Auth(Auth),
    Organization(Organization),
    Member(Member),
    Project(Project),
    Run(Run),
    Perf(Perf),
    Report(Report),
    MetricKind(MetricKind),
    Branch(Branch),
    Testbed(Testbed),
    Benchmark(Benchmark),
    Threshold(Threshold),
    Statistic(Statistic),
    Alert(Alert),
    User(User),
    Token(Token),
    Server(Server),
    Mock(Mock),
    #[cfg(feature = "docs")]
    Docs(Docs),
}

impl TryFrom<CliSub> for Sub {
    type Error = CliError;

    fn try_from(sub: CliSub) -> Result<Self, Self::Error> {
        Ok(match sub {
            CliSub::Auth(auth) => Self::Auth(auth.try_into()?),
            CliSub::Organization(organization) => Self::Organization(organization.try_into()?),
            CliSub::Member(member) => Self::Member(member.try_into()?),
            CliSub::Project(project) => Self::Project(project.try_into()?),
            CliSub::Run(run) => Self::Run(run.try_into()?),
            CliSub::Perf(perf) => Self::Perf(perf.try_into()?),
            CliSub::Report(report) => Self::Report(report.try_into()?),
            CliSub::MetricKind(metric_kind) => Self::MetricKind(metric_kind.try_into()?),
            CliSub::Branch(branch) => Self::Branch(branch.try_into()?),
            CliSub::Testbed(testbed) => Self::Testbed(testbed.try_into()?),
            CliSub::Benchmark(benchmark) => Self::Benchmark(benchmark.try_into()?),
            CliSub::Threshold(threshold) => Self::Threshold(threshold.try_into()?),
            CliSub::Statistic(statistic) => Self::Statistic(statistic.try_into()?),
            CliSub::Alert(alert) => Self::Alert(alert.try_into()?),
            CliSub::User(user) => Self::User(user.try_into()?),
            CliSub::Token(token) => Self::Token(token.try_into()?),
            CliSub::Server(server) => Self::Server(server.try_into()?),
            CliSub::Mock(mock) => Self::Mock(mock.into()),
            #[cfg(feature = "docs")]
            CliSub::Docs(docs) => Self::Docs(docs.into()),
        })
    }
}

#[async_trait]
impl SubCmd for Sub {
    async fn exec(&self) -> Result<(), CliError> {
        match self {
            Self::Auth(auth) => auth.exec().await,
            Self::Organization(organization) => organization.exec().await,
            Self::Member(member) => member.exec().await,
            Self::Project(project) => project.exec().await,
            Self::Run(run) => run.exec().await,
            Self::Perf(perf) => perf.exec().await,
            Self::Report(report) => report.exec().await,
            Self::MetricKind(metric_kind) => metric_kind.exec().await,
            Self::Branch(branch) => branch.exec().await,
            Self::Testbed(testbed) => testbed.exec().await,
            Self::Benchmark(benchmark) => benchmark.exec().await,
            Self::Threshold(threshold) => threshold.exec().await,
            Self::Statistic(statistic) => statistic.exec().await,
            Self::Alert(alert) => alert.exec().await,
            Self::User(user) => user.exec().await,
            Self::Token(token) => token.exec().await,
            Self::Server(server) => server.exec().await,
            Self::Mock(mock) => mock.exec().await,
            #[cfg(feature = "docs")]
            Self::Docs(docs) => docs.exec().await,
        }
    }
}
