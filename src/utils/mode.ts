/**
 *  判断当前环境是否为开发模式
 */
export const inDevMode = () => process.env.mode === "development";

/**
 *  判断当前是否处于生产模式
 */
export const inProdMode = () => process.env.mode === "production";

/**
 *  判断当前是否为Android平台
 */
export const isAndroid = () => process.env.target === "android";

/**
 *  判断当前是否为Linux平台
 */
export const isLinux = () => process.env.target === "linux";

/**
 *  判断当前是否为Windows平台
 */
export const isWindows = () => process.env.target === "windows";

/**
 *  判断当前是否为Macos平台
 */
export const isMacos = () => process.env.target === "macos";
