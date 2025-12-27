use std::sync::Arc;

use crate::core::agents::{planner_agent::PlannerAgent, types::GeneratedRoadmap};

#[derive(Clone)]
pub struct PlannerService {
    planner_agent: Arc<PlannerAgent>,
}

impl PlannerService {
    pub fn new(planner_agent: Arc<PlannerAgent>) -> Self {
        Self { planner_agent }
    }

    pub async fn generate_roadmap(
        &self,
        goal: &str,
    ) -> Result<GeneratedRoadmap, Box<dyn std::error::Error>> {
        self.planner_agent.generate_roadmap(goal).await
    }
}
