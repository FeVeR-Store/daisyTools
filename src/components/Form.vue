<template>
  <form @submit.prevent>
    <fieldset
      class="fieldset w-full bg-base-200 border border-base-300 p-4"
      v-bind="$attrs"
    >
      <legend class="fieldset-legend">{{ title }}</legend>
      <template v-for="{ display, name, type, data, slot, optional } in form">
        <label class="fieldset-label mt-1"
          >{{ !optional ? "*" : "" }}{{ display }}</label
        >
        <SlotAppend :model="formData" :name :render="slot">
          <template #default="bind = {}">
            <Dropable
              @drop="formData[name] = createPlug($event.data.path, formData[name])"
              :filter="(data)=> typeCheck(data.type, type)"
              dragover-class="form-drop-outline"
              :id="JsonNodeSymbol"
            >
              <template #enter>
                <div class="bg-primary opacity-40 w-full h-full flex rounded">
                  <Icon class="m-auto" :path="mdiPowerPlugOutline"></Icon>
                </div>
              </template>
              <Switch :value="type" :cases="formType">
                <template #String>
                  <Input
                    @plug-remove="formData[name] = getOldValue(formData[name])"
                    :placeholder="(data as InputData)?.placeholder"
                    v-bind="bind"
                    v-model="formData[name]"
                  />
                </template>
                <template #Number>
                  <Input
                    @plug-remove="formData[name] = getOldValue(formData[name])"
                    type="number"
                    :placeholder="(data as InputData)?.placeholder"
                    v-bind="bind"
                    v-model="formData[name]"
                  />
                </template>
                <template #Option>
                  <Select
                    @plug-remove="formData[name] = getOldValue(formData[name])"
                    :placeholder="t('option.placeholder', { display })"
                    :data="(data as OptionData)"
                    v-bind="bind"
                    v-model="formData[name]"
                  >
                  </Select>
                  <!-- <select
                  v-bind="bind"
                  v-model="formData[name]"
                  class="select w-full"
                >
                  <option value="" disabled selected>
                    {{ t("option.placeholder", { display }) }}
                  </option>
                  <option
                    v-for="{ label, value } in (data as OptionData)"
                    :value
                  >
                    {{ label }}
                  </option>
                </select> -->
                </template>
                <template #Date>
                  <!-- <ElDatePicker v-bind="bind" v-model="formData[name]" /> -->
                  <input
                    type="text"
                    class="input pika-single"
                    ref="datePicker"
                    v-model="formData[name]"
                  />
                </template>
                <template #TextArea>
                  <input
                    v-bind="bind"
                    v-model="formData[name]"
                    type="textarea"
                  />
                </template>
                <template #Switch>
                  <Toggle
                    v-model="formData[name]"
                    @plug-remove="formData[name] = getOldValue(formData[name])"
                  >
                  </Toggle>
                  <!-- <input
                      class="toggle"
                      type="checkbox"
                      v-model="formData[name]"
                    /> -->
                </template>
                <!-- <template #File>
              <ElUpload
                :http-request="
                  (file) => {
                    return handleUpload(file, name);
                  }
                "
                class="avatar-uploader"
                :show-file-list="false"
              >
                <ElButton
                  type="primary"
                  :icon="Plus"
                  class="avatar-uploader-trigger"
                >
                  上传
                </ElButton>
              </ElUpload>
            </template> -->
                <!-- <template #AutoComplete>
              <ElAutocomplete
                v-bind="bind"
                value-key="name"
                v-model="formData[name]"
                :placeholder="`请输入${display}`"
                :fetch-suggestions="(queryString, cb) => handleFetchSuggestions(queryString, cb, name as string, (data as AutoCompleteData).fetchSuggestions)"
              >
              </ElAutocomplete>
            </template> -->
                <template #Range>
                  <Range
                    @plug-remove="formData[name] = getOldValue(formData[name])"
                    :min="(data as RangeData).min"
                    :max="(data as RangeData).max"
                    :step="(data as RangeData).step"
                    :measure="3"
                    v-model="formData[name]"
                  ></Range>
                  <!-- <input
                    type="range"
                    :min="(data as RangeData).min"
                    :max="(data as RangeData).max"
                    :step="(data as RangeData).step"
                    v-model="formData[name]"
                    class="range"
                  /> -->
                </template>
                <template #Code>
                  <div>
                    <CodeEditor
                      v-model="formData[name]"
                      @plug-remove="
                        formData[name] = getOldValue(formData[name])
                      "
                    ></CodeEditor>
                  </div>
                </template>
              </Switch>
            </Dropable>
          </template>
        </SlotAppend>
      </template>
    </fieldset>
  </form>
</template>

<script
  setup
  lang="ts"
  generic="T extends { [key: string]: any },R extends { [key: string]: any } = T"
>
import Switch from "../utils/components/Switch.vue";
import {
  defineComponent,
  onMounted,
  onUnmounted,
  onUpdated,
  ref,
  Slot,
  VNode,
  watchEffect,
} from "vue";
import Pikaday from "pikaday/";
import CodeEditor from "./dataInput/CodeEditor.vue";
import { I18n, useI18n } from "vue-i18n";
import { CardComponentProps } from "../invoke/type";
import { createI18nWithUtils } from "../i18n/utils";
import Dropable from "../utils/components/Dropable.vue";
import Icon from "./Icon.vue";
import { mdiPowerPlugOutline } from "@mdi/js";
import Input from "./dataInput/Input.vue";
import Select from "./dataInput/Select.vue";
import Toggle from "./dataInput/Toggle.vue";
import Range from "./dataInput/Range.vue";
import { createPlug, getOldValue, Plug } from "./dataInput/PlugDisplay.utils";
import { processForm, processFormItem, typeCheck } from "./Form.utils";
import {
  FormItem,
  formType,
  InputData,
  OptionData,
  RangeData,
} from "./Form.type";
import { toData } from "../utils/type";

const JsonNodeSymbol = Symbol.for("jsonNode");
const { t, locale } = useI18n({});

const props = withDefaults(
  defineProps<{
    props?: CardComponentProps;
    title: string;
    form: FormItem<R>[];
    // expose?: (props: { formData: { [key: string]: any } }) => Record<string, any>;
    formData: { [key: string]: any | Plug } & T;
    formFormatter?: (form: { [key: string]: any }) => { [key: string]: any };
    //   handleSubmit: (formData: T) => void;
  }>(),
  {
    formFormatter: (form: any) => form,
  }
);

const formData = ref<{ [key: string]: any | Plug }>(props.formData);

const valid = defineModel("valid", {
  default: false,
});

watchEffect(() => {
  valid.value = props.form.reduce((passed, { name, optional }) => {
    if (!passed) return false;
    const haveValue =
      name in formData.value &&
      typeof formData.value[name] !== "undefined" &&
      formData.value[name] !== null &&
      formData.value[name] !== '';
    const requirement = optional || haveValue;
    return passed && requirement;
  }, true);
});

onUpdated(() => {
  formData.value = props.formData;
});

const currentI18n = ref<I18n<any>>();

const form = ref<FormItem<R>[]>();

watchEffect(() => {
  if (props.props) {
    // 先释放原来的i18n实例
    currentI18n.value?.dispose();
    // 取出Card的i18n
    const messages = props.props.cardInfo.i18n;
    // 创建i18n实例并储存
    const i18n = createI18nWithUtils({
      messages,
      locale: locale.value,
    });
    currentI18n.value = i18n;

    // 接下来处理form的display
    form.value = props.form.map((item) => {
      processFormItem(item, i18n);
      return item;
    });
  }
});

defineExpose({
  getData: () => {
    const form: { [key: string]: any } = {};
    props.form.forEach((item) => {
      const value = processForm(item, formData.value[item.name]);
      form[item.name] = value;
    });
    return toData(props.formFormatter(form));
  },
});

const datePicker = ref();

onMounted(() => {
  const picker = new Pikaday({
    field: datePicker.value,
  });
  picker;
});

onUnmounted(() => {
  currentI18n.value?.dispose();
});

const SlotAppend = defineComponent({
  props: ["model", "render", "name"],
  setup(
    {
      model,
      render,
      name,
    }: {
      model: any;
      render: ((res: Slot<any>, model: any, name: string) => VNode) | undefined;
      name: string;
    },
    { slots }
  ) {
    return () => {
      return [render ? render(slots.default!, model, name) : slots.default!()];
    };
  },
});
</script>

<i18n lang="yaml">
zh-CN:
  option:
    placeholder: 请选择{ display }
en:
  option:
    placeholder: Please select { display }
</i18n>

<style>
@reference "../main.css";
.form-drop-outline {
  box-shadow: 0 1px
      color-mix(in oklab, var(--input-color) calc(var(--depth) * 10%), #0000)
      inset,
    0 -1px oklch(100% 0 0 / calc(var(--depth) * 0.1)) inset;
}
</style>
