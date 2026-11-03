<!-- 文档整理：Codex -->
# 开发文档入口

这里是项目文档的统一阅读入口。用户操作、工程维护与游戏计算实现使用不同的说明，按任务阅读即可。

## 先选任务

| 任务 | 文档 |
| --- | --- |
| 使用工具、导入、计算、备份 | [用户说明](../../src/pages/helps/InstructionPage/instruction.md) |
| 安装与维护 YAS 网页连接 | [YAS 接入](../../docs/yas-web.md) |
| 安装开发环境、构建、部署、CI | [开发与部署](../../docs/development.md) |
| 找前端、Store、WASM 等代码入口 | [结构与数据流](../../docs/architecture.md) |
| AI 协作约定 | [AGENTS.md](../../AGENTS.md) |
| 浏览器自动化 | [Mona API](../../mona_api/mona-api.md)、[测试框架](../../tests/README.md) |

全部文档导航见 [docs/README.md](../../docs/README.md)。

## 修改游戏计算实现

先理解与本次改动相关的公共接口：

- [Attribute](attribute.md)：面板属性、属性来源及效果接口。
- [Config](config.md)：配置类型与全局配置约定。
- [DamageBuilder](damage_builder.md)：技能效果、伤害、治疗、护盾与其他输出。

再阅读具体模块：

- [角色](character/index.md)
- [武器](weapon/index.md)
- [圣遗物效果](artifact/index.md)
- [目标函数](target_function/index.md)
- [Buff](buff/index.md)
- [代码审查](review/index.md)

模块索引中的注册、命名和示例约定仍适用。较早实现可能保留旧模型，新增内容优先使用对应文档推荐的实现。

原始游戏资料通常由维护者放在根目录 `prompt/`，该目录不随 Git 分发。资料缺失时列出需要确认的倍率、效果条件等具体内容；不要把另一个角色的实现当作新角色数值的依据。

只修改页面或工程配置时，无需逐个阅读所有角色示例。修改行为后同时更新相应说明，避免继续积累与实际代码不一致的教程。
