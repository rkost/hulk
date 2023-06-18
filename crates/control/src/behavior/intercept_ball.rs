use nalgebra::{Isometry2, Point2, UnitComplex, Vector2};
use spl_network_messages::{GamePhase, SubState};
use types::{
    parameters::InterceptBall, Arc, BallState, Circle, FilteredGameState, GameControllerState,
    HeadMotion, LineSegment, MotionCommand, Orientation, OrientationMode, PathSegment, Step,
    WorldState,
};

pub fn execute(
    world_state: &WorldState,
    parameters: InterceptBall,
    maximum_step_size: Step,
    current_step: Step,
) -> Option<MotionCommand> {
    if let Some(
        GameControllerState {
            game_phase: GamePhase::PenaltyShootout { .. },
            ..
        }
        | GameControllerState {
            sub_state: Some(SubState::PenaltyKick),
            ..
        },
    ) = world_state.game_controller_state
    {
        return None;
    }
    match (
        world_state.filtered_game_state,
        world_state.ball,
        world_state.robot.robot_to_field,
    ) {
        (
            Some(FilteredGameState::Playing { ball_is_free: true }),
            Some(ball),
            Some(robot_to_field),
        ) => {
            if !ball_is_interception_candidate(ball, robot_to_field, parameters) {
                return None;
            }

            let Step {
                forward,
                left,
                turn: _,
            } = maximum_step_size;

            if forward == 0.0 || left == 0.0 {
                return None;
            }

            let normalized_velocity = ball.ball_in_ground_velocity.normalize();

            // Find the point with the least distance from the line traversed by the ball
            let optimal_interception_point = ball.ball_in_ground
                - ball.ball_in_ground.coords.dot(&normalized_velocity) * normalized_velocity;

            if optimal_interception_point.coords.norm() > parameters.maximum_intercept_distance {
                return None;
            }

            let walking_direction = Vector2::new(current_step.forward, current_step.left);
            let path = if walking_direction.norm() < 0.1 {
                vec![PathSegment::LineSegment(LineSegment(
                    Point2::origin(),
                    optimal_interception_point,
                ))]
            } else {
                // If we are moving, we can not change the direction instantaneously without
                // slowing down. Instead, traverse an Arc with radius dependent on the current
                // speed until the direction is changed.
                let arc_radius = walking_direction.norm();
                let arc = calculate_arc_tangent_to(
                    walking_direction,
                    optimal_interception_point.coords,
                    arc_radius,
                );
                let arc_orientation =
                    Orientation::triangle_orientation(arc.circle.center, arc.start, arc.end);

                let interception_point = optimal_interception_point + (arc.end - arc.circle.center);

                vec![
                    PathSegment::Arc(arc, arc_orientation),
                    PathSegment::LineSegment(LineSegment(arc.end, interception_point)),
                ]
            };

            Some(MotionCommand::Walk {
                head: HeadMotion::LookAt {
                    target: ball.ball_in_ground,
                    camera: None,
                },
                path,
                left_arm: types::ArmMotion::Swing,
                right_arm: types::ArmMotion::Swing,
                orientation_mode: OrientationMode::Override(UnitComplex::default()),
            })
        }
        _ => None,
    }
}

fn ball_is_interception_candidate(
    ball: BallState,
    robot_to_field: Isometry2<f32>,
    parameters: InterceptBall,
) -> bool {
    let ball_in_front_of_robot = ball.ball_in_ground.coords.norm()
        < parameters.maximum_ball_distance
        && ball.ball_in_ground.x > 0.0;
    let ball_moving_towards_robot =
        ball.ball_in_ground_velocity.x < -parameters.minimum_ball_velocity_towards_robot;

    let ball_in_field_velocity = robot_to_field * ball.ball_in_ground_velocity;
    let ball_moving = ball_in_field_velocity.norm() > parameters.minimum_ball_velocity;
    let ball_moving_towards_own_half =
        ball_in_field_velocity.x < -parameters.minimum_ball_velocity_towards_own_half;

    ball_in_front_of_robot
        && ball_moving
        && ball_moving_towards_robot
        && ball_moving_towards_own_half
}

fn calculate_arc_tangent_to(vector1: Vector2<f32>, vector2: Vector2<f32>, radius: f32) -> Arc {
    let normalized_vector1 = vector1.normalize();
    let normalized_vector2 = vector2.normalize();
    let sin1 = Vector2::new(1.0, 0.0).dot(&normalized_vector1);
    let sin2 = Vector2::new(1.0, 0.0).dot(&normalized_vector2);
    let cos1 = Vector2::new(0.0, 1.0).dot(&normalized_vector1);
    let cos2 = Vector2::new(0.0, 1.0).dot(&normalized_vector2);

    let arc_center = Point2::new(radius * -sin1, radius * cos1);
    let end_point = Point2::new(radius * sin2, radius * (1.0 - cos2));
    Arc::new(Circle::new(arc_center, radius), Point2::origin(), end_point)
}
