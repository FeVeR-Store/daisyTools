import { h } from "vue";
import Form from "../components/Form.vue";
import { createDefaultFormFromFormItems } from "../invoke/helper";

export function process(renderable: any, props: any) {
  for (const ext of extension) {
    const result = ext.process(renderable, props);
    if (result) return result;
  }
}

const extension = [
  {
    name: "Form",
    process(renderable: any, props: any) {
      if ("form" in renderable && renderable.form) {
        // ? 如果有form字段, 说明是表单信息, 使用Form组件
        return h(Form<{ [key: string]: any }, { [key: string]: any }>, {
          form: renderable.form,
          title: renderable.title,
          formData: createDefaultFormFromFormItems(renderable.form),
          formFormatter: renderable.formFormatter,
          props,
        });
      }
    },
  },
];
