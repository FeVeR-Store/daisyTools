<script setup lang="ts" generic="T">
import {
  computed,
  h,
  inject,
  onMounted,
  provide,
  ref,
  useTemplateRef,
  watchEffect,
} from "vue";
import MenuView from "../views/MenuView.vue";
import { MenuItem } from "../components/Menu.vue";
import { normalizeRenderable } from "../utils/render.ts";
import { display } from "../invoke/helper.ts";
import { CardMeta, Data, LitCard, litCardViewProps } from "../invoke/type.ts";
import CardViewButton, { CardButtonType } from "./CardView.Button.vue";
import EmptyView from "./EmptyView.vue";
import Switch from "../utils/components/Switch.vue";
import {
  mdiCardsClubOutline,
  mdiCardsDiamondOutline,
  mdiUnfoldMoreVertical,
} from "@mdi/js";
import Plug from "../components/Plug.vue";

import { useI18n } from "vue-i18n";
import createScopeI18n from "../composable/useScopeI18n.ts";
import { parseData } from "../utils/type.ts";
import Stat from "../components/Stat.vue";
import { createI18nWithUtils } from "../i18n/utils.ts";
import Icon from "../components/Icon.vue";
import { useFind } from "../composable/useFind.ts";
import { normalizeStatProps } from "../components/Stat.utils.ts";

export type DraggableCardData =
  | {
      lit: true;
      data: LitCard;
      card: CardMeta;
    }
  | { lit: false; card: CardMeta };

const { t, locale } = useI18n({});

const selectedCardRef = ref<{
  name: string;
  lit: boolean;
  index: string | null;
}>({ name: "", lit: false, index: null });

const {
  cardMetas,
  fetchLitCards,
  type,
  showDescriptionView: isShowDescriptionView,
  draggableId,
} = defineProps<{
  cardMetas: CardMeta[];
  type: string;
  showDescriptionView?: boolean;
  fetchLitCards: () => Promise<LitCard[]>;
  draggableId?: symbol;
}>();

const litCardsRef = ref<LitCard[]>();
const findCardInfo = useFind(cardMetas, ["name"]);

const selectAction = inject<boolean>("selectAction", false);
const buttonType = computed<CardButtonType>(() => {
  switch (true) {
    case type === "action" && selectAction && selectedCardRef.value.lit:
      return "selectAction";
    case type === "action" && selectedCardRef.value.lit:
      return "runAction";
    case type === "trigger" && selectedCardRef.value.lit:
      return "addAction";
    default:
    case !!cardInfo.value:
      return "litCard";
  }
});
// 左侧菜单
const cardMetaMenuItems = computed(() => {
  const triggerMap: { [name: string]: MenuItem[] } = {};
  cardMetas.forEach((card) => {
    const { i18n, parent: parentName, name } = card;
    const label = i18n?.[locale.value]?.title ?? `<> ${name}`;
    const parent = display(parentName);
    const item: MenuItem = {
      label,
      onClick() {
        selectedCardRef.value = { name, lit: false, index: card.name };
      },
      payload: { lit: false, card },
      id: card.name,
    };
    triggerMap[parent]
      ? triggerMap[parent].push(item)
      : (triggerMap[parent] = [item]);
  });
  return Object.entries(triggerMap).map(([parent, children]) => ({
    label: parent,
    children,
  }));
});

// 右侧内容
const litCardInfo = ref<LitCard | null>();
provide(type, litCardInfo);
provide("jumpToPlugProvider", selectLitCardByLabel);

const cardInfo = ref<CardMeta>();

const card = useTemplateRef<{ getData: () => Data }>("card");

watchEffect(() => {
  const name = selectedCardRef.value.name;
  if (selectedCardRef.value.lit) {
    const litTrigger = (litCardsRef.value ?? []).find(
      (litTrigger) => litTrigger.id === name
    );
    litCardInfo.value = litTrigger;
    cardInfo.value = cardMetas.find(
      (trigger) => trigger.name === litTrigger?.type
    );
    return;
  }
  litCardInfo.value = null;
  cardInfo.value = cardMetas.find((trigger) => trigger.name == name);
});

const litCardMenuItems = computed(() => {
  return {
    label: t("lit.title"),
    loading: litCardsRef.value == null,
    children: litCardsRef.value?.map((card) => {
      // console.log(card)
      const cardInfo = findCardInfo("name", card.type);
      // console.log(cardInfo)
      const item: MenuItem = {
        id: card.id,
        label: card.label,
        onClick() {
          selectedCardRef.value = {
            name: card.id,
            lit: true,
            index: card.id,
          };
        },
        payload: { lit: true, data: card, card: cardInfo },
      };
      return item;
    }),
  };
});

async function reloadLitCard() {
  litCardsRef.value = await fetchLitCards();
}

onMounted(reloadLitCard);

async function handleLitCard(id: string) {
  await reloadLitCard();
  selectedCardRef.value = { name: id, lit: true, index: id };
}

async function handleRemoveCard() {
  await reloadLitCard();
  showEmptyView();
}

const selectedCardView = computed(() => {
  if (cardInfo.value) {
    const useI18n = createScopeI18n({
      locale: locale.value,
      messages: cardInfo.value.i18n,
    });
    return normalizeRenderable(cardInfo.value.view, {
      useI18n,
      cardInfo: cardInfo.value,
    });
  } else {
    return;
  }
});

const selectedLitCardView = computed(() => {
  if (cardInfo.value && litCardInfo.value) {
    const i18n = createI18nWithUtils({
      messages: cardInfo.value.i18n,
      locale: locale.value,
    });
    const stat = cardInfo.value.litCardView({
      ...i18n.global,
      litCardInfo: {
        ...litCardInfo.value,
        data: parseData(litCardInfo.value.data),
      },
      cardInfo: cardInfo.value,
    } as litCardViewProps);

    return h(Stat, {
      stat: stat.map((stat) => {
        return normalizeStatProps(
          stat,
          parseData(litCardInfo.value!.data),
          i18n.global,
          cardInfo.value!.args
        );
      }),
      onVnodeUnmounted() {
        i18n.dispose();
      },
    });
  }
  return;
});

defineExpose({
  selectLitCardByLabel,
  selectLitCardById: selectCardById,
  showEmptyView,
  currentValue: selectedCardRef,
});

//#region method
function selectLitCardByLabel(label: string) {
  const card = (litCardsRef.value ?? []).find((card) => card.label === label)!;
  selectCardById(card.id);
}

function selectCardById(id: string) {
  selectedCardRef.value = { name: id, lit: !!id.match(/\d+/), index: id };
}
function showEmptyView() {
  selectedCardRef.value = { name: "", lit: false, index: null };
}

//#endregion
</script>

<template>
  <MenuView
    v-model:active="selectedCardRef.index"
    :hide-view="!isShowDescriptionView"
    routerMethod="replace"
    :draggableId
    :items="[litCardMenuItems, ...cardMetaMenuItems]"
  >
    <Switch
      v-if="!selectedCardRef.name"
      :cases="<const>['trigger','action']"
      :value="type"
    >
      <template #trigger>
        <EmptyView
          :icon="mdiCardsClubOutline"
          :decoration="t('Trigger.decoration')"
          :title="t('Trigger.title')"
        ></EmptyView>
      </template>
      <template #action>
        <EmptyView
          :icon="mdiCardsDiamondOutline"
          :decoration="t('Action.decoration')"
          :title="t('Action.title')"
        ></EmptyView>
      </template>
    </Switch>
    <div
      v-else
      class="hero transition-all card place-items-baseline p-3 relative"
    >
      <div
        class="hero-content items-start flex-col lg:flex-row-reverse w-full h-full"
      >
        <div class="w-full h-full">
          <!-- 此处直接使用i18n，可能出现问题 -->
          <template v-if="cardInfo?.i18n?.[locale]">
            <h1 v-if="selectedCardRef.lit" class="text-5xl font-bold">
              {{ litCardInfo!.label }}
              <div
                @click="selectCardById(selectedCardRef.name)"
                class="badge badge-soft badge-accent cursor-pointer"
              >
                {{ cardInfo?.i18n[locale].title }}
              </div>
            </h1>
            <h1 v-else class="text-5xl font-bold">
              {{ cardInfo?.i18n[locale].title }}
            </h1>
            <p class="py-6">{{ cardInfo?.i18n[locale].description }}</p>
          </template>
          <template v-else>
            <h1 class="text-5xl font-bold">
              {{ cardInfo?.name }}
              <div
                @click="selectCardById(selectedCardRef.name)"
                class="badge badge-soft badge-warning"
              >
                <Icon :path="mdiUnfoldMoreVertical"></Icon>
                {{ t("developing.title") }}
                {{ t("developing.tip") }}
              </div>
            </h1>
          </template>
          <slot>
            <div
              class="w-full h-[calc-size(var(--view-height),size)] overflow-y-auto"
            >
              <template v-if="selectedCardRef.lit">
                <component
                  :key="selectedCardRef.name"
                  v-if="cardInfo?.litCardView"
                  :is="selectedLitCardView"
                ></component>
              </template>
              <div v-else>
                <component
                  :key="selectedCardRef.name"
                  v-if="cardInfo?.view"
                  ref="card"
                  :is="selectedCardView"
                >
                </component>
              </div>
            </div>
          </slot>
        </div>
      </div>
    </div>
    <CardViewButton
      v-if="cardInfo"
      @remove="handleRemoveCard"
      @lit="handleLitCard"
      :card
      :value="buttonType"
      :card-info="cardInfo"
      :lit-card-info
      :id="selectedCardRef.name"
    >
    </CardViewButton>
    <Plug
      :plug="litCardInfo?.plug"
      :card-id="selectedCardRef.name"
      v-if="selectedCardRef.lit"
    ></Plug>
  </MenuView>
</template>

<i18n lang="yaml">
zh-CN:
  Trigger:
    title: 触发器
    decoration: Trigger
  Action:
    title: 动作
    decoration: Action
  lit:
    title: 已点亮的卡片
  developing:
    title: 开发中
    tip: 未完成的卡片在构建后将不会被展示

en:
  Trigger:
    title: Trigger
    decoration: 触发器
  Action:
    title: Action
    decoration: 动作
  lit:
    title: Lit Cards
  developing:
    title: Developing
    tip: Unfinished cards will not be displayed after being built
</i18n>
