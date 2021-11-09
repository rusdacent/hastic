use crate::services::{
    analytic_service::types, metric_service::MetricService, segments_service::SegmentsService,
};

use super::types::{AnalyticUnit, LearningResult, ThresholdConfig};

use async_trait::async_trait;

// TODO: move to config
const DETECTION_STEP: u64 = 10;

pub struct ThresholdAnalyticUnit {
    config: ThresholdConfig,
}

impl ThresholdAnalyticUnit {
    pub fn new(config: ThresholdConfig) -> ThresholdAnalyticUnit {
        ThresholdAnalyticUnit { config }
    }
}

#[async_trait]
impl AnalyticUnit for ThresholdAnalyticUnit {
    async fn learn(&mut self, _ms: MetricService, _ss: SegmentsService) -> LearningResult {
        return LearningResult::Finished;
    }
    async fn detect(
        &self,
        ms: MetricService,
        from: u64,
        to: u64,
    ) -> anyhow::Result<Vec<(u64, u64)>> {
        let mr = ms.query(from, to, DETECTION_STEP).await.unwrap();

        if mr.data.keys().len() == 0 {
            return Ok(Vec::new());
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[k];

        let mut result = Vec::<(u64, u64)>::new();
        let mut from: Option<u64> = None;
        for (t, v) in ts {
            if *v > self.config.threashold {
                if from.is_some() {
                    continue;
                } else {
                    from = Some(*t);
                }
            } else {
                if from.is_some() {
                    result.push((from.unwrap(), *t));
                    from = None;
                }
            }
        }

        // TODO: don't repeat myself
        if from.is_some() {
            result.push((from.unwrap(), ts.last().unwrap().0));
        }

        // TODO: decide what to do it from is Some() in the end

        Ok(result)
    }
}