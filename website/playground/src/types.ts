import { RomeOutput } from "../pkg";
import { Dispatch } from "react";

export enum IndentStyle { Tab = "tab", Space = "space" }
export enum SourceType { Module = "module", Script = "script" }
export enum QuoteStyle { Double = "double", Single = "single" }

export interface PlaygroundState {
	code: string,
	lineWidth: number,
	indentStyle: IndentStyle,
	indentWidth: number,
	quoteStyle: QuoteStyle,
	sourceType: SourceType,
	isTypeScript: boolean,
	isJsx: boolean,
}

export interface PlaygroundProps {
	playgroundState: PlaygroundState,
	setPlaygroundState: Dispatch<PlaygroundState>,
	prettierOutput: string,
	romeOutput: RomeOutput,
}

export type PlaygroundSettings = Pick<
	PlaygroundState,
		| "lineWidth"
		| "setLineWidth"
		| "indentWidth"
		| "setIndentWidth"
		| "indentStyle"
		| "setIndentStyle"
		| "quoteStyle"
		| "setQuoteStyle"
		| "sourceType"
		| "setSourceType"
		| "isTypeScript"
		| "setIsTypeScript"
		| "isJsx"
		| "setIsJsx"
>;
