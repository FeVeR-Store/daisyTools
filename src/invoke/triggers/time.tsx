import { ref, SetupContext, watchEffect } from "vue";
import { api } from "..";
import { toast } from "../../utils/components/ToastProvider.vue";
import { defineCard } from "../helper";
import cronParser from "cron-parser";
import cronstrue from "cronstrue";
import { CardComponentProps } from "../type";

export default defineCard({
  parent: "trigger.time",
  name: "cron_trigger",
  args: { value: "String" },
  litCardView: () => {
    return [];
    //   return [{
    //     title: t()
    //   }];
  },
  i18n: {
    "zh-CN": {
      title: "Cron触发器",
      args_description: "接收一个参数expression, 表示指定的Cron表达式",
      description: "基于Cron表达式的定时触发器",
      litArgs: "Cron表达式",
      cron: {
        description: "描述",
        title: "Cron表达式",
        placeholder: "请输入Cron表达式",
        error: "当前cron表达式不合法",
        time: "第{0}次触发",
      },
    },
    en: {
      title: "Cron Trigger",
      args_description:
        "Receive a parameter expression, representing the specified Cron expression",
      description: "Cron-based timer trigger",
      litArgs: "Cron Expression",
      cron: {
        description: "Description",
        title: "Cron Expression",
        placeholder: "Please enter a Cron expression",
        error: "The current cron expression is invalid",
        time: "The @:nth trigger",
      },
    },
  },
  // @ts-ignore
  view: ({ useI18n }: CardComponentProps, { expose }: SetupContext) => {
    const { t, locale } = useI18n();
    expose({
      getData() {
        if (isVaild) {
          return {
            type: "String",
            value: cron.value,
          };
        } else {
          toast.error(t("cron.error"));
          return false;
        }
      },
    });
    const cron = ref("");
    const timeList = ref<string[]>([]);
    let isVaild = false;
    let cronDescription = ref("");
    watchEffect(async () => {
      let result = "";
      isVaild = cron.value !== "";
      isVaild = await api.isCronExpressionVaild(cron.value);

      try {
        result = cronstrue.toString(cron.value, { locale });
      } catch (e) {
        isVaild = false;
      }
      try {
        let expression = cronParser.parse(cron.value);
        const list = [];
        for (let i = 0; expression.hasNext() && i < 5; i++) {
          list.push(expression.next().toDate().toLocaleString());
        }
        timeList.value = isVaild ? list : [];
      } catch (e) {
        isVaild = false;
      }

      cronDescription.value = isVaild ? result : t("cron.error");
      return;
    });
    return () => {
      return (
        <>
          <fieldset class="fieldset w-full bg-base-200 border border-base-300 p-4 rounded-box">
            <legend class="fieldset-legend text-xl">{t("cron.title")}</legend>
            <input
              v-model={cron.value}
              type="text"
              class="input w-full input-lg"
              placeholder={t("cron.placeholder")}
            />
            <p class="fieldset-label text-[1rem]">{cronDescription.value}</p>
          </fieldset>
          <div class="card h-[calc(100vh-25rem)] overflow-auto">
            <ul class="list bg-base-100 rounded-box shadow-md">
              {timeList.value.map((time, i) => (
                <li class="list-row">
                  <div>
                    <div>{t("cron.time", [i + 1], i)}</div>
                    <div class="text-xs uppercase font-semibold opacity-60">
                      {time}
                    </div>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        </>
      );
    };
  },
});
