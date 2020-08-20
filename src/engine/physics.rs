extern crate nalgebra as na;

use na::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

pub struct PhysicsWorld {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81));
        let geometrical_world = DefaultGeometricalWorld::<f32>::new();
        let bodies = DefaultBodySet::<f32>::new();
        let colliders = DefaultColliderSet::<f32>::new();
        let joint_constraints = DefaultJointConstraintSet::<f32>::new();
        let force_generators = DefaultForceGeneratorSet::<f32>::new();

        Self {
            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators
        }
    }

    pub fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );
    }
}