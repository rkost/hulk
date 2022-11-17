use context_attribute::context;
use framework::{MainOutput, Parameter, RequiredInput};
use types::{CameraMatrix, FilteredSegments, LineData, PerspectiveGridCandidates};

pub struct PerspectiveGridCandidatesProvider {}

#[context]
pub struct NewContext {
    pub ball_radius: Parameter<f32, "field_dimensions/ball_radius">,
    pub fallback_radius:
        Parameter<f32, "perspective_grid_candidates_provider/$cycler_instance/fallback_radius">,
    pub minimum_radius:
        Parameter<f32, "perspective_grid_candidates_provider/$cycler_instance/minimum_radius">,
}

#[context]
pub struct CycleContext {
    pub camera_matrix: RequiredInput<Option<CameraMatrix>, "camera_matrix?">,
    pub filtered_segments: RequiredInput<Option<FilteredSegments>, "filtered_segments?">,
    pub line_data: RequiredInput<Option<LineData>, "line_data?">,

    pub ball_radius: Parameter<f32, "field_dimensions/ball_radius">,
    pub fallback_radius:
        Parameter<f32, "perspective_grid_candidates_provider/$cycler_instance/fallback_radius">,
    pub minimum_radius:
        Parameter<f32, "perspective_grid_candidates_provider/$cycler_instance/minimum_radius">,
}

#[context]
#[derive(Default)]
pub struct MainOutputs {
    pub perspective_grid_candidates: MainOutput<Option<PerspectiveGridCandidates>>,
}

impl PerspectiveGridCandidatesProvider {
    pub fn new(_context: NewContext) -> anyhow::Result<Self> {
        Ok(Self {})
    }

    pub fn cycle(&mut self, _context: CycleContext) -> anyhow::Result<MainOutputs> {
        Ok(MainOutputs::default())
    }
}