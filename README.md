# craftinginterpreters

porting http://www.craftinginterpreters.com/scanning.html

## 重要过程方法

| Method    | Description                  | current       |
| :-------- | :--------------------------- | :------------ |
| peek      | 返回当前字符                 | `keep`        |
| peek_next | 返回下一个字符               | `keep`        |
| advance   | 返回下一个字符               | `current + 1` |
| expect    | 判断下一个字符是否为预期字符 | `current + `  |

## 判断过程
