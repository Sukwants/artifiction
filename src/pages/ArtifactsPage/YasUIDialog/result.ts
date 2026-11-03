import type { IArtifactContentOnly } from '@/types/artifact';

const slots = ['flower', 'feather', 'sand', 'cup', 'head'] as const;
export type MonaResult = Record<typeof slots[number], IArtifactContentOnly[]>;

function object(value: unknown): value is Record<string, unknown> {
    return typeof value === 'object' && value !== null && !Array.isArray(value);
}

// Validate the whole batch before the importer mutates the artifact store.
export function validateMonaResult(value: unknown, setNames: ReadonlySet<string>, statNames: ReadonlySet<string>): asserts value is MonaResult {
    if (!object(value)) throw new Error('YAS 返回的扫描结果格式无效');
    let count = 0;
    const validStat = (stat: unknown) => object(stat) && typeof stat.name === 'string' && statNames.has(stat.name)
        && typeof stat.value === 'number' && Number.isFinite(stat.value) && stat.value >= 0;
    for (const slot of slots) {
        const items = value[slot];
        if (!Array.isArray(items)) throw new Error('YAS 返回的扫描结果缺少圣遗物分组');
        for (const item of items) {
            if (!object(item) || item.position !== slot || typeof item.setName !== 'string' || !setNames.has(item.setName)
                || typeof item.star !== 'number' || !Number.isInteger(item.star) || item.star < 1 || item.star > 5
                || typeof item.level !== 'number' || !Number.isInteger(item.level) || item.level < 0 || item.level > item.star * 4
                || !validStat(item.mainTag) || !Array.isArray(item.normalTags) || item.normalTags.length > 4
                || !item.normalTags.every(validStat)
                || (item.equip !== undefined && typeof item.equip !== 'string')) {
                throw new Error('扫描结果包含无效或网页暂不支持的圣遗物，请更新网页和 YAS 后重试');
            }
        }
        count += items.length;
    }
    if (count === 0) throw new Error('没有扫描到圣遗物，未导入数据');
}
