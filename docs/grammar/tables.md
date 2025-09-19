# 테이블

<div v-pre>

SevenMark에서 테이블은 `{{{#table}}}` 구문을 사용하며, 행과 셀 구조가 중첩된 `[[]]`로 구성됩니다.

## 기본 테이블

```sevenmark
{{{#table
[[[[Cell 1,1]] [[Cell 1,2]]]]
[[[[Cell 2,1]] [[Cell 2,2]]]]
}}}
```

실제 구조는 다음과 같습니다:
- `{{{#table}}}`: 테이블 컨테이너
- `[[[[셀]] [[셀]]]]`: 테이블 행 (TableInnerElement1)
- 행 내부의 `[[셀]]`: 개별 셀 (TableInnerElement2)

## 스타일이 적용된 테이블

```sevenmark
{{{#table #style="border-collapse:collapse"
[[[[**Header 1**]] [[*Header 2*]] [[~~Header 3~~]]]]
[[[[Simple cell]] [[Styled cell]] [[Another cell]]]]
}}}
```

## 셀 병합

### 가로 병합 (colspan)

`#x` 매개변수를 사용합니다:

```sevenmark
{{{#table
[[[[#x="2" 가로로 병합된 셀]] [[일반 셀]]]]
[[[[셀 1]] [[셀 2]] [[셀 3]]]]
}}}
```

### 세로 병합 (rowspan)

`#y` 매개변수를 사용합니다:

```sevenmark
{{{#table
[[[[#y="2" 세로로 병합된 셀]] [[셀 1,2]]]]
[[[[ ]] [[셀 2,2]]]]
}}}
```

## 스타일이 적용된 테이블

### 테이블 전체 스타일

```sevenmark
{{{#table #style="border: 2px solid #333;"
[[[[헤더1]] [[헤더2]]]]
[[[[셀1]] [[셀2]]]]
}}}
```

### 개별 셀 스타일

```sevenmark
{{{#table
[[[[헤더1]] [[헤더2]]]]
[[[[#color="red" 빨간 텍스트]] [[일반 셀]]]]
[[[[#bg_color="yellow" 노란 배경]] [[일반 셀]]]]
}}}
```

## 복잡한 테이블 예제

```sevenmark
{{{#table #style="width: 100%; border-collapse: collapse;"
[[[[#style="text-align: center; font-weight: bold;" 제품명]] [[가격]] [[재고]]]]
[[[[#color="blue" 노트북]] [[#style="text-align: right;" ₩1,200,000]] [[5개]]]]
[[[[#color="green" 마우스]] [[#style="text-align: right;" ₩30,000]] [[20개]]]]
[[[[#x="2" #style="text-align: center; font-weight: bold;" 총 합계]] [[#style="text-align: right; font-weight: bold;" ₩1,230,000]]]]
}}}
```

## 중첩된 마크업

테이블 셀 안에서도 다른 SevenMark 구문을 사용할 수 있습니다:

```sevenmark
{{{#table
[[[[기능]] [[설명]]]]
[[[[**볼드**]] [[*이탤릭과* 함께 사용]]]]
[[[[{{{#code inline_code() }}}]] [[코드도 가능]]]]
[[[[@media #file="image.png" 이미지]] [[미디어 요소도 가능]]]]
}}}
```

</div>