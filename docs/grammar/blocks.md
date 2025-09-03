# 블록 요소

<div v-pre>

SevenMark는 다양한 블록 레벨 요소를 지원합니다.

## 헤더

`#` 기호를 사용하여 헤더를 만들 수 있습니다. 레벨 1부터 6까지 지원합니다.

```sevenmark
# 레벨 1 헤더
## 레벨 2 헤더
### 레벨 3 헤더
#### 레벨 4 헤더
##### 레벨 5 헤더
###### 레벨 6 헤더
```

### 접히는 헤더

헤더 뒤에 `-`를 붙이면 접힘 가능한 헤더가 됩니다:

```sevenmark
## 접히는 헤더-
이 내용은 헤더를 클릭하면 접힙니다.
```

## 수평선

세 개의 하이픈(`---`)으로 수평선을 만들 수 있습니다:

```sevenmark
---
```

## 인용문

`{{{#blockquote}}}` 구문을 사용합니다:

```sevenmark
{{{#blockquote
이것은 인용문입니다.
여러 줄에 걸쳐 쓸 수 있습니다.
}}}
```

### 스타일이 적용된 인용문

매개변수를 사용해 스타일을 지정할 수 있습니다:

```sevenmark
{{{#blockquote #color="blue"
파란색 인용문
}}}
```

## 폴드

접고 펼칠 수 있는 영역을 만듭니다:

```sevenmark
{{{#fold
요약 텍스트
---
접혀있는 상세 내용
}}}
```

### 스타일이 적용된 폴드

```sevenmark
{{{#fold #style="border: 1px solid #ccc;"
커스텀 스타일 폴드
---
내용
}}}
```

## 코드 블록

프로그래밍 코드를 표시할 때 사용합니다:

```sevenmark
{{{#code #language="rust"
fn main() {
    println!("Hello, World!");
}
}}}
```

### 언어 지정 없는 코드 블록

```sevenmark
{{{#code
plain text code
}}}
```

## TeX 수식

수학 수식을 표시할 수 있습니다:

### 인라인 수식

```sevenmark
{{{#tex E = mc^2 }}}
```

### 블록 수식

```sevenmark
{{{#tex #block=true
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
}}}
```

## 리터럴

마크업을 그대로 표시하고 싶을 때 사용합니다:

```sevenmark
{{{
**이것은 볼드로 렌더링되지 않습니다**
*이탤릭도 마찬가지입니다*
}}}
```

</div>