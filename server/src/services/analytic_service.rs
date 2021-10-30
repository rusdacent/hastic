use crate::{utils::get_random_str};

use super::{metric_service::MetricService, segments_service::{self, ID_LENGTH, Segment, SegmentType, SegmentsService}};

use subbeat::metric::Metric;

use anyhow;

mod pattern_detector;


#[derive(Clone)]
pub struct AnalyticService {
    metric_service: MetricService,
    segments_service: SegmentsService
}

impl AnalyticService {
    pub fn new(metric_service: MetricService, segments_service: segments_service::SegmentsService) -> AnalyticService {
        AnalyticService {
            metric_service,
            segments_service
        }
    }

    pub async fn get_pattern_detection() -> anyhow::Result<Vec<Segment>> {
        // TODO: get segments
        // TODO: get reads from segments
        // TODO: run learn
        // TODO: run detections
        // TODO: convert detections to segments
        Ok(Vec::new())
    }

    pub async fn get_threshold_detections(
        &self,
        from: u64,
        to: u64,
        step: u64,
        threashold: f64
    ) -> anyhow::Result<Vec<Segment>> {
        let prom = self.metric_service.get_prom();
        let mr = prom.query(from, to, step).await?;

        if mr.data.keys().len() == 0 {
            return Ok(Vec::new());
        }

        let key = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[key];

        let mut result = Vec::<Segment>::new();
        let mut from: Option<u64> = None;
        for (t, v) in ts {
            if *v > threashold {
                if from.is_some() {
                    continue;
                } else {
                    from = Some(*t);
                }
            } else {
                if from.is_some() {
                    result.push(Segment {
                        // TODO: persist detections together with id
                        id: Some(get_random_str(ID_LENGTH)),
                        from: from.unwrap(),
                        to: *t,
                        segment_type: SegmentType::Detection,
                    });
                    from = None;
                }
            }
        }

        // TODO: don't repeat myself
        if from.is_some() {
            result.push(Segment {
                id: Some(get_random_str(ID_LENGTH)),
                from: from.unwrap(),
                to,
                segment_type: SegmentType::Detection,
            });
        }

        // TODO: decide what to do it from is Some() in the end

        Ok(result)
    }
}
