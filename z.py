import json

with open("ToParse.txt", "r", encoding="utf-8") as f:
    content = f.read()

# json.dumps가 자동으로 " → \" 와 줄바꿈 → \n 처리를 해줌
escaped = json.dumps(content, ensure_ascii=False)

with open("ToParse_escaped.txt", "w", encoding="utf-8") as f:
    f.write(escaped)

print("완료! ToParse_escaped.txt에 저장됨")
print(f"결과: {escaped[:200]}..." if len(escaped) > 200 else f"결과: {escaped}")