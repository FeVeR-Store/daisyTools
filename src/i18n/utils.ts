import { createI18n, I18nOptions } from "vue-i18n";

// 英文序数规则
export const nth = {
  rule(
    choice: number,
    choiceLength: number,
    org: (choice: number, choiceLength: number) => number
  ) {
    if (choiceLength !== 4) return org(choice, choiceLength);
    if (choice < 3) {
      return choice;
    } else {
      return choiceLength - 1;
    }
  },
  message: { nth: "1st | 2nd | 3rd | {0}th" },
};


export function createI18nWithUtils(option: I18nOptions) {
  return createI18n({
    ...option,
    pluralRules: {
      en: nth.rule,
    },
    messages: {
      ...option.messages,
      en: {
        ...nth.message,
        ...option.messages?.en,
      },
    },
  });
}
