import LineWidthInput from "./LineWidthInput";
import IndentStyleSelect from "./IndentStyleSelect";
import QuoteStyleSelect from "./QuoteStyleSelect";
import SourceTypeSelect from "./SourceTypeSelect";
import { PlaygroundSettings, PlaygroundState } from "./types";
import { Dispatch, SetStateAction } from "react";

interface Props {
	settings: PlaygroundSettings,
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>,
}

export function SettingsMenu(
	{
		settings: {
			lineWidth,
			setLineWidth,
			indentWidth,
			setIndentWidth,
			indentStyle,
			setIndentStyle,
			quoteStyle,
			setQuoteStyle,
			sourceType,
			setSourceType,
			isTypeScript,
			setIsTypeScript,
			isJsx,
			setIsJsx,
		},
		setPlaygroundState,
	}: Props,
) {
	return (
		<div>
			<div className="flex flex-col sm:flex-row">
				<LineWidthInput
					lineWidth={lineWidth}
					setPlaygroundState={setPlaygroundState}
				/>
				<IndentStyleSelect
					indentWidth={indentWidth}
					indentStyle={indentStyle}
					setPlaygroundState={setPlaygroundState}
				/>
			</div>
			<div className="flex flex-col sm:flex-row">
				<QuoteStyleSelect
					quoteStyle={quoteStyle}
					setPlaygroundState={setPlaygroundState}
				/>
				<SourceTypeSelect
					isTypeScript={isTypeScript}
					setPlaygroundState={setPlaygroundState}
					isJsx={isJsx}
					sourceType={sourceType}
				/>
			</div>
		</div>
	);
}
