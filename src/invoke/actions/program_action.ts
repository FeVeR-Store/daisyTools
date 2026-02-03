/* This part is the automatically generated source code. Please modify it in src\service\action\program.rs */

import { defineCard } from "../helper";
const program_action = defineCard({
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
        }
    ],
    parent: "action.program",
    name: "program_action",
    args: {
        code: "Code",
        lang: "String"
    },
    litCardView: ()=>{
        return [
            {
                key: "code",
                width: 2
            },
            {
                key: "lang"
            }
        ];
    },
    view: {
        title: "",
        form: [
            {
                name: "code",
                type: "Code",
                optional: false,
                data: {}
            },
            {
                name: "lang",
                type: "Option",
                optional: false,
                data: [
                    {
                        label: "JavaScript",
                        value: "JavaScript"
                    }
                ]
            }
        ]
    },
    i18n: {
        en: {
            description: "Execute a code segment",
            title: "Execute Code",
            code: {
                title: "Code content",
                description: "Code to execute"
            },
            lang: {
                title: "Language",
                description: "Programming language used"
            }
        },
        "zh-CN": {
            description: "执行一段代码",
            title: "执行代码",
            code: {
                title: "代码内容",
                description: "要执行的代码"
            },
            lang: {
                title: "语言",
                description: "使用的编程语言"
            }
        }
    }
});

/* This section can be used to extend or override */
export default program_action;