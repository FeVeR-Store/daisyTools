/* This part is the automatically generated source code. Please modify it in src\service\action\debug.rs */

import { defineCard } from "../helper";
const inject_context_action = defineCard({
    branches: [
        {
            branch: "source",
            type: "source",
            id: "Result",
            position: "left"
        },
        {
            branch: "Success",
            type: "primary",
            id: "Result",
            position: "right"
        },
        {
            branch: "ABC",
            type: "primary",
            id: "Result",
            position: "bottom",
            plug: {
                "\0type": "object",
                nest: {
                    "\0type": "object"
                }
            }
        }
    ],
    parent: "action.debug",
    name: "inject_context_action",
    args: {
        key: "String",
        value: "String"
    },
    litCardView: ()=>{
        return [
            {
                key: "key"
            },
            {
                key: "value"
            }
        ];
    },
    view: {
        title: "",
        form: [
            {
                name: "key",
                type: "String",
                optional: false,
                data: {}
            },
            {
                name: "value",
                type: "String",
                optional: false,
                data: {}
            }
        ]
    },
    i18n: {
        "zh-CN": {
            title: "注入context",
            key: {
                title: "context键",
                description: ""
            },
            value: {
                title: "context值",
                description: ""
            }
        }
    }
});

/* This section can be used to extend or override */
export default inject_context_action;