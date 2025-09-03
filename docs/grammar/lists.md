# 리스트

<div v-pre>

SevenMark에서 리스트는 `{{{#list}}}` 구문을 사용합니다.

## 기본 리스트

### 숫자 리스트 (1, 2, 3...)

```sevenmark
{{{#list #kind="1"
첫 번째 항목
두 번째 항목
세 번째 항목
}}}
```

### 알파벳 리스트 (a, b, c...)

```sevenmark
{{{#list #kind="a"
첫 번째 항목
두 번째 항목
세 번째 항목
}}}
```

### 대문자 알파벳 리스트 (A, B, C...)

```sevenmark
{{{#list #kind="A"
첫 번째 항목
두 번째 항목
세 번째 항목
}}}
```

### 로마 숫자 리스트 (i, ii, iii...)

```sevenmark
{{{#list #kind="i"
첫 번째 항목
두 번째 항목
세 번째 항목
}}}
```

### 대문자 로마 숫자 리스트 (I, II, III...)

```sevenmark
{{{#list #kind="I"
첫 번째 항목
두 번째 항목
세 번째 항목
}}}
```

## 중첩 리스트

리스트 항목 안에 다른 리스트를 중첩할 수 있습니다:

```sevenmark
{{{#list #kind="1"
첫 번째 주요 항목
{{{#list #kind="a"
하위 항목 a
하위 항목 b
}}}
두 번째 주요 항목
{{{#list #kind="i"
하위 항목 i
하위 항목 ii
}}}
}}}
```

## 스타일이 적용된 리스트

### 전체 리스트 스타일

```sevenmark
{{{#list #kind="1" #color="blue"
파란색 리스트 항목 1
파란색 리스트 항목 2
}}}
```

### 개별 항목 스타일

```sevenmark
{{{#list #kind="1"
{{{#item #style="font-weight: bold;" 볼드 항목 }}}
{{{#item #color="red" 빨간색 항목 }}}
일반 항목
}}}
```

## 복잡한 리스트 예제

```sevenmark
{{{#list #kind="1" #style="line-height: 1.8;"
{{{#item
**주요 기능**
- 빠른 성능
- 다양한 문법 지원
}}}
{{{#item #color="green"
**장점**
{{{#list #kind="a"
사용하기 쉬움
확장성이 좋음
{{{#list #kind="i"
플러그인 시스템
매크로 지원
}}}
}}}
}}}
{{{#item
**예제 코드**
{{{#code #language="rust"
fn main() {
    println!("Hello, SevenMark!");
}
}}}
}}}
}}}
```

## 리스트와 다른 요소 조합

```sevenmark
{{{#list #kind="1"
텍스트 스타일: **볼드**, *이탤릭*, __밑줄__
코드: {{{#code println!("Hello!"); }}}
테이블:
{{{#table
| 항목 | 값 |
| ---- | -- |
| A    | 1  |
| B    | 2  |
}}}
이미지: [[#file="example.png" 예제 이미지]]
}}}
```

</div>