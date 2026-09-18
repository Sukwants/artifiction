# Mona 页面自动化 API

本文档描述 `window.monaApi` 1.1.0 的实际接口、数据结构、调用顺序和当前限制。

该 API 用于让自动化程序和用户共同操作同一个页面。API 通过现有 Store、Vue 响应式状态和 WASM 计算接口工作；通过 API 进行的修改会反映到页面，页面后续计算也会使用修改后的状态。

## 目录

- [快速开始](#快速开始)
- [通用约定](#通用约定)
- [顶层接口](#顶层接口)
- [页面导航 API](#页面导航-api)
- [账号 API](#账号-api)
- [圣遗物 API](#圣遗物-api)
- [圣遗物套装方案 API](#圣遗物套装方案-api)
- [计算预设 API](#计算预设-api)
- [元数据 API](#元数据-api)
- [计算器生命周期](#计算器生命周期)
- [计算器人物 API](#计算器人物-api)
- [完整计算器状态](#完整计算器状态)
- [动态配置 API](#动态配置-api)
- [伤害与伤害详情](#伤害与伤害详情)
- [面板与圣遗物计算](#面板与圣遗物计算)
- [事件](#事件)
- [错误处理](#错误处理)
- [当前限制](#当前限制)

## 快速开始

API 在应用入口挂载到 `window`：

```js
const api = window.monaApi

await api.ready()

console.log(api.version)
console.log(await api.capabilities())
console.log(await api.accounts.getCurrent())
```

操作计算器前，应确认计算器页面已经挂载：

```js
if (!api.calculator.isActive()) {
  throw new Error("请先打开计算器页面")
}

const characters = await api.calculator.characters.list()
const characterId = characters[0].id

await api.calculator.characters.patch(characterId, {
  skill: {selectedIndex: 0},
  enemy: {level: 100},
  infusion: "Hydro",
})

const damage = await api.calculator.damage.getCurrent({characterId})
console.log(damage)
```

## 通用约定

### Promise

除以下接口外，公开方法都返回 `Promise`：

```ts
monaApi.calculator.isActive(): boolean
monaApi.events.getRevision(): number
monaApi.events.subscribe(listener): () => void
```

即使某项操作当前完全在内存中完成，调用方也应使用 `await`，以允许未来增加持久化等待、页面更新等待或异步校验。

### 可序列化结果

API 返回普通对象或数组，不返回 Vue `ref`、响应式 Proxy、Store、组件实例或 Rust/WASM 对象引用。调用方修改返回对象不会直接修改页面。

### 标识和显示名称

角色、武器、计算函数、Buff、反应和附魔均使用稳定英文键，例如：

```text
Furina
SplendorOfTranquilWaters
overload
Hydro
```

自动化程序不应使用页面翻译后的中文名称作为标识。显示名称可通过元数据中的 `nameLocale` 交给项目现有国际化系统处理。

### 人物 ID 与页面选择

所有人物数据操作显式接受 `characterId`，不会隐式切换页面标签：

```js
await monaApi.calculator.characters.patch(2, {
  enemy: {level: 100},
})
```

只有以下调用会改变用户当前看到的人物标签：

```js
await monaApi.calculator.selection.select(2)
```

人物 ID 在当前计算器页面生命周期内稳定，但不是人物名称，也不保证连续。

### 版本

顶层 API 版本：

```js
monaApi.version // "1.1.0"
```

导出文档和完整计算器状态还具有各自的 `schemaVersion`。调用方导入前应检查 `schema` 和 `schemaVersion`。

## 顶层接口

### `ready()`

```ts
ready(): Promise<void>
```

等待账号数据和持久化后端完成初始化。建议每个自动化会话开始时调用一次。

`ready()` 不会自动打开计算器页面，也不保证计算器控制器已经挂载。

### `capabilities()`

```ts
capabilities(): Promise<{
  accounts: true
  artifacts: true
  artifactLoadouts: true
  presets: true
  calculator: true
  navigation: true
  damageAnalysis: "dynamic-tree"
}>
```

用于调用方在未来版本中进行能力检测，不应仅依据属性是否存在来推断功能。

### 顶层命名空间

```ts
window.monaApi = {
  version,
  ready,
  capabilities,
  accounts,
  artifacts,
  artifactLoadouts,
  presets,
  navigation,
  calculator,
  meta,
  events,
}
```

## 页面导航 API

导航 API 只接受已经具有自动化接口的页面，不暴露首页、设置、多人优化、圣遗物潜力、Playground 等尚无对应自动化接口的页面。

```ts
type MonaApiPage =
  | "accounts"
  | "artifacts"
  | "artifactLoadouts"
  | "presets"
  | "calculator"

navigation.enter(page: MonaApiPage): Promise<{
  page: MonaApiPage
  path: string
  routeName: string | null
}>
```

页面与当前路由的对应关系：

| 页面键 | 页面 | 路由 |
| --- | --- | --- |
| `accounts` | 账号 | `/account` |
| `artifacts` | 圣遗物 | `/artifacts` |
| `artifactLoadouts` | 圣遗物套装方案 | `/artifacts-kumi` |
| `presets` | 计算预设 | `/presets` |
| `calculator` | 计算器 | `/calculate` |

典型调用：

```js
await monaApi.ready()

await monaApi.navigation.enter("calculator")

const characters = await monaApi.calculator.characters.list()
```

该接口使用 Vue Router，不要求调用方判断项目当前使用 hash 路由还是 history 路由。Promise 在路由跳转、目标页面挂载和进入动画结束后返回；进入 `calculator` 时还会等待计算器页面控制器注册完成。因此自动化程序等待该 Promise 后即可调用对应页面 API 或截图，不需要额外使用固定时长的 `sleep`。

传入未支持的页面键会抛出 `UNSUPPORTED_PAGE`。导航到当前页面是允许的，并仍会返回当前路由信息和产生导航事件。

## 账号 API

账号结构：

```ts
interface Account {
  id: number
  name: string
}
```

### 列出账号

```ts
accounts.list(): Promise<Account[]>
```

### 获取当前账号

```ts
accounts.getCurrent(): Promise<Account>
```

### 创建账号

```ts
accounts.create(input: {name: string}): Promise<Account>
```

名称会去除首尾空白，空名称会抛出 `INVALID_ARGUMENT`。

```js
const account = await monaApi.accounts.create({name: "测试账号"})
```

### 重命名账号

```ts
accounts.rename(id: number, name: string): Promise<Account>
```

### 删除账号

```ts
accounts.remove(id: number): Promise<void>
```

不能删除当前账号，否则抛出 `CURRENT_ACCOUNT_CANNOT_BE_REMOVED`。应先切换到另一个账号。

### 切换账号

```ts
accounts.switch(id: number): Promise<Account>
```

该 Promise 会在目标账号的圣遗物、套装方案和预设载入后完成。

切换账号会替换这些 Store 的当前内容，因此调用方不要继续使用切换前取得的圣遗物或预设快照。

## 圣遗物 API

### 数据结构

圣遗物部位使用项目现有键：

```ts
type ArtifactPosition = "flower" | "feather" | "sand" | "cup" | "head"
```

注意空之杯是 `cup`，不是 `goblet`。

```ts
interface ArtifactStat {
  name: string
  value: number
}

interface NewArtifact {
  setName: string
  position: ArtifactPosition
  star: number
  level: number
  mainTag: ArtifactStat
  normalTags: ArtifactStat[]
  locked?: boolean
}

interface Artifact extends Omit<NewArtifact, "locked"> {
  id: number
  omit: boolean
  contentHash: string
}
```

当前项目内部的 `omit` 表示“不参与计算”。API 为了贴合页面术语，将其暴露为 `locked` 查询和 `setLocked()` 操作：

```text
locked = true  <=>  omit = true  <=>  不参与圣遗物计算
```

### 列出圣遗物

```ts
artifacts.list(query?: {
  ids?: number[]
  position?: ArtifactPosition
  setName?: string
  star?: number
  level?: number
  locked?: boolean
}): Promise<Artifact[]>
```

所有条件同时生效：

```js
const flowers = await monaApi.artifacts.list({
  position: "flower",
  star: 5,
  level: 20,
  locked: false,
})
```

### 获取单个圣遗物

```ts
artifacts.get(id: number): Promise<Artifact>
```

不存在时抛出 `NOT_FOUND`。

### 添加圣遗物

```ts
artifacts.create(input: NewArtifact): Promise<Artifact>
```

ID 和内容哈希由 Store 生成：

```js
const artifact = await monaApi.artifacts.create({
  setName: "EmblemOfSeveredFate",
  position: "flower",
  star: 5,
  level: 20,
  mainTag: {name: "lifeStatic", value: 4780},
  normalTags: [
    {name: "critical", value: 0.066},
    {name: "criticalDamage", value: 0.132},
  ],
  locked: false,
})
```

### 更新圣遗物

```ts
artifacts.update(
  id: number,
  patch: Partial<Omit<NewArtifact, "locked">>,
): Promise<Artifact>
```

这是浅层 patch。修改 `mainTag` 或 `normalTags` 时应提供完整字段值。锁定状态不属于圣遗物内容 patch，应使用 `artifacts.setLocked()`。

### 锁定或解锁

```ts
artifacts.setLocked(id: number, locked: boolean): Promise<Artifact>
```

### 删除圣遗物

```ts
artifacts.remove(id: number): Promise<void>
```

删除圣遗物不会自动修复计算器或套装方案中保存的对应 ID。读取计算器装备时，不存在的 ID 会被视为无对应圣遗物；再次通过 API 设置装备时会进行存在性校验。

### 导出圣遗物

```ts
artifacts.export(): Promise<ArtifactExportDocument>
```

```ts
interface ArtifactExportDocument {
  schema: "mona.artifacts"
  schemaVersion: 1
  data: {
    flower: Artifact[]
    feather: Artifact[]
    sand: Artifact[]
    cup: Artifact[]
    head: Artifact[]
  }
}
```

该接口返回对象，不会触发浏览器下载。

### 导入圣遗物

```ts
artifacts.import(
  document: ArtifactExportDocument | ArtifactExportDocument["data"],
  options?: {
    mode?: "append" | "replace"
    duplicate?: "skip" | "allow" | "overwrite" | "error"
    dryRun?: boolean
    confirm?: boolean
  },
): Promise<ImportReport>
```

```ts
interface ImportReport {
  imported: number
  skipped: number
  total: number
  dryRun: boolean
}
```

选项含义：

| 选项 | 默认值 | 含义 |
| --- | --- | --- |
| `mode` | `append` | 追加到当前仓库，或清空后替换 |
| `duplicate` | `skip` | 如何处理内容哈希相同的圣遗物 |
| `dryRun` | `false` | 只校验和统计，不修改 Store |
| `confirm` | `false` | `replace` 模式必须显式传入 `true` |

重复策略：

| 策略 | 行为 |
| --- | --- |
| `skip` | 跳过仓库中已有的同内容圣遗物 |
| `allow` | 允许保存内容相同的多件圣遗物 |
| `overwrite` | 删除已有同内容圣遗物，再添加导入项 |
| `error` | 遇到第一件重复圣遗物即抛出错误 |

重复检测同时覆盖当前仓库和导入文档内部。文档内部重复时，`skip` 保留第一件，`overwrite` 保留最后一件，`error` 抛错，`allow` 保留全部；预检统计与实际写入使用同一执行计划。

推荐先预检：

```js
const report = await monaApi.artifacts.import(document, {
  mode: "replace",
  duplicate: "skip",
  dryRun: true,
  confirm: true,
})

if (report.total > 0) {
  await monaApi.artifacts.import(document, {
    mode: "replace",
    duplicate: "skip",
    confirm: true,
  })
}
```

## 圣遗物套装方案 API

该命名空间对应用户保存的五件圣遗物组合，即项目中的 `kumi`，不是游戏静态套装定义。

五个部位在此处使用固定数组顺序：

```text
[flower, feather, sand, cup, head]
```

### 数据结构

```ts
interface LoadoutFolder {
  id: number
  title: string
  dir: true
  children: number[]
}

interface ArtifactLoadout {
  id: number
  title: string
  dir: false
  artifactIds: [
    number | null,
    number | null,
    number | null,
    number | null,
    number | null,
  ]
}
```

### 收藏夹

```ts
artifactLoadouts.folders.list(): Promise<LoadoutFolder[]>
artifactLoadouts.folders.create(input: {name: string}): Promise<LoadoutFolder>
artifactLoadouts.folders.rename(id: number, name: string): Promise<LoadoutFolder>
artifactLoadouts.folders.remove(id: number): Promise<void>
```

删除收藏夹会沿用现有 Store 行为，同时删除它包含的套装方案。

### 套装方案

```ts
artifactLoadouts.list(query?: {folderId?: number}): Promise<ArtifactLoadout[]>
artifactLoadouts.get(id: number): Promise<ArtifactLoadout>

artifactLoadouts.create(input: {
  folderId: number
  name: string
  artifactIds?: Array<number | null>
}): Promise<ArtifactLoadout>

artifactLoadouts.rename(id: number, name: string): Promise<ArtifactLoadout>
artifactLoadouts.updateArtifacts(
  id: number,
  artifactIds: Array<number | null>,
): Promise<ArtifactLoadout>

artifactLoadouts.remove(id: number): Promise<void>
```

`artifactIds` 必须恰好包含五项。当前这一层沿用原 Store，不校验每个 ID 的存在性和部位；计算器装备 API 会进行更严格的校验。

## 计算预设 API

预设条目沿用项目现有 version 4 格式：

```ts
interface PresetEntry {
  name: string
  version: number
  item: Preset
}
```

`Preset` 主要包含：

```ts
interface Preset {
  name: string
  character: unknown
  weapon: unknown
  targetFunction: unknown
  buffs?: Record<number, unknown>
  artifactConfig?: unknown
  artifactEffectMode?: "custom" | "auto"
  constraint?: unknown
  filter?: unknown
  algorithm?: "AStar" | "Heuristic" | "Naive"
  useDSL?: boolean
  dslSource?: string
  globalConfigUnlinked?: Record<string, boolean>
}
```

### 查询和保存

```ts
presets.list(): Promise<PresetEntry[]>
presets.get(name: string): Promise<PresetEntry>

presets.save(
  name: string,
  item: Preset,
  options?: {overwrite?: boolean},
): Promise<PresetEntry>
```

目标名称已存在且未传 `overwrite: true` 时抛出 `ALREADY_EXISTS`。

### 重命名和删除

```ts
presets.rename(name: string, newName: string): Promise<PresetEntry>
presets.remove(name: string): Promise<void>
```

### 导出和导入

```ts
presets.export(names?: string[]): Promise<{
  schema: "mona.presets"
  schemaVersion: 1
  data: PresetEntry[]
}>

presets.import(
  document: PresetEntry[] | PresetExportDocument,
  options?: {
    overwrite?: boolean
    dryRun?: boolean
  },
): Promise<ImportReport>
```

不传 `names` 时导出全部预设。预设导入会走现有升级逻辑，并保留 `globalConfigUnlinked`。

## 元数据 API

元数据 API 只读，用于发现当前版本中可选择的角色、武器、计算函数、Buff、技能和动态配置结构。

### 角色

```ts
meta.characters.list(): Promise<Array<{
  key: string
  nameLocale: unknown
  element: string
  weaponType: string
  star: number
  configSchema: ConfigMeta[]
}>>

meta.characters.get(name: string): Promise<unknown>
```

### 武器

```ts
meta.weapons.list(query?: {
  characterName?: string
  weaponType?: string
}): Promise<WeaponDescriptor[]>

meta.weapons.get(name: string): Promise<unknown>
```

传入 `characterName` 时会根据角色武器类型筛选。显式的 `weaponType` 优先于 `characterName`。

### 计算函数

```ts
meta.targetFunctions.list(query?: {
  characterName?: string
}): Promise<TargetFunctionDescriptor[]>

meta.targetFunctions.get(name: string): Promise<unknown>
```

传入角色名时，返回角色专属计算函数和通用计算函数。

### Buff

```ts
meta.buffs.list(): Promise<BuffDescriptor[]>
meta.buffs.get(name: string): Promise<unknown>
```

### 游戏静态圣遗物套装

```ts
meta.artifactSets.list(): Promise<ArtifactSetDescriptor[]>
meta.artifactSets.get(name: string): Promise<unknown>
```

这里是游戏静态套装数据，只读。返回值包含不同件数对应的动态配置 schema。

### 技能选项

```ts
meta.skills.list(input: {
  characterName: string
}): Promise<Array<{
  index: number
  nameLocale: unknown
}>>
```

`index` 与 WASM 和 `skill.selectedIndex` 一致。

### 反应和附魔

```ts
meta.reactions.list(): Promise<{
  transformative: string[]
  elevative: string[]
}>

meta.infusions.list(): Promise<string[]>
```

当前独立聚变反应键：

```text
swirl_cryo
swirl_pyro
swirl_hydro
swirl_electro
overload
electro_charged
shatter
superconduct
bloom
hyperbloom
burgeon
burning
crystallize
```

当前独立擢升反应键：

```text
lunar_charged_reaction
lunar_crystallize_reaction
stellar_swirl_reaction_anemo
stellar_swirl_reaction_cryo
```

其他只能由具体角色技能产生的擢升伤害，应通过 `damage.getCurrent()` 获取。

### 动态配置元数据

各描述对象的 `configSchema` 来自生成数据，典型结构如下：

```ts
interface ConfigMeta {
  name: string
  type: string
  title: unknown
  default: unknown
  min?: number
  max?: number
  global_link?: {
    key: string
    priority: number
    unlinked: boolean
    team_shared?: boolean
  }
  [key: string]: unknown
}
```

调用方必须允许未来增加新的 `type` 和额外元数据字段。

## 计算器生命周期

计算器状态保存在计算器页面组件中，而不是全局 Store。因此：

```ts
calculator.isActive(): boolean
```

只有计算器页面处于挂载或 KeepAlive 激活状态时，计算器人物、配置、伤害、面板和优化接口才可用。

页面未激活时调用会抛出：

```text
PAGE_NOT_ACTIVE
```

推荐流程：

```js
await monaApi.ready()

if (!monaApi.calculator.isActive()) {
  // 由宿主自动化打开 /calculate 页面
}

const characters = await monaApi.calculator.characters.list()
```

切换账号不等于销毁计算器页面。现有页面会清空优化结果和已装备圣遗物，但其他人物页面状态是否重置仍由原页面逻辑决定。

## 计算器人物 API

### 人物摘要

```ts
interface CalculatorCharacterSummary {
  id: number
  name?: string
  teamId: number
  onField: boolean
  selected: boolean
}
```

### 列出人物

```ts
calculator.characters.list(): Promise<CalculatorCharacterSummary[]>
```

### 获取人物完整状态

```ts
calculator.characters.get(id: number): Promise<CharacterState>
```

### 新增人物

```ts
calculator.characters.add(input?: Partial<CharacterState>): Promise<CharacterState>
```

先创建默认人物，再应用 `input`。返回新人物的稳定 ID。

```js
const character = await monaApi.calculator.characters.add({
  teamId: 1,
  onField: false,
  character: {
    name: "Furina",
    level: 90,
    ascend: true,
    constellation: 2,
  },
})
```

新增人物目前会选中新标签，这是现有页面新增标签的行为。

### 修改人物

```ts
calculator.characters.patch(
  id: number,
  patch: CharacterPatch,
): Promise<CharacterState>
```

修改顺序由控制器处理：角色、武器和计算函数切换后，会等待配置注册，再恢复动态配置；圣遗物 ID 更新后，也会等待套装效果配置注册。

支持的主要字段：

```ts
interface CharacterPatch {
  teamId?: number
  onField?: boolean
  character?: Partial<CharacterBaseState>
  weapon?: Partial<WeaponState>
  targetFunction?: Partial<TargetFunctionState>
  buffs?: BuffState[]
  artifacts?: EquippedArtifacts | Array<number | null>
  skill?: Partial<SkillState>
  enemy?: Partial<EnemyState>
  infusion?: string | null
  artifactEffect?: Partial<ArtifactEffectState>
  optimization?: Partial<OptimizationState>
  configUnlinked?: Record<string, boolean>
  currentPresetName?: string | null
}
```

### 删除人物

```ts
calculator.characters.remove(id: number): Promise<void>
```

删除时会同时从 `ConfigManager` 注销该人物的配置。

### 队伍和在场状态

```ts
calculator.characters.setTeam(id: number, teamId: number): Promise<CharacterState>
calculator.characters.setOnField(id: number, onField: boolean): Promise<CharacterState>
```

这些是 `characters.patch()` 的便捷接口。

### 装备圣遗物

```ts
interface EquippedArtifacts {
  flower: number | null
  feather: number | null
  sand: number | null
  cup: number | null
  head: number | null
}

calculator.characters.setArtifacts(
  id: number,
  artifacts: EquippedArtifacts | Array<number | null>,
): Promise<CharacterState>
```

对象形式也兼容输入 `goblet`，但输出始终使用 `cup`。

API 会校验：

- 数组必须恰好五项；
- 非空 ID 必须存在；
- 圣遗物部位必须与目标槽位相符。

### 应用预设

```ts
calculator.characters.applyPreset(
  id: number,
  name: string,
): Promise<CharacterState>
```

沿用页面现有预设应用逻辑，包括角色、武器、计算函数、Buff、优化约束、圣遗物效果模式和解除联动状态。

### 页面选择

```ts
calculator.selection.get(): Promise<number | null>
calculator.selection.select(id: number): Promise<void>
```

选择只改变当前标签，不修改人物数据。

## 完整计算器状态

### `CharacterState`

```ts
interface CharacterState {
  id: number
  teamId: number
  onField: boolean

  character: {
    name: string
    level: number
    ascend: boolean
    constellation: number
    skill1: number
    skill2: number
    skill3: number
    tags: string[]
    params: Record<string, unknown>
  }

  weapon: {
    name: string
    level: number
    ascend: boolean
    refine: number
    params: Record<string, unknown>
  }

  targetFunction: {
    name: string
    params: Record<string, unknown>
    use_dsl: boolean
    dsl_source: string
    type: "builtin" | "dsl"
    dslSource: string
  }

  buffs: Array<{
    id: number
    name: string
    config: Record<string, Record<string, unknown>>
    lock: boolean
  }>

  artifacts: EquippedArtifacts

  skill: {
    selectedIndex: number
    params: Record<string, unknown>
  }

  enemy: EnemyState
  infusion: string

  artifactEffect: {
    mode: "auto" | "custom"
    params: Record<string, unknown>
    equippedParams: Record<string, unknown>
  }

  optimization: {
    algorithm: string
    constraint: unknown
    filter: unknown
  }

  configUnlinked: Record<string, boolean>
  currentPresetName: string | null
}
```

Buff 状态输出使用模块包裹结构，例如 `config: {ATKPercentage: {p: 0.2}}`。调用 `characters.patch()` 时也可以传入简化结构 `config: {p: 0.2}`，API 会根据 Buff 名称自动补上外层模块键。

`targetFunction.type` 和 `targetFunction.dslSource` 是页面 API 提供的友好字段；`use_dsl` 和 `dsl_source` 是传给 WASM 的兼容字段。读取完整状态时两组字段会同时存在。修改时推荐使用 `type` 和 `dslSource`，但 `characters.patch()` 也接受底层字段名。

`character.skill1/2/3` 使用 WASM 的零基技能等级格式。调用 patch 时也可以使用更符合页面习惯的一基形式：

```js
await monaApi.calculator.characters.patch(id, {
  character: {
    talents: {
      normal: 9,
      skill: 10,
      burst: 10,
    },
  },
})
```

不要在同一次 patch 中同时提供 `skill1/2/3` 和 `talents`；后者会覆盖前者。

敌人结构：

```ts
interface EnemyState {
  level: number
  electro_res: number
  pyro_res: number
  hydro_res: number
  cryo_res: number
  geo_res: number
  anemo_res: number
  dendro_res: number
  physical_res: number
}
```

### 页面状态

```ts
interface CalculatorState {
  schema: "mona.calculator-state"
  schemaVersion: 1
  selectedCharacterId: number | null
  characters: CharacterState[]
}
```

### 获取状态

```ts
calculator.state.get(): Promise<CalculatorState>
```

### 恢复状态

```ts
calculator.state.set(
  state: CalculatorState,
  options?: {mode?: "replace" | "merge"},
): Promise<CalculatorState>
```

`replace` 是默认模式。它会在输入人物较少时删除页面末尾多余人物，再依次应用输入人物状态。

`merge` 不删除现有人物，但仍会逐项应用 `state.characters` 中的人物。

状态中的人物 ID 会尽量复用；无法复用时，控制器会建立导入 ID 到实际页面 ID 的映射，并据此恢复当前选中人物。

### 页面级 patch

```ts
calculator.state.patch(patch: Partial<CalculatorState>): Promise<CalculatorState>
```

当前实现是页面状态的浅层合并，适合修改 `selectedCharacterId` 或一次提供完整 `characters`。修改单个人物时应优先使用 `calculator.characters.patch()`。

## 动态配置 API

动态配置由页面级 `ConfigManager` 管理。包括：

- 角色额外配置；
- 武器额外配置；
- 计算函数参数；
- Buff 参数；
- 技能参数；
- 自定义圣遗物效果；
- 当前装备触发的圣遗物套装参数。

### 配置描述对象

```ts
interface ConfigDescriptor {
  id: string
  address: {
    character_id: number
    module_name: string
    object_name: string
    config_name: string
  }
  meta: ConfigMeta
  localValue: unknown
  effectiveValue: unknown
  unlinked: boolean
}
```

当前 `id` 格式类似：

```text
1#character:Furina:some_config
1#weapon:SplendorOfTranquilWaters:some_config
1#buff:SomeBuff-123456:some_config
```

调用方应把它当作不透明句柄，不要自行拼接。Buff 地址包含实例 ID。

### 枚举配置

```ts
calculator.config.list(query?: {
  characterId?: number
  module?: string
}): Promise<ConfigDescriptor[]>
```

不传 `characterId` 时返回页面所有人物的配置。

目前可能出现的内部模块名包括：

```text
character
weapon
target_function
buff
character_skill
artifact_config
artifact_single_config
```

调用方必须允许未来增加模块名。

### 读取配置

```ts
calculator.config.get(
  id: string,
  options?: {mode?: "local" | "effective"},
): Promise<unknown>
```

默认 `mode` 为 `effective`：

| 模式 | 含义 |
| --- | --- |
| `effective` | 返回考虑全局链接、优先级和跨角色共享后的实际生效值 |
| `local` | 返回该地址自身保存的原始值，不解析全局链接 |

### 修改配置

```ts
calculator.config.set(
  id: string,
  value: unknown,
  options?: {mode?: "local" | "effective"},
): Promise<unknown>
```

默认 `effective` 与用户在页面上修改控件的语义一致：如果配置受更高优先级全局项控制，更新会落到当前生效的最高优先级地址。

`local` 只更新目标地址自身，主要用于精确恢复快照：

```js
const configs = await monaApi.calculator.config.list({characterId: 1})
const target = configs.find(item => item.meta.name === "some_config")

await monaApi.calculator.config.set(target.id, 0.8, {
  mode: "local",
})
```

API 当前依赖生成的配置和页面控件进行范围约束；`config.set()` 本身不会通用校验所有配置类型、枚举和最大最小值。自动化程序应根据 `descriptor.meta` 校验输入。

### 解除联动

```ts
calculator.config.setUnlinked(
  id: string,
  unlinked: boolean,
): Promise<void>
```

设置为 `true` 后，该地址读取和修改时不再使用全局链接值。

## 伤害与伤害详情

三类入口共用同一个动态分析结果协议：

```ts
calculator.damage.getCurrent(input): Promise<DamageAnalysisResult>
calculator.damage.getTransformative(input): Promise<DamageAnalysisResult>
calculator.damage.getAllTransformative(input): Promise<Record<string, DamageAnalysisResult>>
calculator.damage.getElevative(input): Promise<DamageAnalysisResult>
calculator.damage.getAllElevative(input): Promise<Record<string, DamageAnalysisResult>>
```

这些接口直接使用当前页面响应式状态构造的 WASM 输入，不读取表格文字，也不返回页面中 `Math.round()` 后的数字。

### 当前技能

```ts
calculator.damage.getCurrent(input: {
  characterId: number
  includeRaw?: boolean
}): Promise<DamageAnalysisResult>
```

使用该人物当前的：

- `skill.selectedIndex`；
- 技能动态配置；
- 附魔；
- 敌人；
- 角色、武器、Buff 和圣遗物；
- 同页面中的其他队伍人物。

返回事件可能是普通伤害、聚变伤害、擢升伤害、治疗、护盾、一般数值或无结果。

### 聚变反应

```ts
calculator.damage.getTransformative(input: {
  characterId: number
  reaction: string
  includeRaw?: boolean
}): Promise<DamageAnalysisResult>

calculator.damage.getAllTransformative(input: {
  characterId: number
  includeRaw?: boolean
}): Promise<Record<string, DamageAnalysisResult>>
```

```js
const hyperbloom = await monaApi.calculator.damage.getTransformative({
  characterId: 1,
  reaction: "hyperbloom",
})
```

### 擢升反应

```ts
calculator.damage.getElevative(input: {
  characterId: number
  reaction: string
  includeRaw?: boolean
}): Promise<DamageAnalysisResult>

calculator.damage.getAllElevative(input: {
  characterId: number
  includeRaw?: boolean
}): Promise<Record<string, DamageAnalysisResult>>
```

反应键应通过 `meta.reactions.list()` 获取，不要硬编码未被当前后端独立暴露的枚举值。

### 规范化结果

```ts
interface DamageAnalysisResult {
  kind: string
  variants: DamageVariant[]
  raw?: unknown
}

interface DamageVariant {
  key: string
  result: {
    expectation: number
    critical: number
    nonCritical: number
  }
  details: AnalysisNode[]
}
```

普通技能可能同时返回多个 variant，例如：

```text
Damage.normal
Damage.melt
Damage.vaporize
Damage.spread
Damage.aggravate
```

治疗、护盾、聚变和擢升事件通常只有一个 variant。

不要假定 `variants[0]` 永远代表特定反应；应按 `key` 查找：

```js
const analysis = await monaApi.calculator.damage.getCurrent({
  characterId: 1,
})

const normal = analysis.variants.find(item => item.key === "Damage.normal")
const vaporize = analysis.variants.find(item => item.key === "Damage.vaporize")
```

### 动态详情节点

```ts
interface AnalysisNode {
  key: string
  kind: "group" | "number" | "string" | "boolean" | "null" | "value"
  value?: unknown
  children?: AnalysisNode[]
}
```

详情字段不写死。当前后端的 `atk`、`bonus`、`reaction_enhance`、`elevative_base` 等字段都会被递归转换成节点；以后后端新增字段时，会自动出现新的 `key`。

通用遍历示例：

```js
function walk(nodes, path = []) {
  for (const node of nodes) {
    const currentPath = [...path, node.key]

    if (node.kind === "group") {
      walk(node.children ?? [], currentPath)
    } else {
      console.log(currentPath.join("."), node.value)
    }
  }
}

const overload = await monaApi.calculator.damage.getTransformative({
  characterId: 1,
  reaction: "overload",
})

walk(overload.variants[0].details)
```

调用方必须：

- 允许未知 `key`；
- 允许详情节点顺序变化；
- 根据 `kind` 解释值；
- 不根据数组下标定位某个乘区；
- 不假定所有事件拥有相同详情字段。

### 原始 WASM 结果

传入 `includeRaw: true` 时，结果增加 `raw`：

```js
const result = await monaApi.calculator.damage.getCurrent({
  characterId: 1,
  includeRaw: true,
})

console.log(result.raw)
```

`raw` 用于调试和研究公式，不属于稳定协议。后端升级可以修改其字段、枚举表示和嵌套结构。

`kind` 和动态节点也会反映后端新增事件，但 `DamageAnalysisResult`、`DamageVariant` 和 `AnalysisNode` 的外壳是 API 稳定边界。

## 面板与圣遗物计算

### 右侧人物面板

```ts
calculator.panel.get(input: {
  characterId: number
}): Promise<unknown>
```

返回 `CommonInterface.get_attribute()` 的普通对象副本。当前未对面板字段进行规范化，调用方应把其字段视为后端相关数据。

### 单人圣遗物优化

```ts
calculator.optimize.run(input: {
  characterId: number
  applyResult?: number | false
}): Promise<{
  results: OptimizationResult[]
  appliedResult: number | false
}>
```

`applyResult` 使用一基序号：

- `false`：只计算，不改变人物装备；
- `1`：应用第一名结果；
- `2`：应用第二名结果；
- 以此类推。

省略时默认为 `false`，与页面按钮自动应用第一项的行为不同，避免自动化查询产生隐式修改。

```js
const result = await monaApi.calculator.optimize.run({
  characterId: 1,
  applyResult: false,
})

console.log(result.results[0])
```

计算使用当前页面的算法、约束、主词条筛选、收藏夹筛选和至少 16 级的未锁定圣遗物。没有可计算圣遗物时抛出 `NO_ARTIFACTS`。

### 最佳静态套装计算

```ts
calculator.bestArtifactSet.run(input: {
  presetName?: string
  config?: unknown
  timeout?: number
}): Promise<Array<{
  stats: string[]
  value: number
  setType: unknown
  ratio: number
}>>
```

必须提供 `presetName` 或底层 `config`。同时提供时优先使用 `config`。

```js
const result = await monaApi.calculator.bestArtifactSet.run({
  presetName: "芙宁娜预设",
  timeout: 120000,
})
```

结果按 `value` 从高到低排序，`ratio` 是相对于最高值的比例。

该计算会启动独立 Worker，通常显著慢于伤害查询和单次圣遗物优化。`timeout` 单位为毫秒，默认 `120000`；超时会终止本次 Worker 并抛出 `COMPUTATION_TIMEOUT`，不会修改计算器页面的配置。WASM 或 Worker 内部计算失败时抛出 `COMPUTATION_FAILED`。

`config` 是底层 WASM 输入，属于高级用法：

```ts
interface BestArtifactSetConfig {
  character: unknown
  weapon: unknown
  target_function: unknown
  buffs: unknown
  enemy: unknown | null
  artifact_config: unknown | null
}
```

## 事件

### 订阅

```ts
events.subscribe(
  listener: (event: MonaApiEvent) => void,
): () => void
```

```ts
interface MonaApiEvent {
  type: string
  revision: number
  source: "api" | "page"
  data?: unknown
}
```

```js
const unsubscribe = monaApi.events.subscribe(event => {
  console.log(event.revision, event.type, event.data)
})

// 不再需要监听时
unsubscribe()
```

### Revision

```ts
events.getRevision(): number
```

Revision 从页面加载后的 0 开始，每发出一个 API 事件递增一次。它目前用于事件排序，不是持久化版本，也没有实现 `expectedRevision` 冲突检查。

### 当前事件类型

```text
account.created
account.changed
account.removed
account.switched

artifact.created
artifact.changed
artifact.removed
artifacts.imported

artifactLoadout.folder.created
artifactLoadout.folder.changed
artifactLoadout.folder.removed
artifactLoadout.created
artifactLoadout.changed
artifactLoadout.removed

preset.changed
preset.renamed
preset.removed
presets.imported

calculator.character.created
calculator.character.changed
calculator.character.removed
calculator.selection.changed
calculator.state.changed
calculator.config.changed
calculator.optimization.completed

navigation.entered
```

当前实现只保证 API 发起的操作产生事件，`source` 因此通常是 `api`。用户直接操作页面控件尚未统一产生 `source: "page"` 事件。

## 错误处理

API 业务错误使用：

```ts
class MonaApiError extends Error {
  code: string
  details?: unknown
}
```

调用示例：

```js
try {
  await monaApi.calculator.damage.getTransformative({
    characterId: 1,
    reaction: "unknown_reaction",
  })
} catch (error) {
  console.error(error.code, error.message, error.details)
}
```

当前可能出现的错误码：

| 错误码 | 含义 |
| --- | --- |
| `PAGE_NOT_ACTIVE` | 计算器页面未挂载或未激活 |
| `PAGE_NOT_READY` | 新增人物后子组件未能及时完成挂载 |
| `NOT_FOUND` | 账号、圣遗物、人物、配置、预设等不存在 |
| `INVALID_ARGUMENT` | 参数结构、名称或数组长度无效 |
| `INVALID_ARTIFACT` | 圣遗物缺少必要字段 |
| `INVALID_ARTIFACT_SLOT` | 圣遗物部位与装备槽不匹配 |
| `INCOMPATIBLE_WEAPON` | 武器类型与当前角色不兼容 |
| `CURRENT_ACCOUNT_CANNOT_BE_REMOVED` | 尝试删除当前账号 |
| `CONFIRMATION_REQUIRED` | 危险导入操作缺少显式确认 |
| `DUPLICATE_ARTIFACT` | 重复策略为 `error` 且发现重复圣遗物 |
| `INVALID_IMPORT` | 导入文档结构无效 |
| `ALREADY_EXISTS` | 预设等目标名称已经存在 |
| `UNSUPPORTED_REACTION` | 当前独立反应计算接口不支持该反应键 |
| `UNSUPPORTED_PAGE` | 页面没有对应自动化接口，或页面键无效 |
| `NO_ARTIFACTS` | 没有符合条件的圣遗物可参与优化 |
| `COMPUTATION_TIMEOUT` | Worker 计算超过指定超时时间 |
| `COMPUTATION_FAILED` | Worker 或 WASM 内部计算失败 |

通过 `calculator.optimize.run` 和 `calculator.bestArtifactSet.run` 发起的 Worker/WASM 错误会被规范化为上述错误码。其他页面内部直接触发的旧计算流程仍可能产生普通 `Error` 或字符串错误。

## 完整使用示例

下面示例在已打开计算器页面的前提下，创建人物、设置基本状态、枚举动态配置并读取伤害：

```js
const api = window.monaApi
await api.ready()

await api.navigation.enter("calculator")

const character = await api.calculator.characters.add({
  teamId: 1,
  onField: true,
  character: {
    name: "Furina",
    level: 90,
    ascend: true,
    constellation: 2,
    talents: {
      normal: 6,
      skill: 10,
      burst: 10,
    },
  },
  weapon: {
    name: "SplendorOfTranquilWaters",
    level: 90,
    ascend: true,
    refine: 1,
  },
  skill: {
    selectedIndex: 0,
  },
  enemy: {
    level: 100,
    hydro_res: 0.1,
  },
  infusion: "None",
})

const options = await api.meta.skills.list({
  characterName: character.character.name,
})

console.log("可选技能", options)

const configs = await api.calculator.config.list({
  characterId: character.id,
})

for (const config of configs) {
  console.log(
    config.id,
    config.meta.type,
    config.localValue,
    config.effectiveValue,
    config.unlinked,
  )
}

const damage = await api.calculator.damage.getCurrent({
  characterId: character.id,
})

for (const variant of damage.variants) {
  console.log(variant.key, variant.result)
}
```

完整状态备份和恢复：

```js
const snapshot = await monaApi.calculator.state.get()

// 进行若干页面或 API 操作

await monaApi.calculator.state.set(snapshot, {
  mode: "replace",
})
```

## 当前限制

1. 计算器 API 依赖计算器页面组件，不能在任意路由后台创建一个无页面计算器。
2. 事件目前主要覆盖 API 写操作，尚未完整监听用户直接操作页面产生的变化。
3. 尚未实现事务、批量事件合并和 `expectedRevision` 并发冲突检测。
4. `calculator.state.patch()` 是浅层合并；单个人物修改应使用 `characters.patch()`。
5. `config.set()` 尚未统一校验所有动态配置类型和范围，应参考 `ConfigDescriptor.meta`。
6. `calculator.panel.get()` 和伤害结果的 `raw` 仍跟随后端结构，不保证跨版本稳定。
7. 优化计算目前直接返回 Promise，没有独立任务 ID、进度查询和取消接口。
8. 圣遗物套装方案 API 尚未提供整体导入、导出和跨收藏夹移动接口。
9. 删除仓库圣遗物不会自动清理所有引用它的套装方案。
10. 静态元数据的 `get()` 在名称不存在时当前返回 `undefined`，没有统一抛出 `NOT_FOUND`。

这些限制是 1.1.0 的明确边界。后续增加能力时，应优先保持现有方法语义和外层数据协议兼容。
