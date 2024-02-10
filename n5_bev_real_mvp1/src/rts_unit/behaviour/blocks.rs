#[derive(Bundle)]
pub struct BasicControlledBehaviour {
    to_root: ToRTSUnitRoot,

    controlled_navigation: BasicControlled,
    to_attack_target: ToAttackTargetDetection,
    to_attack_arbitrary: ToAttackArbitraryDetection,

    transform: TransformBundle,
}

commands.entity(behaviour).insert(Behaviour{
    to_root: ToRTSUnitRoot::new(root),

    controlled_navigation: BasicControlled,
    to_attack_target: ToAttackTargetDetection::new(attack_detection),
    to_attack_arbitrary: ToAttackArbitraryDetection::new(attack_detection),

    transform: TransformBundle::default(),
});