export const AIConfigHelp = {
  title: { "zh-CN": "AI 配置", en: "AI Config" },
  markdownContent: {
    "zh-CN": `
## 配置 AI 能为我带来什么？

DaisyTools 在设计之初便将全链路AI辅助作为一个重点，如果您配置了AI，将有以下能力：
<li>使用AI创建自动化工作流</li>
<li>在帮助文档中使用AI辅助理解</li>
<li>使用AI创建桌面小组件</li>
<li>更多神奇的功能！</li>

## 为什么需要AI配置？

DaisyTools 一个开源免费的工具，我们并不提供AI服务，您需要使用自己的大模型API为DaisyTools提供AI支持。

## 如何配置？

### **1. api\_key（API 密钥）**

这是一个用于身份验证的 **密钥**，用来告诉 AI供应商 “我是谁”。它可以让你访问他们的 AI 服务。

<div class="alert alert-info alert-dash">
  <strong>声明：</strong>DaisyTools 仅在本地存储您的 API 密钥，不会将您的 API 密钥用于除您正常使用外的任何其他用途。
</div>

 
---

### **2. model（模型）**

指你要用哪个 **AI 模型**，不同模型聪明程度和特点都不同。

---

### **3. temperature（温度）**

这个值控制 AI 的 **随机性**。

* **值低（接近 0）**：AI 比较 “死板”，回答更稳重。
* **值高（接近 1）**：AI 会更有创造力，但可能不那么精准。 

---

### **4. max\_tokens（最大输出字数）**

AI 最多生成的token数量，差不多是英文单词的数量，中文使用的token要更多一些。 

---

### **5. top\_p（核采样概率）**

这个和 temperature 类似，也是用来控制 AI 随机性。

* **值接近 1**：AI 会从所有可能的词里选一个，生成更多样化的内容。
* **值接近 0**：AI 只从最有可能的词里选一个，生成更稳定的内容。

---

### **6. frequency\_penalty（重复惩罚）**

这个会 **惩罚重复内容**。

* **值越高**：AI 会尽量避免重复一个词或句子。  
`,
    en: `
## What can configuring AI do for me?

DaisyTools was designed with full-chain AI assistance as a key feature. If you configure AI, you will have the following capabilities:
<li>Create automated workflows using AI</li>
<li>Use AI to assist understanding in help documentation</li>
<li>Create desktop widgets with AI</li>
<li>And more amazing features!</li>

## Why do I need AI configuration?

DaisyTools is an open-source and free tool. We do not provide AI services. You need to use your own large model API to provide AI support for DaisyTools.

## How to configure?

### **1. api\_key (API Key)**

This is a **key** used for authentication, to tell the AI provider "who I am". It allows you to access their AI services.

<div class="alert alert-info alert-dash">
  <strong>Statement:</strong> DaisyTools only stores your API key locally and will not use your API key for any purpose other than your normal use.
</div>

---

### **2. model (Model)**

This refers to which **AI model** you want to use. Different models have different intelligence levels and characteristics.

---

### **3. temperature (Temperature)**

This value controls the **randomness** of the AI.

* **Low value (close to 0):** The AI is more "rigid" and gives more stable answers.
* **High value (close to 1):** The AI is more creative, but may be less accurate.

---

### **4. max\_tokens (Maximum Output Tokens)**

The maximum number of tokens the AI can generate, which is roughly the number of English words. Chinese uses more tokens per character.

---

### **5. top\_p (Nucleus Sampling Probability)**

This is similar to temperature and is also used to control AI randomness.

* **Value close to 1:** The AI will choose from all possible words, generating more diverse content.
* **Value close to 0:** The AI will only choose from the most likely words, generating more stable content.

---

### **6. frequency\_penalty (Repetition Penalty)**

This **penalizes repeated content**.

* **The higher the value:** The AI will try to avoid repeating a word or sentence.
`,
  },
};
