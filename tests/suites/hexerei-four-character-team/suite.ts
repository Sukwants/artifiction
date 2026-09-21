import {defineProjectTestSuite, projectSteps, projectTestCase} from "../../framework"

const varkaPreset = "法尔伽-狼的武功歌"
const sucrosePreset = "砂糖-祭礼残章"
const durinPreset = "杜林-黑蚀"
const nicolePreset = "尼可-尘光七谕"

export default defineProjectTestSuite({
    id: "hexerei-four-character-team",
    name: "魔导四角色队伍测试集",
    description: "验证法尔伽、砂糖、杜林、尼可的队伍配置与尼可奥迹造影伤害。",
    cases: [
        projectTestCase(import.meta.url, {
            id: "hexerei-four-character-team-nicole-arcane-projection",
            name: "尼可奥迹造影暴击伤害",
            category: "calculation",
            operations: [
                projectSteps.importArtifacts("artifacts.json"),
                projectSteps.importPresets("presets.json"),
                projectSteps.enter("calculator"),

                projectSteps.addCharacter(varkaPreset, true),
                projectSteps.applyPreset(varkaPreset),
                projectSteps.applyArtifactSet("法尔伽-风起之日", varkaPreset),

                projectSteps.addCharacter(sucrosePreset),
                projectSteps.applyPreset(sucrosePreset),
                projectSteps.applyArtifactSet("砂糖-翠绿之影", sucrosePreset),
                projectSteps.setConfig(
                    sucrosePreset,
                    "artifact_config",
                    "viridescentVenerer*set4",
                    "element",
                    ["Pyro"]
                ),
                projectSteps.setConfig(sucrosePreset, "character_skill", "Sucrose", "talent1_coverage", 1.0),
                projectSteps.setConfig(sucrosePreset, "character_skill", "Sucrose", "talent2_coverage", 1.0),
                projectSteps.setConfig(
                    sucrosePreset,
                    "character_skill",
                    "Sucrose",
                    "small_wind_spirit_coverage",
                    1.0
                ),

                projectSteps.addCharacter(durinPreset),
                projectSteps.applyPreset(durinPreset),
                projectSteps.applyArtifactSet("杜林-风起之日", durinPreset),

                projectSteps.addCharacter(nicolePreset),
                projectSteps.applyPreset(nicolePreset),
                projectSteps.applyArtifactSet("尼可-天之美赐", nicolePreset),

                projectSteps.setOnField(varkaPreset, true),
                projectSteps.selectCharacter(nicolePreset),
                projectSteps.selectSkill(10),
                projectSteps.setEnemyLevel(85),
                projectSteps.expectDamage("Damage.normal", "critical", 74913, 0.02)
            ]
        })
    ]
})
