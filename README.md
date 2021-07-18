# craftinginterpreters
porting http://www.craftinginterpreters.com/scanning.html


## 重要过程方法

| Method      | Description | current       |
| :-----------| :-----------| :----------   |
| peek        | 返回当前字符  |  `keep`       | 
| peek_next   | 返回下一个字符 |  `keep`       |
| advance     | 返回下一个字符 |  `current + 1`|
