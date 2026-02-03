export const PlugHelp = {
  title: { "zh-CN": "插头", en: "Plug" },
  markdownContent: {
    "zh-CN": `
## 什么是插头？

插头是用于将一个卡片的结果传递给另一个卡片的组件。您可以理解为卡片完成工作后，结果可以通过插头传递给下一个卡片。
如果您需要将一个卡片的结果传递给另一个卡片，您需要先为卡片定义插头。

## 我必须为每个卡片定义插头吗？

如果您并不在乎卡片的结果，比如您只是想执行一个卡片，那么并不需要定义插头。
如果您不确定，也不需要担心，在创建自动化工作流时，如果需要定义插头，我们会提示您的。

## 插头可以传递哪些类型？

为了方便理解，我们将插头分为两种类型：

<li>简单类型</li>
<li>复杂类型</li>

关于两种类型的具体内容，请参考相关帮助文档。
`,
    en: `
## What is a plug?

A plug is a component that passes the result of one card to another card. You can understand that after a card completes its work, the result can be passed to the next card through the plug.

## Do I need to define a plug for each card?

If you don't care about the result of a card, such as just want to execute a card, you don't need to define a plug.
If you are not sure, you don't need to worry, we will prompt you when you create an automation workflow if you need to define a plug.

## What types can a plug pass?

To make it easier to understand, we divide plugs into two types:

<li>Simple type</li>
<li>Complex type</li>

For the specific content of the two types, please refer to the relevant help documentation.
`,
  },
};

export const TypeHelp = {
  title: { "zh-CN": "类型", en: "Type" },
  markdownContent: {
    "zh-CN": `
## 我能使用什么类型？

为了方便理解，我们将类型分为两种：

<li>简单类型</li>
<li>复杂类型</li>

### 简单类型

**简单类型**（基本类型）指的是值本身就是一个单一的、不可再拆分的值，比如数字（\`number\`）、字符串（\`string\`）、布尔值（\`boolean\`）等。这些类型的值很简单、直接，不包含其他数据结构。

特点：
<li>直接表示数据（比如一个数字、一个文字）。</li>
<li>没有更复杂的内部结构。</li>

<p>详细的讲：</p>
<li>
数字（Number）：表示数值，比如
<code>1</code>、<code>3.14</code>、<code>-5</code>。
</li>
<li>
字符串（String）：表示一串文字，比如
<code>"Hello"</code>、<code>"123"</code>。
</li>
<li>
布尔值（Boolean）：表示逻辑真假，比如
<code>true</code>（真）和<code>false</code>（假）。
</li>

### 复杂类型 

**复杂类型**（引用类型）指的是值本身是由多个部分组成的结构，比如对象（\`object\`）、数组（\`array\`）、元组（\`tuple\`）等。复杂类型包含多个简单类型的值，它们可以互相组合形成更复杂的数据结构。

特点：
<li>可以包含多个不同或相同类型的值。</li>
<li>值之间有组织结构（比如属性、键值对）。</li>

详细的讲：
<li>
  对象（Object）：表示多个属性的集合，用键值对存储数据，比如
  <code> { name: "Tom", age: 25 }。 </code>
</li>
<li>
  数组（Array）：表示一组按顺序排列的值，比如<code>
    [1, 2, 3]。
  </code>
</li>
<li>
  元组（Tuple）：表示一组有序的值，比如<code>
    [1, "hello"]。
  </code>
</li>

### 我有点理解不了，能说人话吗？

首先我们介绍对象的概念，对象是对一个东西的抽象化描述，它包含多个属性，
每个属性都有自己的名称和值，这些属性共同描述了这个东西。

<div class="alert alert-info alert-dash">
 所谓抽象化，就是将一个具体的事物，用可以描述它的属性来描述。我们后面会举例子。
</div>

比如小明养了一条斑点狗，名字叫点点，今年5岁了，爱吃骨头。我们就可以用一个对象来描述这条狗：
\`\`\`
{
  名字: "点点",
  年龄: 5,
  爱好: "骨头",
  主人: "小明",
  品种: "斑点狗"
}
\`\`\`
以一个大括号包裹，每个属性之间用一个逗号分隔，属性的名称和值之间用一个冒号分隔的结果，就是一个对象。

\`\`\`
// 对象的结构
{
   [属性名1]: [属性值1],
   [属性名2]: [属性值2],
   ...
}
\`\`\`

一个对象的属性值，也可以是另一个对象，这样就形成了嵌套的对象。
比如我们来描述小明，他今年20岁，其他的不重要，我们就可以用一个对象来描述他：

\`\`\`
{
  姓名: "小明",
  年龄: 20,
  宠物: {
    名字: "点点",
    年龄: 5,
    爱好: "骨头",
    主人: "小明",
    品种: "斑点狗"
  }
}
\`\`\`

了解了对象的概念，我们来看数组的概念，数组是一个有序的值的集合，比如：

小明有3个朋友，分别是张三、李四、王五。我们就可以用一个数组来描述小明的3个朋友：

\`\`\`
[
  "张三",
  "李四",
  "王五"
]
\`\`\`

\`\`\`
// 数组的结构
[
  [成员1],
  [成员2],
  ...
]
\`\`\`

我们将数组中每个值，称为数组成员，数组成员的类型可以是任何类型，比如字符串、数字、对象等。
需要注意的是，数组的每个成员的类型必须一致。

\`\`\`
// 数组成员的类型
[100, 90, 32] // 数字数组
["张三", "李四", "王五"] // 字符串数组
[true, false, true] // 布尔数组
[
  {
    name: "小明",
    age: 20
  },
  {
    name: "小红",
    age: 18
  }
] // 对象数组
\`\`\`

最后我们来看元组的概念，元组是一个有序的值的集合。

你可以将元组理解为一种特殊的数组，它的长度是固定的，每个成员的类型也是固定的。
这意味着元组中的每个成员的类型可以不同。

元组更注重顺序和定长，适合小型、顺序敏感的数据，比如坐标，rgb颜色。

\`\`\`
(100, 200) // 坐标 (x, y)
(255, 0, 0) // rgb颜色 (red, green, blue)
("小明", 20, "男") // 个人信息 (姓名, 年龄, 性别)
\`\`\`

\`\`\`
// 元组的结构
(
  [成员1],
  [成员2],
  ...
)
\`\`\`
`,
    en: `
Here is the English translation of your content:

---

## What Types Can I Use?

To make things easier to understand, we divide types into two categories:

* Simple Types
* Complex Types

### Simple Types

**Simple types** (also called primitive types) are values that are single and indivisible, such as numbers (\`number\`), strings (\`string\`), and booleans (\`boolean\`). These values are straightforward and do not contain other structures.

**Characteristics:**

* Represent data directly (e.g., a number or a word).
* Do not have complex internal structures.

**In detail:**

* **Number**: Represents numeric values, such as
  \`<code>1</code>\`, \`<code>3.14</code>\`, \`<code>-5</code>\`.

* **String**: Represents a sequence of characters, such as
  \`<code>"Hello"</code>\`, \`<code>"123"</code>\`.

* **Boolean**: Represents logical true or false, such as
  \`<code>true</code>\` (true) and \`<code>false</code>\` (false).

### Complex Types

**Complex types** (also called reference types) are values made up of multiple parts or structures, such as objects (\`object\`), arrays (\`array\`), and tuples (\`tuple\`). They contain multiple simple values and can combine them into more complex structures.

**Characteristics:**

* Can contain multiple values of the same or different types.
* Have an organized structure (e.g., properties, key-value pairs).

**In detail:**

* **Object**: A collection of properties stored as key-value pairs. For example:
  \`<code>{ name: "Tom", age: 25 }</code>\`

* **Array**: An ordered list of values. For example:
  \`<code>[1, 2, 3]</code>\`

* **Tuple**: An ordered list of values with fixed length and types. For example:
  \`<code>[1, "hello"]</code>\`

### I Still Don’t Quite Get It—Can You Explain in Plain Language?

Let’s first talk about the concept of an object. An object is an abstract description of something. It contains multiple *properties*, and each property has a *name* and a *value*. These properties together describe the object.

<div class="alert alert-info alert-dash">
Abstraction means describing a specific thing using a set of properties that represent it. We’ll give examples shortly.
</div>

For instance, Tom has a Dalmatian dog named Bruce. It’s 5 years old and loves eating bones. We can use an object to describe this dog:

\`\`\`
{
  name: "Bruce",
  age: 5,
  hobby: "bones",
  owner: "Tony",
  breed: "Dalmatian"
}
\`\`\`

An object is wrapped in curly braces, properties are separated by commas, and each property's name and value are separated by a colon.

\`\`\`
// Object structure
{
   [propertyName1]: [propertyValue1],
   [propertyName2]: [propertyValue2],
   ...
}
\`\`\`

An object’s property value can also be another object—this is called a nested object.

For example, to describe Tony, who is 20 years old and owns a pet dog, we can write:

\`\`\`
{
  name: "Tony",
  age: 20,
  pet: {
    name: "Diandian",
    age: 5,
    hobby: "bones",
    owner: "Tony",
    breed: "Dalmatian"
  }
}
\`\`\`

Now that we understand objects, let’s look at arrays. An array is an ordered collection of values.

For example, if Tony has three friends: Tom, Jerry, and Spike, we can describe them with an array:

\`\`\`
[
  "Tom",
  "Jerry",
  "Spike"
]
\`\`\`

\`\`\`
// Array structure
[
  [element1],
  [element2],
  ...
]
\`\`\`

Each value in the array is called an element. The elements can be of any type—strings, numbers, objects, etc.
However, the type of all elements in an array should be the same.

\`\`\`
// Types of array elements
[100, 90, 32] // Number array
["Tom", "Jerry", "Spike"] // String array
[true, false, true] // Boolean array
[
  {
    name: "Tony",
    age: 20
  },
  {
    name: "Jerry",
    age: 18
  }
] // Object array
\`\`\`

Finally, let’s talk about tuples. A tuple is an ordered collection of values.

You can think of a tuple as a special type of array with a fixed length, and each element has a fixed type.
This means the elements can be of different types, and their order matters.

Tuples emphasize order and fixed length. They are suitable for small and order-sensitive data, such as coordinates or RGB colors.

\`\`\`
(100, 200) // Coordinates (x, y)
(255, 0, 0) // RGB color (red, green, blue)
("Tony", 20, "male") // Personal info (name, age, gender)
\`\`\`

\`\`\`
// Tuple structure
(
  [element1],
  [element2],
  ...
)
\`\`\`
`,
  },
};

export const ForAI = {
  title: { "zh-CN": "关于插头", en: "About Plug" },
  markdownContent: `
## 插头的定义
简单说，插头（Plug）是一个是动作（action）或者触发器（trigger）

  `,
};
