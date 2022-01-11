# Light Mark 极简的标记语言

---
一个`Light Mark`标记语言分析器  
本项目使用 [Pest](https://pest.rs/)  

## 特色

1. 兼容汉语输入法：
   括号可以接受 `()` 也接受 `（）` 还能接受以上混合  
   引号接受 `"` `'` `` ` `` 同样接受 `“` `”`  
   逗号接受 `,` 也接受 `，`  
   方法名同时提供英文与中文版本

## 简单语法

- `color( Color, expr )` | `颜色（Color，expr）`
  用于将`expr`上色，`expr`为任意合法表达式
  Color 有两种格式

  - `#FFFFFF` --# + 连续 6 个 16 进制数字
  - `(255,255,255)` -- R,G,B 通道，每个值 0~255

- `blob( expr )` | `加粗（ expr ）`
  用于加粗`expr`, `expr`为任意合法表达式

- `italic( expr )` | `斜体（ expr ）`
  用于使得`expr`变为斜体, `expr`为任意合法表达式

- `paragraph()` | `分段（）`
  标记新段落

- `seperate()` | `分割线（）`
  添加分割线

- `font_size( Size, expr )` | `字体大小（Size，expr）`
  用于指定`expr`字体大小，`expr`为任意合法表达式
  `Size` 为 非负浮点数

- `url( Url[, expr] )` | `链接（ Url[, expr] ）`
  用于插入一个链接，`Url`为任意合法网址  
  `expr` 为 任意合法表达式

- `image( Url[, expr] )` | `图片（ Url[, expr] ）`
  用于插入一个图片，`Url`为任意合法网址  
  `expr` 为 任意合法表达式

- `(( expr ))`
  用于将`expr`使用括号包装，包装使用的括号为内侧括号

## 使用

```rust
fn main(){
    let input = "颜色（（11，22，32），特别注意）";
    ///```json
    /// [{
    ///     "type": "Color",
    ///     "inner": {
    ///         "color": { "r": 17, "g": 34, "b": 50 },
    ///         "inner": [
    ///             {
    ///                 "type": "Plain",
    ///                 "inner": "特别注意"
    ///             }
    ///         ]
    ///     }
    ///}]
    ///```
    let result:serde_json::Value=light_mark::parse(input).unwarp();
}

```

## 示例

```txt
这是一条通知
颜色（（11，22，32），特别注意）
分段（）
分割线（）
也没有什么特别的emm 好吧 加粗（斜体（颜色（#FFFFFF, 非常重要！！）））
分段（）
url(https://pest.rs/book/grammars/syntax.html, 颜色（#ABCDEF,好欸）)
图片（https://pest.rs/book/grammars/syntax.html，“怎么样？”）
```

将会被转换为

```json
[
  {
    "type": "Plain",
    "inner": "这是一条通知        "
  },
  {
    "type": "Color",
    "inner": {
      "color": { "r": 17, "g": 34, "b": 50 },
      "inner": [
        {
          "type": "Plain",
          "inner": "特别注意"
        }
      ]
    }
  },
  {
    "type": "Paragraph"
  },
  {
    "type": "Seperate"
  },
  {
    "type": "Plain",
    "inner": "也没有什么特别的emm 好吧 "
  },
  {
    "type": "Blod",
    "inner": [
      {
        "type": "Italic",
        "inner": [
          {
            "type": "Color",
            "inner": {
              "color": { "r": 255, "g": 255, "b": 255 },
              "inner": [
                {
                  "type": "Plain",
                  "inner": "非常重要！！"
                }
              ]
            }
          }
        ]
      }
    ]
  },
  {
    "type": "Paragraph"
  },
  {
    "type": "Url",
    "inner": {
      "name": [
        {
          "type": "Color",
          "inner": {
            "color": { "r": 171, "g": 205, "b": 239 },
            "inner": [
              {
                "type": "Plain",
                "inner": "好欸"
              }
            ]
          }
        }
      ],
      "url": "https://pest.rs/book/grammars/syntax.html"
    }
  },
  {
    "type": "Plain",
    "inner": "       "
  },
  {
    "type": "Image",
    "inner": {
      "name": "怎么样？",
      "url": "https://pest.rs/book/grammars/syntax.html"
    }
  }
]
```

渲染交给前端
