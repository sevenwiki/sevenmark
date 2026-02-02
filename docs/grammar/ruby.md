# Ruby Text (Japanese Annotations)

<div v-pre>

Ruby text provides pronunciation guides or annotations above base text, commonly used in Japanese (furigana) and East Asian typography.

## Basic Ruby

Use `{{{#ruby}}}` syntax to create ruby annotations:

```sevenmark
{{{#ruby #ruby="かんじ" 漢字}}}
{{{#ruby #ruby="とうきょう" 東京}}}
{{{#ruby #ruby="にほんご" 日本語}}}
```

## Ruby in Context

Ruby text is typically used inline within sentences:

```sevenmark
The capital of Japan is {{{#ruby #ruby="とうきょう" 東京}}} (Tokyo).

Reading {{{#ruby #ruby="かんじ" 漢字}}} can be challenging for learners.

I'm studying {{{#ruby #ruby="にほんご" 日本語}}} at university.
```

## Styled Ruby

Apply styling using parameters:

```sevenmark
{{{#ruby #style="color: red" #ruby="にほん" 日本}}}
{{{#ruby #style="font-size: 1.2em" #ruby="かんじ" 漢字}}}
{{{#ruby #color="blue" #ruby="とうきょう" 東京}}}
```

## Common Japanese Examples

### Kanji with Readings

```sevenmark
Common words:
- {{{#ruby #ruby="がっこう" 学校}}} (gakkou - school)
- {{{#ruby #ruby="せんせい" 先生}}} (sensei - teacher)
- {{{#ruby #ruby="べんきょう" 勉強}}} (benkyou - study)
- {{{#ruby #ruby="だいがく" 大学}}} (daigaku - university)
```

### Place Names

```sevenmark
Cities in Japan:
- {{{#ruby #ruby="とうきょう" 東京}}} (Tōkyō)
- {{{#ruby #ruby="おおさか" 大阪}}} (Ōsaka)
- {{{#ruby #ruby="きょうと" 京都}}} (Kyōto)
- {{{#ruby #ruby="ほっかいどう" 北海道}}} (Hokkaidō)
```

### Common Phrases

```sevenmark
Basic phrases:
- {{{#ruby #ruby="こんにちは" 今日は}}} (Hello)
- {{{#ruby #ruby="ありがとう" 有難う}}} (Thank you)
- {{{#ruby #ruby="さようなら" 左様なら}}} (Goodbye)
```

## Ruby in Complex Structures

### Ruby in Lists

```sevenmark
{{{#list #1
[[Learn {{{#ruby #ruby="ひらがな" 平仮名}}} (Hiragana)]]
[[Learn {{{#ruby #ruby="かたかな" 片仮名}}} (Katakana)]]
[[Learn {{{#ruby #ruby="かんじ" 漢字}}} (Kanji)]]
}}}
```

### Ruby in Tables

```sevenmark
{{{#table
[[[[Kanji]] [[Reading]] [[Meaning]]]]
[[[[{{{#ruby #ruby="みず" 水}}}]] [[mizu]] [[water]]]]
[[[[{{{#ruby #ruby="ひ" 火}}}]] [[hi]] [[fire]]]]
[[[[{{{#ruby #ruby="き" 木}}}]] [[ki]] [[tree]]]]
}}}
```

### Ruby in Headers

```sevenmark
# Introduction to {{{#ruby #ruby="にほんご" 日本語}}}

## Learning {{{#ruby #ruby="かんじ" 漢字}}}

Learn the basics of kanji characters.
```

## Use Cases

### Educational Materials

```sevenmark
Beginner's lesson:

Today we will learn about {{{#ruby #ruby="どうぶつ" 動物}}} (animals).

Common {{{#ruby #ruby="どうぶつ" 動物}}}:
- {{{#ruby #ruby="いぬ" 犬}}} (dog)
- {{{#ruby #ruby="ねこ" 猫}}} (cat)
- {{{#ruby #ruby="とり" 鳥}}} (bird)
```

### Literary Text

```sevenmark
{{{#quote
{{{#ruby #ruby="はる" 春}}}の{{{#ruby #ruby="おがわ" 小川}}}は静かに{{{#ruby #ruby="なが" 流}}}れる

Spring streams flow gently.
}}}
```

### Language Learning

```sevenmark
{{{#fold
[[Practice Vocabulary]]
[[
Read these words:
1. {{{#ruby #ruby="ほん" 本}}} (hon - book)
2. {{{#ruby #ruby="じかん" 時間}}} (jikan - time)
3. {{{#ruby #ruby="ともだち" 友達}}} (tomodachi - friend)
]]
}}}
```

## Styling Options

All standard parameters are supported:

- `#style` - Custom CSS styling
- `#color` - Text color
- `#size` - Font size
- `#opacity` - Opacity level

```sevenmark
{{{#ruby #style="font-weight: bold; color: darkblue" #ruby="うみ" 海}}}
```

## Best Practices

### When to Use Ruby

✅ **Good uses:**
- Educational materials for language learners
- Difficult or uncommon kanji
- Names with non-standard readings
- Literary works targeting beginners

❌ **Avoid:**
- Common words that readers likely know
- Overusing in advanced materials
- When the base text is already clear

### Placement

```sevenmark
<!-- Good: Natural inline usage -->
Learn to read {{{#ruby #ruby="しんぶん" 新聞}}} every day.

<!-- Less ideal: Excessive ruby -->
{{{#ruby #ruby="わたし" 私}}} {{{#ruby #ruby="は" は}}} {{{#ruby #ruby="まなぶ" 学ぶ}}}...
```

## Cultural Context

Ruby annotations are essential in Japanese typography:

- **Furigana**: Reading guides for kanji
- **Educational**: Helps children and learners
- **Disambiguation**: Clarifies uncommon readings
- **Accessibility**: Aids readers with reading difficulties

Example from children's literature:

```sevenmark
{{{#ruby #ruby="むかしむかし" 昔々}}}、{{{#ruby #ruby="ある" 或}}}ところに{{{#ruby #ruby="やさ" 優}}}しい{{{#ruby #ruby="おうさま" 王様}}}が{{{#ruby #ruby="す" 住}}}んでいました。

Once upon a time, in a certain place, there lived a kind king.
```

## Technical Notes

- Ruby text uses the `{{{#ruby #ruby="reading" base}}}` syntax
- The `#ruby` parameter contains the annotation text (furigana)
- The content between tags is the base text (kanji)
- Parameters can be used to style both the base text and annotations
- Ruby elements can be nested within other SevenMark elements
- Empty ruby elements are valid: `{{{#ruby #ruby="" }}}`

</div>
