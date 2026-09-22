import {defineProjectTestSuite, projectSteps, projectTestCase} from "../../framework"

export default defineProjectTestSuite({
    id: "flins-thunderous-symphony",
    name: "菲林斯雷霆交响测试集",
    description: "只验证菲林斯预设、圣遗物套装与雷霆交响额外伤害。",
    cases: [
        projectTestCase(import.meta.url, {
            id: "flins-thunderous-symphony-additional",
            name: "菲林斯雷霆交响伤害",
            category: "calculation",
            operations: [
                projectSteps.importArtifacts("flins-artifacts.json"),
                projectSteps.importPresets("flins-preset.json"),
                projectSteps.enter("calculator"),
                projectSteps.applyPreset("菲林斯-血染荒城"),
                projectSteps.applyArtifactSet("菲林斯-穹境示现之夜"),
                projectSteps.selectSkill(19),
                projectSteps.setEnemyLevel(85),
                projectSteps.expectDamage("ElevativeDamage", "critical", 132761, 0.05)
            ]
        })
    ]
})
