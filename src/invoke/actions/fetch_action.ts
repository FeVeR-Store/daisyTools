/* This part is the automatically generated source code. Please modify it in actions/web/src/lib.rs */

import { defineCard } from "../helper";
const fetch_action = defineCard({
  branches: [
    {
      branch: "source",
      type: "source",
      id: "Result",
      position: "left",
    },
    {
      branch: "Success",
      type: "primary",
      id: "Result",
      position: "right",
      plug: {
        "\0type": "object",
        a: {
          "\0type": "object",
          hello: "string",
        },
      },
    },
    {
      branch: "Error",
      type: "primary",
      id: "Result",
      position: "bottom",
      plug: {
        "\0type": "object",
        timeout: null,
      },
    },
  ],
  parent: "action.web",
  name: "fetch_action",
  args: {
    url: "String",
    method: "String",
    timeout: "Int",
    proxy: "String",
    http2: "Bool",
  },
  litCardView: () => {
    return [
      {
        key: "url",
      },
      {
        key: "method",
      },
      {
        key: "timeout",
      },
      {
        key: "proxy",
      },
      {
        key: "http2",
      },
    ];
  },
  view: {
    title: "",
    form: [
      {
        name: "url",
        type: "String",
        optional: false,
        data: {},
      },
      {
        name: "method",
        type: "Option",
        optional: false,
        data: [
          {
            label: "Get",
            value: "Get",
          },
          {
            label: "Post",
            value: "Post",
          },
          {
            label: "Delete",
            value: "Delete",
          },
          {
            label: "Put",
            value: "Put",
          },
        ],
      },
      {
        name: "timeout",
        type: "Number",
        optional: false,
        data: {},
      },
      {
        name: "proxy",
        type: "String",
        optional: true,
        data: {},
      },
      {
        name: "http2",
        type: "Switch",
        optional: true,
        data: {},
      },
    ],
  },
  i18n: {
    en: {
      Method_Get: "Get",
      Success: "Success",
      description: "Send a network request",
      title: "Network request",
      url: {
        title: "Request URL",
        description: "The target address of the request",
      },
      method: {
        title: "Request Method",
        description: "Method used when sending network requests",
      },
      timeout: {
        title: "Timeout",
        description: "Request timeout",
      },
      proxy: {
        title: "Network Proxy",
        description: "Proxy server to use for the request",
      },
      http2: {
        title: "Enable http2",
        description: "Use the more efficient and secure http/2",
      },
    },
    "zh-CN": {
      Method_Get: "获取",
      Success: "成功",
      description: "发送一个网络请求",
      title: "网络请求",
      url: {
        title: "请求地址",
        description: "请求的目标地址",
      },
      method: {
        title: "请求方法",
        description: "发送网络请求时使用的方法",
      },
      timeout: {
        title: "超时时间",
        description: "请求超时时间",
      },
      proxy: {
        title: "网络代理",
        description: "请求时使用的代理服务器",
      },
      http2: {
        title: "启用http2",
        description: "采用更高效安全的http/2",
      },
    },
  },
});

/* This section can be used to extend or override */
export default fetch_action;
