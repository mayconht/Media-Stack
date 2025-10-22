export type OmitBetterStrict<T, K extends keyof T> = T extends unknown
	? Pick<T, Exclude<keyof T, K>>
	: never;
