use bencher_json::JsonMetric;

use crate::{
    model::project::threshold::{alert::Side, statistic::StatisticKind},
    ApiError,
};

use super::{
    data::MetricsData,
    limits::{Limits, TestKind},
};

#[derive(Default)]
pub struct Boundary {
    pub limits: Limits,
    pub outlier: Option<Side>,
}

impl Boundary {
    pub fn new(
        metric: JsonMetric,
        metrics_data: MetricsData,
        test: StatisticKind,
        min_sample_size: Option<i64>,
        left_side: Option<f64>,
        right_side: Option<f64>,
    ) -> Result<Self, ApiError> {
        Self::new_inner(
            metric,
            metrics_data,
            test,
            min_sample_size,
            left_side,
            right_side,
        )
        .map(|opt| opt.unwrap_or_default())
    }

    fn new_inner(
        metric: JsonMetric,
        metrics_data: MetricsData,
        test: StatisticKind,
        min_sample_size: Option<i64>,
        left_side: Option<f64>,
        right_side: Option<f64>,
    ) -> Result<Option<Self>, ApiError> {
        let data = &metrics_data.data;

        // If there is a min sample size, then check to see if it is met.
        // Otherwise, simply return.
        if let Some(min_sample_size) = min_sample_size {
            if data.len() < min_sample_size as usize {
                return Ok(None);
            }
        }

        // Get the mean and standard deviation of the historical data.
        let Some(mean) = mean(data) else {
            return Ok(None)
        };
        let Some(std_dev) = std_deviation(mean, data) else {
            return Ok(None);
        };

        let test_kind = match test {
            StatisticKind::Z => TestKind::Z,
            // T test requires the degrees of freedom to calculate.
            StatisticKind::T => TestKind::T {
                freedom: (data.len() - 1) as f64,
            },
        };
        let limits = Limits::new(mean, std_dev, test_kind, left_side, right_side)?;

        let datum = metric.value.into();
        let outlier = limits.outlier(datum);

        Ok(Some(Self { limits, outlier }))
    }
}

#[allow(clippy::cast_precision_loss, clippy::float_arithmetic)]
fn mean(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    Some(data.iter().sum::<f64>() / data.len() as f64)
}

fn std_deviation(mean: f64, data: &[f64]) -> Option<f64> {
    variance(mean, data).map(f64::sqrt)
}

#[allow(clippy::cast_precision_loss, clippy::float_arithmetic)]
fn variance(mean: f64, data: &[f64]) -> Option<f64> {
    // Do not calculate variance if there are less than 2 data points
    if data.len() < 2 {
        return None;
    }
    Some(
        data.iter()
            .map(|value| (*value - mean).powi(2))
            .sum::<f64>()
            / data.len() as f64,
    )
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    const DATA_ZERO: [f64; 0] = [];
    const DATA_ONE: [f64; 1] = [1.0];
    const DATA_TWO: [f64; 2] = [1.0, 2.0];
    const DATA_THREE: [f64; 3] = [1.0, 2.0, 3.0];
    const DATA_FIVE: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];

    const MEAN_ZERO: f64 = 0.0;
    const MEAN_ONE: f64 = 1.0;
    const MEAN_TWO: f64 = 1.5;
    const MEAN_THREE: f64 = 2.0;
    const MEAN_FIVE: f64 = 3.0;

    #[test]
    fn test_mean_zero() {
        let mean = super::mean(&DATA_ZERO);
        assert_eq!(mean, None);
    }

    #[test]
    fn test_mean_one() {
        let mean = super::mean(&DATA_ONE).unwrap();
        assert_eq!(mean, MEAN_ONE);
    }

    #[test]
    fn test_mean_two() {
        let mean = super::mean(&DATA_TWO).unwrap();
        assert_eq!(mean, MEAN_TWO);
    }

    #[test]
    fn test_mean_three() {
        let mean = super::mean(&DATA_THREE).unwrap();
        assert_eq!(mean, MEAN_THREE);
    }

    #[test]
    fn test_mean_five() {
        let mean = super::mean(&DATA_FIVE).unwrap();
        assert_eq!(mean, MEAN_FIVE);
    }

    #[test]
    fn test_variance_zero() {
        let variance = super::variance(MEAN_ZERO, &DATA_ZERO);
        assert_eq!(variance, None);

        let variance = super::variance(MEAN_ONE, &DATA_ZERO);
        assert_eq!(variance, None);

        let variance = super::variance(MEAN_TWO, &DATA_ZERO);
        assert_eq!(variance, None);

        let variance = super::variance(MEAN_THREE, &DATA_ZERO);
        assert_eq!(variance, None);

        let variance = super::variance(MEAN_FIVE, &DATA_ZERO);
        assert_eq!(variance, None);
    }

    #[test]
    fn test_variance_one() {
        let variance = super::variance(MEAN_ZERO, &DATA_ONE);
        assert_eq!(variance, None);

        let variance = super::variance(MEAN_ONE, &DATA_ONE);
        assert_eq!(variance, None);

        let variance = super::variance(MEAN_TWO, &DATA_ONE);
        assert_eq!(variance, None);

        let variance = super::variance(MEAN_THREE, &DATA_ONE);
        assert_eq!(variance, None);

        let variance = super::variance(MEAN_FIVE, &DATA_ONE);
        assert_eq!(variance, None);
    }

    #[test]
    fn test_variance_two() {
        let variance = super::variance(MEAN_ZERO, &DATA_TWO).unwrap();
        assert_eq!(variance, 2.5);

        let variance = super::variance(MEAN_ONE, &DATA_TWO).unwrap();
        assert_eq!(variance, 0.5);

        let variance = super::variance(MEAN_TWO, &DATA_TWO).unwrap();
        assert_eq!(variance, 0.25);

        let variance = super::variance(MEAN_THREE, &DATA_TWO).unwrap();
        assert_eq!(variance, 0.5);

        let variance = super::variance(MEAN_FIVE, &DATA_TWO).unwrap();
        assert_eq!(variance, 2.5);
    }

    #[test]
    fn test_variance_three() {
        let variance = super::variance(MEAN_ZERO, &DATA_THREE).unwrap();
        assert_eq!(variance, 4.666666666666667);

        let variance = super::variance(MEAN_ONE, &DATA_THREE).unwrap();
        assert_eq!(variance, 1.6666666666666667);

        let variance = super::variance(MEAN_TWO, &DATA_THREE).unwrap();
        assert_eq!(variance, 0.9166666666666666);

        let variance = super::variance(MEAN_THREE, &DATA_THREE).unwrap();
        assert_eq!(variance, 0.6666666666666666);

        let variance = super::variance(MEAN_FIVE, &DATA_THREE).unwrap();
        assert_eq!(variance, 1.6666666666666667);
    }

    #[test]
    fn test_variance_five() {
        let variance = super::variance(MEAN_ZERO, &DATA_FIVE).unwrap();
        assert_eq!(variance, 11.0);

        let variance = super::variance(MEAN_ONE, &DATA_FIVE).unwrap();
        assert_eq!(variance, 6.0);

        let variance = super::variance(MEAN_TWO, &DATA_FIVE).unwrap();
        assert_eq!(variance, 4.25);

        let variance = super::variance(MEAN_THREE, &DATA_FIVE).unwrap();
        assert_eq!(variance, 3.0);

        let variance = super::variance(MEAN_FIVE, &DATA_FIVE).unwrap();
        assert_eq!(variance, 2.0);
    }

    #[test]
    fn test_std_dev_zero() {
        let variance = super::std_deviation(MEAN_ZERO, &DATA_ZERO);
        assert_eq!(variance, None);

        let variance = super::std_deviation(MEAN_ONE, &DATA_ZERO);
        assert_eq!(variance, None);

        let variance = super::std_deviation(MEAN_TWO, &DATA_ZERO);
        assert_eq!(variance, None);

        let variance = super::std_deviation(MEAN_THREE, &DATA_ZERO);
        assert_eq!(variance, None);

        let variance = super::std_deviation(MEAN_FIVE, &DATA_ZERO);
        assert_eq!(variance, None);
    }

    #[test]
    fn test_std_dev_one() {
        let variance = super::std_deviation(MEAN_ZERO, &DATA_ONE);
        assert_eq!(variance, None);

        let variance = super::std_deviation(MEAN_ONE, &DATA_ONE);
        assert_eq!(variance, None);

        let variance = super::std_deviation(MEAN_TWO, &DATA_ONE);
        assert_eq!(variance, None);

        let variance = super::std_deviation(MEAN_THREE, &DATA_ONE);
        assert_eq!(variance, None);

        let variance = super::std_deviation(MEAN_FIVE, &DATA_ONE);
        assert_eq!(variance, None);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_std_dev_two() {
        let variance = super::std_deviation(MEAN_ZERO, &DATA_TWO).unwrap();
        assert_eq!(variance, 1.5811388300841898);

        let variance = super::std_deviation(MEAN_ONE, &DATA_TWO).unwrap();
        assert_eq!(variance, 0.7071067811865476);

        let variance = super::std_deviation(MEAN_TWO, &DATA_TWO).unwrap();
        assert_eq!(variance, 0.5);

        let variance = super::std_deviation(MEAN_THREE, &DATA_TWO).unwrap();
        assert_eq!(variance, 0.7071067811865476);

        let variance = super::std_deviation(MEAN_FIVE, &DATA_TWO).unwrap();
        assert_eq!(variance, 1.5811388300841898);
    }

    #[test]
    fn test_std_dev_three() {
        let variance = super::std_deviation(MEAN_ZERO, &DATA_THREE).unwrap();
        assert_eq!(variance, 2.160246899469287);

        let variance = super::std_deviation(MEAN_ONE, &DATA_THREE).unwrap();
        assert_eq!(variance, 1.2909944487358056);

        let variance = super::std_deviation(MEAN_TWO, &DATA_THREE).unwrap();
        assert_eq!(variance, 0.9574271077563381);

        let variance = super::std_deviation(MEAN_THREE, &DATA_THREE).unwrap();
        assert_eq!(variance, 0.816496580927726);

        let variance = super::std_deviation(MEAN_FIVE, &DATA_THREE).unwrap();
        assert_eq!(variance, 1.2909944487358056);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_std_dev_five() {
        let variance = super::std_deviation(MEAN_ZERO, &DATA_FIVE).unwrap();
        assert_eq!(variance, 3.3166247903554);

        let variance = super::std_deviation(MEAN_ONE, &DATA_FIVE).unwrap();
        assert_eq!(variance, 2.449489742783178);

        let variance = super::std_deviation(MEAN_TWO, &DATA_FIVE).unwrap();
        assert_eq!(variance, 2.0615528128088303);

        let variance = super::std_deviation(MEAN_THREE, &DATA_FIVE).unwrap();
        assert_eq!(variance, 1.7320508075688772);

        let variance = super::std_deviation(MEAN_FIVE, &DATA_FIVE).unwrap();
        assert_eq!(variance, 1.4142135623730951);
    }
}
