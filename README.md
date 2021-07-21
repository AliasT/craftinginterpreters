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

## 语法分析

> 语法分析器从词法分析器获得一个由词法单元组成的串，并验证这个串可以由源语言的文法生成

## BNF

## parser

递归下降解析器
A recursive descent parser is a literal translation of the grammar’s rules straight into imperative code. Each rule becomes a function.

## 模式

1. 解释器模式
2. 访问者模式

```java
  interface PastryVisitor {
    void visitBeignet(Beignet beignet);
    void visitCruller(Cruller cruller);
  }

  abstract class Pastry {
    abstract void accept(PastryVisitor visitor);
  }

  class Beignet extends Pastry {
    @Override
    void accept(PastryVisitor visitor) {
      visitor.visitBeignet(this);
    }
  }

  class Cruller extends Pastry {
    @Override
    void accept(PastryVisitor visitor) {
      visitor.visitCruller(this);
    }
  }
```
