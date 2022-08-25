use std::str::FromStr;

use bencher_json::report::{
    JsonNewPerf,
    JsonReportAlerts,
};
use diesel::{
    Insertable,
    Queryable,
    SqliteConnection,
};
use dropshot::HttpError;
use uuid::Uuid;

use crate::{
    db::{
        schema,
        schema::perf as perf_table,
    },
    diesel::{
        ExpressionMethods,
        QueryDsl,
        RunQueryDsl,
    },
    util::http_error,
};

pub mod latency;
pub mod min_max_avg;
pub mod threshold;
pub mod throughput;

pub use latency::InsertLatency;
pub use min_max_avg::InsertMinMaxAvg;
pub use throughput::InsertThroughput;

use self::threshold::{
    PerfAlerts,
    PerfThresholds,
};
use super::benchmark::{
    InsertBenchmark,
    QueryBenchmark,
};

const PERF_ERROR: &str = "Failed to get perf.";

#[derive(Queryable)]
pub struct QueryPerf {
    pub id: i32,
    pub uuid: String,
    pub report_id: i32,
    pub iteration: i32,
    pub benchmark_id: i32,
    pub latency_id: Option<i32>,
    pub throughput_id: Option<i32>,
    pub compute_id: Option<i32>,
    pub memory_id: Option<i32>,
    pub storage_id: Option<i32>,
}

impl QueryPerf {
    pub fn get_id(conn: &SqliteConnection, uuid: impl ToString) -> Result<i32, HttpError> {
        schema::perf::table
            .filter(schema::perf::uuid.eq(uuid.to_string()))
            .select(schema::perf::id)
            .first(conn)
            .map_err(|_| http_error!(PERF_ERROR))
    }

    pub fn get_uuid(conn: &SqliteConnection, id: i32) -> Result<Uuid, HttpError> {
        let uuid: String = schema::perf::table
            .filter(schema::perf::id.eq(id))
            .select(schema::perf::uuid)
            .first(conn)
            .map_err(|_| http_error!(PERF_ERROR))?;
        Uuid::from_str(&uuid).map_err(|_| http_error!(PERF_ERROR))
    }
}

#[derive(Insertable)]
#[table_name = "perf_table"]
pub struct InsertPerf {
    pub uuid:          String,
    pub report_id:     i32,
    pub iteration:     i32,
    pub benchmark_id:  i32,
    pub latency_id:    Option<i32>,
    pub throughput_id: Option<i32>,
    pub compute_id:    Option<i32>,
    pub memory_id:     Option<i32>,
    pub storage_id:    Option<i32>,
}

impl InsertPerf {
    pub fn from_json(
        conn: &SqliteConnection,
        project_id: i32,
        report_id: i32,
        iteration: i32,
        benchmark_name: String,
        json_perf: JsonNewPerf,
        perf_thresholds: &PerfThresholds,
    ) -> Result<(Uuid, JsonReportAlerts), HttpError> {
        let mut perf_alerts = None;

        let benchmark_id = if let Ok(benchmark_id) =
            QueryBenchmark::get_id_from_name(conn, project_id, &benchmark_name)
        {
            // Only generate alerts if the benchmark already exists
            // and a threshold is provided.
            // Note these alerts have not yet been committed to the database.
            perf_alerts = Some(perf_thresholds.alerts(conn, benchmark_id)?);
            benchmark_id
        } else {
            let insert_benchmark = InsertBenchmark::new(project_id, benchmark_name);
            diesel::insert_into(schema::benchmark::table)
                .values(&insert_benchmark)
                .execute(conn)
                .map_err(|_| http_error!("Failed to create benchmark."))?;

            schema::benchmark::table
                .filter(schema::benchmark::uuid.eq(&insert_benchmark.uuid))
                .select(schema::benchmark::id)
                .first::<i32>(conn)
                .map_err(|_| http_error!("Failed to create benchmark."))?
        };

        let perf_uuid = Uuid::new_v4();
        let insert_perf = InsertPerf {
            uuid: perf_uuid.to_string(),
            report_id,
            iteration,
            benchmark_id,
            latency_id: InsertLatency::map_json(conn, json_perf.latency)?,
            throughput_id: InsertThroughput::map_json(conn, json_perf.throughput)?,
            compute_id: InsertMinMaxAvg::map_json(conn, json_perf.compute)?,
            memory_id: InsertMinMaxAvg::map_json(conn, json_perf.memory)?,
            storage_id: InsertMinMaxAvg::map_json(conn, json_perf.storage)?,
        };
        diesel::insert_into(schema::perf::table)
            .values(&insert_perf)
            .execute(conn)
            .map_err(|_| http_error!("Failed to create benchmark data."))?;
        let perf_id = QueryPerf::get_id(conn, &perf_uuid)?;

        // Commit the alerts to the database now that the perf exists
        let report_alerts = perf_alerts
            .into_iter()
            .filter_map(|perf_alert| perf_alert.into_report_alert(conn, perf_id).ok())
            .collect();

        Ok((perf_uuid, report_alerts.unwrap_or_default()))
    }
}
