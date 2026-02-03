interface AIEntry {
  type: "method" | "resource";
  description: string | string[];
  value: any;
}

export function defineAiEntry(entry: { [key: string]: AIEntry }) {
  console.log(entry);
}
