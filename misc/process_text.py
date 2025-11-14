#!/usr/bin/env python3
"""
텍스트 파일 처리 스크립트
- 줄바꿈 제거
- 큰따옴표(")를 이스케이프된 따옴표(\")로 치환
"""

import sys
import os


def process_text(input_file, output_file):
    """
    텍스트 파일을 읽어서 처리한 후 새 파일로 저장

    Args:
        input_file: 입력 파일 경로
        output_file: 출력 파일 경로
    """
    try:
        # 파일 읽기
        with open(input_file, 'r', encoding='utf-8') as f:
            content = f.read()

        # 줄바꿈 제거
        content = content.replace('\n', '').replace('\r', '')

        # 큰따옴표를 이스케이프된 따옴표로 치환
        content = content.replace('"', '\\"')

        # 결과 저장
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(content)

        print(f"처리 완료!")
        print(f"입력: {input_file}")
        print(f"출력: {output_file}")
        print(f"처리된 문자 수: {len(content)}")

    except FileNotFoundError:
        print(f"오류: 파일을 찾을 수 없습니다 - {input_file}")
        sys.exit(1)
    except Exception as e:
        print(f"오류 발생: {e}")
        sys.exit(1)


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("사용법: python process_text.py <입력파일> <출력파일>")
        print("예시: python process_text.py input.txt output.txt")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    process_text(input_file, output_file)