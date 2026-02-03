import { LanguageModel, ModelMessage, streamText } from "ai";
import {
  AISDK,
  CreateProviderName,
  Models,
  ProviderOptions,
  Providers,
} from ".";

export interface AiConfig<P extends Providers = Providers> {
  apiKey: string;
  model: Models<P>;
  temperature: number;
  maxTokens: number | "Default";
  topP: number;
  frequencyPenalty: number;
}

interface AIOptions<P extends Providers> extends ProviderOptions, AiConfig<P> {}

export class AiInstance<P extends Providers> {
  languageModel: LanguageModel;
  model: Models<P>;
  message: ModelMessage[] = [];
  aiConfig: Partial<AiConfig>;
  constructor(sdk: AISDK<P>[CreateProviderName<P>], options: AIOptions<P>) {
    const { model, ...rest } = options;
    const { apiKey, baseURL, headers, fetch, ...aiConfig } = rest;
    this.aiConfig = aiConfig;
    this.model = model;
    const provider = sdk({ apiKey, baseURL, headers, fetch });
    this.languageModel = provider(model);
  }
  send(content: string, role: "user" | "system" | "assistant" = "user") {
    this.message.push({
      role,
      content,
    });
    const result = streamText({
      // ...this.aiConfig,
      model: this.languageModel,
      messages: this.message,
      onFinish: ({ text }) => {
        this.message.push({
          role: "assistant",
          content: text,
        });
      },
    });
    return result.textStream;
  }
  callTools() {}
}
